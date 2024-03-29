#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Atom,
    Boolean, // bools are atoms although
    Char,    // codepoint ?a
    Comment,
    Comma,
    Delimiter, // (), {}, []
    Identifier,
    Newline,
    Number, // int, float, bin, oct, hex
    Operator,
    Quote,
    WhiteSpace,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String) -> Self {
        Token { kind, lexeme }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind.clone()
    }

    pub fn lexeme(&self) -> String {
        (&self.lexeme).to_string()
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

    pub fn is_comment(&self) -> bool {
        matches!(self, TokenKind::Comment)
    }

    pub fn is_comma(&self) -> bool {
        matches!(self, TokenKind::Comma)
    }

    pub fn is_delimiter(&self) -> bool {
        matches!(self, TokenKind::Delimiter)
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, TokenKind::Identifier)
    }

    pub fn is_newline(&self) -> bool {
        matches!(self, TokenKind::Newline)
    }

    pub fn is_number(&self) -> bool {
        matches!(self, TokenKind::Number)
    }

    pub fn is_operator(&self) -> bool {
        matches!(self, TokenKind::Operator)
    }

    pub fn is_quote(&self) -> bool {
        matches!(self, TokenKind::Quote)
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self, TokenKind::WhiteSpace)
    }
}
