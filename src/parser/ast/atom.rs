use crate::parser::{Node, NodeKind, Token};

pub struct Atom {
    token: Token,
}

impl From<Token> for Atom {
    fn from(token: Token) -> Self {
        Atom { token }
    }
}

impl Node for Atom {
    fn kind(&self) -> NodeKind {
        NodeKind::Atom
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
    fn atom() {
        let token = Token::new(TokenKind::Number, ":error".to_string());
        let atom = Atom::from(token.clone());
        assert_eq!(atom.to_string(), token.lexeme());
    }

    #[test]
    fn module_atom() {
        let token = Token::new(TokenKind::Number, "Service.Template".to_string());
        let atom = Atom::from(token.clone());
        assert_eq!(atom.to_string(), token.lexeme());
    }
}
