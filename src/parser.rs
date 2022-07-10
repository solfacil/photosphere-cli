pub use self::lexer::{Lexer, Token, TokenKind};
use ast::{AnonCall, Attribute, Boolean, Number};

mod ast;
mod lexer;

pub trait Node {
    fn to_string(&self) -> String;
    fn kind(&self) -> NodeKind;
}

#[derive(Debug, Eq, PartialEq)]
pub enum NodeKind {
    AnonCall,
    Attribute,
    Boolean,
    List,
    Number,
}

// IMPROVE ME use `Result` instead?
// Elixir only has expressions
type Expression = Option<Box<dyn Node>>;

#[derive(Debug)]
pub struct Parser {
    cursor: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            cursor: usize::MIN,
            tokens,
        }
    }

    fn parse_expression(&mut self) -> Expression {
        match self.peek_token()?.kind() {
            TokenKind::At => self.parse_attribute(),
            TokenKind::Identifier => self.parse_identifier(),
            // TokenKind::Delimiter => self.parse_delimited(),
            TokenKind::Number => self.parse_number(),
            TokenKind::Boolean => self.parse_boolean(),
            _ => None,
        }
    }

    fn parse_attribute(&mut self) -> Expression {
        self.cursor += 1;
        let identifier = self.read_token()?;
        let value = self.parse_expression()?;

        Some(Box::new(Attribute::new(identifier, value)))
    }

    fn parse_identifier(&mut self) -> Expression {
        let ahead = self.peek_token_ahead(1)?;

        match ahead.kind() {
            TokenKind::Dot => self.parse_anon_call(),
            _ => None,
        }
    }

    fn parse_number(&mut self) -> Expression {
        let token = self.read_token()?;

        Some(Box::new(Number::from(token)))
    }

    fn parse_boolean(&mut self) -> Expression {
        let token = self.read_token()?;

        Some(Box::new(Boolean::from(token)))
    }

    // fn parse_delimited(&mut self) -> Expression {
    //     let next = self.peek_token()?;
    //
    //     match next.lexeme().as_str() {
    //         "[" => self.parse_list(),
    //         // "%" => self.parse_hashmap(),
    //         // "{" => self.parse_tuple(),
    //         _ => None,
    //     }
    // }

    // fn parse_list(&mut self) -> Expression {
    //     self.cursor += 1;
    //
    //     while 
    // }

    fn parse_anon_call(&mut self) -> Expression {
        let identifier = self.read_token()?;
        self.cursor += 2; // skip `.` and `(`

        if self.peek_token()?.kind().is_delimiter() {
            return Some(Box::new(AnonCall::new(identifier, Vec::<Token>::new())));
        }

        let args = self.read_token_while(|t| !t.kind().is_delimiter())?;

        Some(Box::new(AnonCall::new(identifier, args)))
    }

    fn peek_token(&mut self) -> Option<Token> {
        self.tokens.get(self.cursor).cloned()
    }

    fn peek_token_ahead(&mut self, offset: usize) -> Option<Token> {
        self.tokens.get(self.cursor + offset).cloned()
    }

    fn read_token(&mut self) -> Option<Token> {
        if let Some(token) = self.peek_token() {
            self.cursor += 1;

            return Some(token);
        }

        None
    }

    fn read_token_while<P>(&mut self, mut pred: P) -> Option<Vec<Token>>
    where
        P: FnMut(&Token) -> bool,
    {
        let mut tokens = Vec::<Token>::new();

        while let Some(token) = self.peek_token() {
            if !pred(&token) {
                break;
            }

            let token = self.read_token()?;
            tokens.push(token.clone());
        }

        if tokens.is_empty() {
            return None;
        }

        Some(tokens)
    }

    fn is_done(&self) -> bool {
        self.cursor >= self.tokens.len()
    }
}

impl Iterator for Parser {
    type Item = Box<dyn Node>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_expression()
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
        fn empty() {
            let mut p = Parser::new(setup(""));
            assert!(p.peek_token().is_none());
        }

        #[test]
        fn in_progress() {
            let mut p = Parser::new(setup("1"));
            assert!(p.peek_token().is_some());
        }

        #[test]
        fn is_done() {
            let mut p = Parser::new(setup("1"));
            p.read_token().unwrap();
            assert!(p.peek_token().is_none());
        }
    }

    mod read_token {
        use super::*;

        #[test]
        fn empty() {
            let mut p = Parser::new(setup(""));
            assert!(p.read_token().is_none());
            assert_eq!(p.cursor, 0);
        }

        #[test]
        fn in_progress() {
            let mut p = Parser::new(setup("1"));
            assert!(p.read_token().is_some());
            assert_eq!(p.cursor, 1);
        }

        #[test]
        fn is_done() {
            let mut p = Parser::new(setup("1"));
            p.read_token();
            assert!(p.read_token().is_none());
            assert_eq!(p.cursor, 1);
        }
    }

    mod read_token_while {
        use super::*;

        #[test]
        fn empty() {
            let mut p = Parser::new(setup(""));
            assert!(p.read_token_while(|_t| true).is_none());
        }

        #[test]
        fn is_done() {
            let mut p = Parser::new(setup("1 + 2"));
            assert!(p.read_token_while(|_t| true).is_some());
            assert_eq!(p.cursor, 3);
        }
    }

    #[test]
    fn should_parse_anon_call() {
        let call = r#"anon.("jhon", 42)"#;
        let expr = Parser::new(setup(call)).next().unwrap();
        assert_eq!(expr.kind(), NodeKind::AnonCall);
    }

    #[test]
    fn should_parse_attribute() {
        let attr = "@moduledoc anon.()";
        let expr = Parser::new(setup(attr)).next().unwrap();
        assert_eq!(expr.kind(), NodeKind::Attribute);
    }

    #[test]
    fn should_parse_boolean() {
        let expr = Parser::new(setup("true")).next().unwrap();
        assert_eq!(expr.kind(), NodeKind::Boolean);
    }

    #[test]
    fn should_parse_number() {
        let expr = Parser::new(setup("0b0101")).next().unwrap();
        assert_eq!(expr.kind(), NodeKind::Number);
    }
}
