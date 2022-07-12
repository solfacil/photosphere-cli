use crate::parser::{Node, NodeKind, Token};

pub struct AnonCall {
    identifier: Token,
    args: Vec<Token>,
}

impl AnonCall {
    pub fn new(identifier: Token, args: Vec<Token>) -> Self {
        AnonCall { identifier, args }
    }
}

impl Node for AnonCall {
    fn kind(&self) -> NodeKind {
        NodeKind::AnonCall
    }

    fn to_string(&self) -> String {
        let mut init = format!("{}.(", self.identifier.lexeme());

        for arg in self.args.iter() {
            init.push_str(arg.lexeme().as_str());
        }

        init.push(')');

        init.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TokenKind;

    #[test]
    fn to_string() {
        let ident = Token::new(TokenKind::Identifier, "anon".to_string());
        let args = vec![Token::new(TokenKind::Number, "42".to_string())];
        let anon_call = AnonCall::new(ident, args);

        assert_eq!(anon_call.to_string(), "anon.(42)".to_string());
    }
}
