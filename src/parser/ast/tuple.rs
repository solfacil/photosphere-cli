use crate::parser::{Node, NodeKind};

pub struct Tuple {
    elems: Vec<Box<dyn Node>>,
}

impl Tuple {
    pub fn new(elems: Vec<Box<dyn Node>>) -> Self {
        Tuple { elems }
    }
}

impl Node for Tuple {
    fn kind(&self) -> NodeKind {
        NodeKind::Tuple
    }

    fn to_string(&self) -> String {
        let elems = self
            .elems
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        format!("{{{}}}", elems)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{AnonCall, Atom, Number};
    use super::*;
    use crate::parser::{Token, TokenKind};

    #[test]
    fn basic_tuple() {
        let ok = Token::new(TokenKind::Atom, ":ok".to_string());
        let bin_token = Token::new(TokenKind::Number, "0b1010".to_string());
        let elems: Vec<Box<dyn Node>> =
            vec![Box::new(Atom::from(ok)), Box::new(Number::from(bin_token))];
        let tuple = Tuple::new(elems);
        assert_eq!(tuple.to_string(), "{:ok, 0b1010}".to_string())
    }

    #[test]
    fn complex_list() {
        let ident = Token::new(TokenKind::Identifier, "anon".to_string());
        let args = vec![Token::new(TokenKind::Number, "42".to_string())];
        let err = Token::new(TokenKind::Atom, ":error".to_string());
        let elems: Vec<Box<dyn Node>> = vec![
            Box::new(Atom::from(err)),
            Box::new(AnonCall::new(ident, args)),
        ];
        let tuple = Tuple::new(elems);
        assert_eq!(tuple.to_string(), "{:error, anon.(42)}".to_string())
    }
}
