use crate::parser::{Node, NodeKind, Token};

pub struct Number {
    token: Token,
}

impl From<Token> for Number {
    fn from(token: Token) -> Self {
        Number { token }
    }
}

impl Node for Number {
    fn kind(&self) -> NodeKind {
        NodeKind::Number
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
        let token = Token::new(TokenKind::Number, "0b0110".to_string());
        let number = Number::from(token.clone());
        assert_eq!(number.to_string(), token.lexeme());
    }
}
