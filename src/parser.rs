pub use self::lexer::{Lexer, Token, TokenKind};
use anyhow::{anyhow, bail, Result};
use std::cell::RefCell;

pub mod lexer;

#[derive(Debug)]
struct Node {
    tokens: Vec<Token>,
    kind: NodeKind,
}

#[derive(Debug)]
enum NodeKind {
    AnonCall,
    AnonLiteral,
    BinaryOp,
    HashMap,
    List,
    String,
    UnaryOp,
    Variable,
}

impl Node {
    pub fn new(tokens: Vec<Token>, kind: NodeKind) -> Self {
        Node { tokens, kind }
    }

    pub fn to_string(_self: &Self) -> String {
        "self".to_string()
    }
}

type Expression = Result<Node>;

#[derive(Debug)]
pub struct Parser {
    cursor: usize,
    expressions: RefCell<Vec<Expression>>,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            cursor: usize::MIN,
            expressions: RefCell::new(vec![]),
            tokens,
        }
    }

    pub fn try_parse_expressions(&mut self) -> &mut Self {
        loop {
            if self.cursor > 0 && self.is_done() {
                return self;
            }

            let expr = self.parse_expression();
            self.push_expr(expr);
        }
    }

    fn parse_expression(&mut self) -> Expression {
        match self.peek_token()?.kind() {
            TokenKind::Identifier => self.parse_literal(),
            _ => bail!("Cannot parse expression"),
        }
    }

    fn parse_literal(&mut self) -> Expression {
        let next = self.peek_token()?;

        match next.kind() {
            TokenKind::Delimiter => match next.lexeme().as_str() {
                "." => self.parse_anon_call(),
                _ => unimplemented!(),
            },
            _ => bail!("Cannot parse literal"),
        }
    }

    fn parse_anon_call(&mut self) -> Expression {
        let tokens = self.read_token_while(|token| token.lexeme().eq(")"))?;

        Ok(Node::new(tokens, NodeKind::AnonCall))
    }

    fn push_expr(&mut self, expr: Expression) {
        self.expressions.borrow_mut().push(expr);
    }

    fn peek_token(&mut self) -> Result<&Token> {
        let err_msg = anyhow!("Cannot read next token");

        self.tokens[..].get(self.cursor).ok_or(err_msg)
    }

    fn read_token(&mut self) -> Result<&Token> {
        let tokens = &self.tokens;

        if let Some(token) = tokens[..].get(self.cursor) {
            self.cursor += 1;

            return Ok(token);
        }

        bail!("Cannot read current token")
    }

    fn read_token_while<P>(&mut self, mut pred: P) -> Result<Vec<Token>>
    where
        P: FnMut(&Token) -> bool,
    {
        let mut tokens = Vec::<Token>::new();

        while let Ok(token) = self.peek_token() {
            if !pred(token) {
                break;
            }

            let token = self.read_token()?;
            tokens.push(token.clone());
        }

        if tokens.is_empty() {
            bail!("Failed on parse multiple tokens, parsed: {:?}", tokens);
        }

        Ok(tokens)
    }

    fn is_done(&self) -> bool {
        self.cursor >= self.tokens.len()
    }
}

// TO BE IMPLEMENTED
// impl IntoIterator for Parser {
//     type Into = Vec<Expression>;
// }
//
// impl Iterator for Parser {
//     type Item = Expression;
//
//     fn next(&mut self) -> Option<Self::Item> {
//
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static mut TOKENS: Vec<Token> = Vec::new();

    fn setup(content: &str) -> Vec<Token> {
        unsafe {
            Once::new().call_once(|| {
                TOKENS = Lexer::new(content).collect();
            });

            TOKENS.clone()
        }
    }

    mod cursor {
        use super::*;

        #[test]
        fn empty() {
            let p = Parser::new(setup("dummy"));

            assert_eq!(p.cursor, 0);
        }

        #[test]
        fn in_progress() {
            let mut p = Parser::new(setup("1 + 2"));

            p.read_token().unwrap();

            assert_eq!(p.cursor, 1);
        }

        #[test]
        fn is_done() {
            let mut p = Parser::new(setup("1 + 2"));

            p.read_token().unwrap();
            p.read_token().unwrap();
            p.read_token().unwrap();

            assert_eq!(p.cursor, 3);
        }
    }

    mod is_done {
        use super::*;

        #[test]
        #[should_panic]
        fn empty() {
            let p = Parser::new(setup("dummy"));

            assert!(p.is_done());
        }

        #[test]
        #[should_panic]
        fn in_progress() {
            let mut p = Parser::new(setup("1 + 2"));

            p.read_token().unwrap();

            assert!(p.is_done());
        }

        #[test]
        fn is_done() {
            let mut p = Parser::new(setup("1"));

            p.read_token().unwrap();

            assert!(p.is_done());
        }
    }

    mod peek_token {
        use super::*;

        #[test]
        #[should_panic]
        fn empty() {
            let mut p = Parser::new(setup(""));

            p.peek_token().unwrap();
        }

        #[test]
        fn in_progress() {
            let mut p = Parser::new(setup("1"));

            p.peek_token().unwrap();
        }

        #[test]
        #[should_panic]
        fn is_done() {
            let mut p = Parser::new(setup("1"));

            p.read_token().unwrap();
            p.peek_token().unwrap();
        }
    }

    mod read_token {
        use super::*;

        #[test]
        #[should_panic]
        fn empty() {
            let mut p = Parser::new(setup(""));

            p.read_token().unwrap();
            assert_eq!(p.cursor, 0);
        }

        #[test]
        fn is_done() {
            let mut p = Parser::new(setup("1"));

            p.read_token().unwrap();
            assert_eq!(p.cursor, 1);
        }
    }

    mod read_token_while {
        use super::*;

        #[test]
        #[should_panic]
        fn empty() {
            let mut p = Parser::new(setup(""));

            p.read_token_while(|_t| true).unwrap();
        }

        #[test]
        fn is_done() {
            let mut p = Parser::new(setup("1 + 2"));

            p.read_token_while(|_t| true).unwrap();

            assert_eq!(p.cursor, 5);
        }
    }
}
