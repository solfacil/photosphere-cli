use crate::parser::{Node, NodeKind, Token, TokenKind};

pub struct List {
    elems: Vec<Box<dyn Node>>,
}

impl List {
    pub fn new(elems: Vec<Box<dyn Node>>) -> Self {
        List { elems }
    }
}

impl Node for List {
    fn kind(&self) -> NodeKind {
        NodeKind::List
    }

    fn to_string(&self) -> String {
        let elems = self
            .elems
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("[{}]", elems)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{AnonCall, Number};
    use super::*;

    #[test]
    fn basic_list() {
        let bin_token = Token::new(TokenKind::Number, "0b1010".to_string());
        let de_token = Token::new(TokenKind::Number, "42".to_string());
        let elems: Vec<Box<dyn Node>> = vec![
            Box::new(Number::from(bin_token)),
            Box::new(Number::from(de_token)),
        ];
        let list = List::new(elems);
        assert_eq!(list.to_string(), "[0b1010, 42]".to_string())
    }

    #[test]
    fn complex_list() {
        let ident = Token::new(TokenKind::Identifier, "anon".to_string());
        let args = vec![Token::new(TokenKind::Number, "42".to_string())];
        let bin_token = Token::new(TokenKind::Number, "0b1010".to_string());
        let elems: Vec<Box<dyn Node>> = vec![
            Box::new(AnonCall::new(ident, args)),
            Box::new(Number::from(bin_token)),
        ];
        let list = List::new(elems);
        assert_eq!(list.to_string(), "[anon.(42), 0b1010]".to_string())
    }
}