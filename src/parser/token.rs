#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    lexeme: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Atom,
    Boolean, // bools are atoms although
    Char,
    Charlist,
    Delimiter,
    EOF,
    Identifier,
    Illegal,
    Number, // int, float, bin, oct, hex
    Operator,
    String,
    WhiteSpace,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: Option<String>) -> Self {
        Token { kind, lexeme }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }

    pub fn lexeme(&self) -> Option<String> {
        if let Some(s) = &self.lexeme {
            return Some(s.to_string());
        }

        None
    }
}

impl TokenKind {
    pub fn is_atom(&self) -> bool {
        matches!(self, TokenKind::Atom)
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, TokenKind::Boolean)
    }

    pub fn is_char(&self) -> bool {
        matches!(self, TokenKind::Char)
    }

    pub fn is_charlist(&self) -> bool {
        matches!(self, TokenKind::Charlist)
    }

    pub fn is_delimiter(&self) -> bool {
        matches!(self, TokenKind::Delimiter)
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, TokenKind::EOF)
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, TokenKind::Identifier)
    }

    pub fn is_illegal(&self) -> bool {
        matches!(self, TokenKind::Illegal)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, TokenKind::Number)
    }

    pub fn is_operator(&self) -> bool {
        matches!(self, TokenKind::Operator)
    }

    pub fn is_string(&self) -> bool {
        matches!(self, TokenKind::String)
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self, TokenKind::WhiteSpace)
    }
}
