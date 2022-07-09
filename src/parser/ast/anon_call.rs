use crate::parser::{Node, Token};

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
    fn to_string(&self) -> String {
        let mut init = format!("{}.(", self.identifier.lexeme());

        for arg in self.args.iter() {
            init.push_str(arg.lexeme().as_str());
        }

        init.clone()
    }
}
