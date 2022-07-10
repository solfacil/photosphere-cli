pub use self::lexer::{Lexer, Token, TokenKind};
use anyhow::{anyhow, bail, Result};
use ast::AnonCall;
use std::cell::RefCell;

mod ast;
mod lexer;

pub trait Node {
    fn to_string(&self) -> String;
    fn kind(&self) -> NodeKind;
}

#[derive(Debug, Eq, PartialEq)]
pub enum NodeKind {
    AnonCall,
}

// Elixir only has expressions
type Expression = Result<Box<dyn Node>>;

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
            // TokenKind::At => self.parse_attribute(),
            TokenKind::Identifier => self.parse_identifier(),
            // TokenKind::Delimiter => self.parse_delimited(),
            _ => bail!("Cannot parse expression"),
        }
    }

    // fn parse_attribute(&mut self) {
    //     // attribute literal `@any`
    //     let mut tokens = self.read_token_while(|t| !t.kind().is_whitespace())?;
    //
    //     let mut value = self.parse_expression()?;
    //     tokens.append(&mut value);
    // }

    fn parse_identifier(&mut self) -> Expression {
        let ahead = self.peek_token_ahead(1)?;

        match ahead.kind() {
            TokenKind::Dot => self.parse_anon_call(),
            _ => todo!(),
        }
    }

    // fn parse_delimited(&mut self) -> Expression {
    //     let next = self.peek_token()?;
    //
    //     match next.lexeme().as_str() {
    //         "[" => self.parse_list(),
    //         "%" => self.parse_hashmap(),
    //         "{" => self.parse_tuple(),
    //         _ => bail!("Cannot parse delimited by {}", next.lexeme()),
    //     }
    // }

    fn parse_anon_call(&mut self) -> Expression {
        let identifier = self.read_token()?;
        self.skip(2)?; // skip `.` and `(`
        let args = self.read_token_while(|t| !t.lexeme().eq(")"))?;
        self.skip(1)?;

        Ok(Box::new(AnonCall::new(identifier, args)))
    }

    fn push_expr(&mut self, expr: Expression) {
        self.expressions.borrow_mut().push(expr);
    }

    fn peek_token(&mut self) -> Result<&Token> {
        let err_msg = anyhow!("Cannot read next token");

        self.tokens[..].get(self.cursor).ok_or(err_msg)
    }

    fn peek_token_ahead(&mut self, offset: usize) -> Result<&Token> {
        let err_msg = anyhow!("Cannot read {}nth token ahead", offset);

        self.tokens[..].get(self.cursor + offset).ok_or(err_msg)
    }

    fn read_token(&mut self) -> Result<Token> {
        let tokens = &self.tokens;

        if let Some(token) = tokens[..].get(self.cursor) {
            self.cursor += 1;

            return Ok(token.clone());
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

    fn skip(&mut self, offset: usize) -> Result<()> {
        for _ in 0..offset {
            self.read_token()?;
        }

        Ok(())
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

    mod skip {
        use super::*;

        #[test]
        fn empty() {
            let mut p = Parser::new(setup(""));
            assert!(p.skip(2).is_err());
            assert!(p.cursor < 1);
        }

        #[test]
        fn in_progress() {
            let mut p = Parser::new(setup("{1, 2}"));
            p.skip(1).unwrap();
            assert!(p.cursor == 1);
        }

        #[test]
        #[should_panic]
        fn is_done() {
            let mut p = Parser::new(setup("1"));
            p.skip(1).unwrap();
            assert!(p.cursor == 1);
            p.skip(1).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn should_push_expression() {
        let mut p = Parser::new(setup(""));
        let err = Err(anyhow!("Nothing to parse..."));
        p.push_expr(err);

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
        assert_eq!(expr.unwrap().kind(), NodeKind::AnonCall);
    }
    //
    // #[test]
    // fn should_parse_attribute() {
    //     let attr = "@moduledoc";
    //     let mut p = Parser::new(setup(attr));
    //
    //     let parsed = p.parse_attribute();
    //     p.push_expr(parsed);
    //
    //     assert!(p.is_done());
    //     let expr = p.into_iter().next().unwrap();
    //     assert_eq!(expr.unwrap().kind(), NodeKind::Attribute);
    // }
}
