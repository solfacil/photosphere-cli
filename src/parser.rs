pub use self::lexer::{Lexer, Token, TokenKind};
use anyhow::{anyhow, bail, Result};
use std::cell::RefCell;

pub mod lexer;

// ELixir only has Expressions
pub type Expression = Result<Node>;

#[derive(Debug, Eq, PartialEq)]
pub struct Node {
    tokens: Vec<Token>,
    kind: NodeKind,
}

#[derive(Debug, Eq, PartialEq)]
pub enum NodeKind {
    AnonCall,
    Attribute, // @moduletag or @any
    BinaryOp,
    Function,
    Guard,
    HashMap,
    HereDoc,
    Import,
    KeyWord,
    Macro,
    Module,
    List,
    Protocol, // `defimpl` and `defprotocol`
    Sigil,
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
            TokenKind::Delimiter => match next.kind() {
                TokenKind::Dot => self.parse_anon_call(),
                _ => bail!("Nothing to parse"),
            },
            _ => bail!("Cannot parse literal"),
        }
    }

    fn parse_anon_call(&mut self) -> Expression {
        let mut tokens = self.read_token_while(|token| !token.lexeme().eq(")"))?;
        // ")"
        tokens.push(self.read_token()?.clone());

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

impl IntoIterator for Parser {
    type Item = Expression;
    type IntoIter = <Vec<Expression> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.expressions.into_inner().into_iter()
    }
}

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

    #[test]
    #[should_panic]
    fn should_push_expression() {
        let mut p = Parser::new(setup("content"));
        let token = Token::new(TokenKind::Identifier, "content".to_string());
        let node = Node::new(vec![token], NodeKind::Variable);
        p.push_expr(Ok(node));

        assert!(p.expressions.into_inner().is_empty());
    }

    #[test]
    fn should_parse_anon_call() {
        let call = r#"anon.("jhon", 42)"#;
        let mut p = Parser::new(setup(call));

        assert!(p.peek_token().is_ok());
        let anon = p.parse_anon_call();
        p.push_expr(anon);

        assert!(p.is_done());
        let expr = p.into_iter().next().unwrap();
        assert_eq!(expr.unwrap().kind, NodeKind::AnonCall);
    }
}
