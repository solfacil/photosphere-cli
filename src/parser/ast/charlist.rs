use crate::parser::{Node, NodeKind, Token, TokenKind};

pub struct Charlist {
    token: Token,
}

impl From<Token> for Charlist {
    fn from(token: Token) -> Self {
        Charlist { token }
    }
}

impl Node for Charlist {
    fn kind(&self) -> NodeKind {
        NodeKind::Charlist
    }

    fn to_string(&self) -> String {
        self.token.lexeme()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let token = Token::new(TokenKind::Charlist, "'hello, world'".to_string());
        let charlist = Charlist::from(token.clone());
        assert_eq!(charlist.to_string(), token.lexeme());
    }
}
