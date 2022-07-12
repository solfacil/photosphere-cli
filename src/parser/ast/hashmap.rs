use crate::parser::{Node, NodeKind};

type Key = Box<dyn Node>;
type Value = Box<dyn Node>;

pub struct HashMap {
    keys: Vec<Key>,
    values: Vec<Value>,
}

impl HashMap {
    pub fn new(keys: Vec<Key>, values: Vec<Value>) -> Self {
        HashMap { keys, values }
    }
}

impl Node for HashMap {
    fn kind(&self) -> NodeKind {
        NodeKind::HashMap
    }

    fn to_string(&self) -> String {
        let elems = self
            .keys
            .iter()
            .zip(self.values.iter())
            .map(|(k, v)| match k.kind() {
                NodeKind::Atom => format!("{}: {}", k.to_string(), v.to_string()),
                NodeKind::String => format!("{} => {}", k.to_string(), v.to_string()),
                _ => unimplemented!(),
            })
            .collect::<Vec<String>>()
            .join(", ");

        format!("%{{{}}}", elems)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{AnonCall, List, Number};
    use super::*;
    use crate::parser::{Token, TokenKind};

    #[test]
    fn should_stringify_atom_maps() {
        let id_t = Token::new(TokenKind::Atom, "id".to_string());
        let payments_t = Token::new(TokenKind::Atom, "payments".to_string());
        let keys: Vec<Key> = vec![Box::new(Atom::from(id_t)), Box::new(Atom::from(payments_t))];
        let ident = Token::new(TokenKind::Identifier, "anon".to_string());
        let args = vec![Token::new(TokenKind::Number, "42".to_string())];
        let number = Token::new(TokenKind::Number, "84".to_string());
        let list_elems: Vec<Box<dyn Node>> = vec![Box::new(Number::from(number))];
        let values: Vec<Value> = vec![
            Box::new(AnonCall::new(ident, args)),
            Box::new(List::new(list_elems)),
        ];
        let hashmap = HashMap::new(keys, values);
        assert_eq!(
            hashmap.to_string(),
            "%{id: anon.(42), payments: [84]}".to_string()
        );
    }
}
