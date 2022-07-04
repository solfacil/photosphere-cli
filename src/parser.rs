use self::lexer::{Token, TokenKind};

pub mod lexer;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    errors: Vec<String>,
    current: Option<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            errors: vec![],
            current: None,
        }
    }

    pub fn parse_deps<T>(_self: &mut Self) -> Vec<T> {
        vec![]
    }
}

// impl Iterator for Parser {
//     type Item = ParserKind;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         while let Some(token) = self.lexer.next() {
//             match token.kind() {
//                 TokenKind::Atom => ,
//                 TokenKind::Boolean =>,
//                 TokenKind::Char =>,
//                 TokenKind::Comment =>,
//                 TokenKind::Comma=> ,
//                 TokenKind::Delimiter => ,
//                 TokenKind::Identifier =>,
//                 TokenKind::Newline =>,
//                 TokenKind::Number =>,
//                 TokenKind::Operator =>,
//                 TokenKind::Quote => ,
//                 TokenKind::WhiteSpace =>,
//             }
//         }
//     }
// }
