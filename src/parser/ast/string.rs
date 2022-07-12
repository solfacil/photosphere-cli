use crate::parser::{Node, NodeKind, Token};

pub struct StringLiteral {
    token: Token,
}

impl From<Token> for StringLiteral {
    fn from(token: Token) -> Self {
        StringLiteral { token }
    }
}

impl Node for StringLiteral {
    fn kind(&self) -> NodeKind {
        NodeKind::String
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
        let token = Token::new(TokenKind::String, "hello, world".to_string());
        let string = StringLiteral::from(token.clone());
        assert_eq!(string.to_string(), token.lexeme());
    }
}
