use crate::parser::{Expression, Node, NodeKind};

pub struct List {
    elems: Vec<Expression>,
}

impl List {
    pub fn new(elems: Vec<Expression>) -> Self {
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
    use crate::parser::{Token, TokenKind};

    #[test]
    fn basic_list() {
        let bin_token = Token::new(TokenKind::Number, "0b1010".to_string());
        let de_token = Token::new(TokenKind::Number, "42".to_string());
        let elems: Vec<Expression> = vec![
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
        let elems: Vec<Expression> = vec![
            Box::new(AnonCall::new(ident, args)),
            Box::new(Number::from(bin_token)),
        ];
        let list = List::new(elems);
        assert_eq!(list.to_string(), "[anon.(42), 0b1010]".to_string())
    }
}
