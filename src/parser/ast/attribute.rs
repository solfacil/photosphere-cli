use crate::parser::{Node, NodeKind, Token, TokenKind};

pub struct Attribute {
    identifier: Token,
    value: Box<dyn Node>,
}

impl Attribute {
    pub fn new(identifier: Token, value: Box<dyn Node>) -> Self {
        Attribute { identifier, value }
    }
}

impl Node for Attribute {
    fn kind(&self) -> NodeKind {
        NodeKind::Attribute
    }

    fn to_string(&self) -> String {
        format!("@{} {}", self.identifier.lexeme(), self.value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::super::AnonCall;
    use super::*;

    #[test]
    fn to_string() {
        let ident = Token::new(TokenKind::Identifier, "base_url".to_string());
        let anon_ident = Token::new(TokenKind::Identifier, "anon".to_string());
        let value = AnonCall::new(anon_ident, Vec::<Token>::new());
        let attr = Attribute::new(ident, Box::new(value));

        assert_eq!(attr.to_string(), "@base_url anon.()".to_string());
    }
}
