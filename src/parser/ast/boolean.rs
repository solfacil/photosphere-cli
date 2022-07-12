use crate::parser::{Node, NodeKind, Token};

pub struct Boolean {
    token: Token,
}

impl From<Token> for Boolean {
    fn from(token: Token) -> Self {
        Boolean { token }
    }
}

impl Node for Boolean {
    fn kind(&self) -> NodeKind {
        NodeKind::Boolean
    }

    fn to_string(&self) -> String {
        self.token.lexeme()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TokenKind;

    #[test]
    fn to_string() {
        let token = Token::new(TokenKind::Boolean, "true".to_string());
        let bool = Boolean::from(token.clone());
        assert_eq!(bool.to_string(), token.lexeme());
    }
}
