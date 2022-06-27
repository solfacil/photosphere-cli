use super::token::{Token, TokenKind};
use anyhow::Result;
use std::char;

type Tokens = Vec<Token>;

#[derive(Clone, Debug)]
pub struct Lexer {
    cursor: usize,
    input: Vec<char>,
}

impl Lexer {
    pub fn new(string: &str) -> Self {
        Lexer {
            cursor: usize::MIN,
            input: string.chars().collect(),
        }
    }

    // consumes a char and advances to next
    fn read(&mut self) -> Option<&char> {
        if let Some(ch) = self.input.get(self.cursor) {
            self.cursor += ch.len_utf8();

            return Some(ch);
        }

        None
    }

    fn read_while<P>(&mut self, mut pred: P) -> Option<String>
    where
        P: FnMut(char) -> bool,
    {
        let mut string = String::new();

        if let Some(fst) = self.read() {
            string.push(*fst);

            while let Some(ch) = self.read() {
                if !pred(*ch) {
                    break;
                }

                string.push(*ch);
            }
        }

        if string.is_empty() {
            return None;
        }

        Some(string)
    }

    // "look ahead" to a N single char
    fn peek_ahead(&self, cursor: usize) -> Option<&char> {
        self.input.get(self.cursor + cursor)
    }

    // "look ahead" to a single next char
    fn peek(&self) -> Option<&char> {
        self.input.get(self.cursor)
    }

    // EOF
    fn is_done(&self) -> bool {
        self.cursor == self.input.len()
    }
}

// Main entry point, given a lexer
// parses all tokens
pub fn tokenize(lex: &mut Lexer) -> Result<Tokens> {
    let mut tokens = Vec::<Token>::new();

    loop {
        if lex.cursor > 0 && lex.is_done() {
            tokens.push(Token::new(TokenKind::EOF, None));
            return Ok(tokens);
        }

        match lex.peek() {
            Some(c) if c.eq(&':') => read_atom(lex, &mut tokens),
            Some(c) if c.eq(&'"') => unimplemented!(),
            Some(c) if c.is_whitespace() => read_whitespace(lex, &mut tokens),
            Some(c) if c.is_numeric() => read_number(lex, &mut tokens),
            Some(c) if c.is_alphabetic() => unimplemented!(),
            Some(_) => continue,
            None => continue,
        }
    }
}

fn read_atom(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(a) = lex.read_while(is_identifier) {
        tokens.push(Token::new(TokenKind::Atom, Some(a)))
    }
}

fn read_whitespace(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(s) = lex.read_while(char::is_whitespace) {
        tokens.push(Token::new(TokenKind::WhiteSpace, Some(s)))
    }
}

fn read_number(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(n) = lex.read_while(is_number) {
        tokens.push(Token::new(TokenKind::Number, Some(n)))
    }
}

fn is_number(c: char) -> bool {
    c.is_ascii_alphanumeric() || c.eq(&'.')
}

fn is_identifier(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_alphanumeric()
}

#[cfg(test)]
mod cursor {
    use super::*;

    #[test]
    fn empty() {
        let lex = Lexer::new("");

        assert_eq!(lex.cursor, 0);
    }

    #[test]
    fn in_progress() {
        let mut lex = Lexer::new("abc");

        lex.read();

        assert_eq!(lex.cursor, 1);
    }

    #[test]
    fn is_done() {
        let mut lex = Lexer::new("abc");

        lex.read();
        lex.read();
        lex.read();

        assert_eq!(lex.cursor, 3);
    }
}

#[cfg(test)]
mod is_done {
    use super::*;

    #[test]
    fn emtpy() {
        let lex = Lexer::new("");

        assert!(lex.is_done())
    }

    #[test]
    fn not_done() {
        let mut lex = Lexer::new("abc");

        lex.read();

        assert_eq!(lex.is_done(), false)
    }

    #[test]
    fn done() {
        let mut lex = Lexer::new("abc");

        lex.read();
        lex.read();
        lex.read();

        assert!(lex.is_done())
    }
}

#[cfg(test)]
mod peek {
    use super::*;

    #[test]
    fn empty() {
        let lex = Lexer::new("");

        assert_eq!(lex.peek(), None)
    }

    #[test]
    fn not_done() {
        let mut lex = Lexer::new("abc");

        lex.read();

        assert_eq!(lex.peek(), Some(&'b'))
    }
}

#[cfg(test)]
mod read {
    use super::*;

    #[test]
    fn empty() {
        let mut lex = Lexer::new("");

        assert_eq!(lex.read(), None);
        assert_eq!(lex.cursor, 0)
    }

    #[test]
    fn not_done() {
        let mut lex = Lexer::new("abc");

        assert_eq!(lex.read(), Some(&'a'));
        assert_eq!(lex.cursor, 1)
    }

    #[test]
    fn done() {
        let mut lex = Lexer::new("abc");

        lex.read();
        lex.read();
        lex.read();

        assert_eq!(lex.read(), None);
        assert_eq!(lex.cursor, 3)
    }
}

#[cfg(test)]
mod lexer {
    use super::*;

    #[test]
    fn should_read_atoms() {
        let single = ":enabled?";
        let pair = ":enabled? :disabled?";

        let single_tokens = &tokenize(&mut Lexer::new(single)).unwrap();
        let pair_tokens = &tokenize(&mut Lexer::new(pair)).unwrap();

        assert_eq!(single_tokens.len(), 2);
        assert_eq!(pair_tokens.len(), 3);

        let single_token = &single_tokens[0];

        assert_eq!(single_token.kind(), TokenKind::Atom);
        assert_eq!(single_token.lexeme(), Some(single.to_string()));

        assert!(pair_tokens.iter().any(|t| t.kind().is_atom()));
    }

    #[test]
    fn should_read_int() {
        let int = "40";
        let token = &tokenize(&mut Lexer::new(int)).unwrap()[0];
        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(token.lexeme(), Some(int.to_string()));
    }

    #[test]
    fn should_read_float() {
        let float = "11.45";
        let token = &tokenize(&mut Lexer::new(float)).unwrap()[0];
        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(token.lexeme(), Some(float.to_string()));
    }

    #[test]
    fn should_read_sci_float() {
        let sci_f = "1.11e10";
        let token = &tokenize(&mut Lexer::new(sci_f)).unwrap()[0];
        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(token.lexeme(), Some(sci_f.to_string()));
    }

    #[test]
    fn should_read_bin() {
        let bin = "0b1010";
        let token = &tokenize(&mut Lexer::new(bin)).unwrap()[0];
        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(token.lexeme(), Some(bin.to_string()));
    }

    #[test]
    fn should_read_octal() {
        let oct = "0o17";
        let token = &tokenize(&mut Lexer::new(oct)).unwrap()[0];
        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(token.lexeme(), Some(oct.to_string()));
    }

    #[test]
    fn should_read_hexa() {
        let hex = "0xFFF";
        let token = &tokenize(&mut Lexer::new(hex)).unwrap()[0];
        assert_eq!(token.kind(), TokenKind::Number);
        assert_eq!(token.lexeme(), Some(hex.to_string()));
    }
}