use self::token::{Token, TokenKind};
use std::char;

pub mod token;

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
        P: FnMut(&char) -> bool,
    {
        let mut string = String::new();

        while let Some(ch) = self.read() {
            if !pred(ch) {
                break;
            }

            string.push(*ch);
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

    fn is_done(&self) -> bool {
        self.cursor == self.input.len()
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let peek = self.peek()?;

        match peek {
            ',' => read_comma(self),
            ':' => read_atom(self),
            '?' => read_char(self),
            ch if is_quote(ch) => read_quote(self),
            ch if is_delim(ch) => read_delim(self),
            ch if is_operator(ch) => read_operator(self),
            ch if ch.is_numeric() => read_number(self),
            ch if is_identifier(ch) => read_identifier(self),
            ch if ch.is_whitespace() => read_whitespace(self),
            _ => None,
        }
    }
}

fn read_comma(lex: &mut Lexer) -> Option<Token> {
    let comma = lex.read()?.to_string();

    Some(Token::new(TokenKind::Comma, comma))
}

fn read_char(lex: &mut Lexer) -> Option<Token> {
    let char = lex.read_while(is_char)?;

    Some(Token::new(TokenKind::Char, char))
}

fn read_atom(lex: &mut Lexer) -> Option<Token> {
    let next = lex.peek_ahead(1)?;
    if next.is_alphanumeric() || is_quote(next) {
        let atom = lex.read_while(is_atom)?;

        return Some(Token::new(TokenKind::Atom, atom));
    }

    // IMPROVE ME double colon is
    // a macro for defininf typespecs
    read_operator(lex)
}

fn read_quote(lex: &mut Lexer) -> Option<Token> {
    let quote = lex.read()?.to_string();

    Some(Token::new(TokenKind::Quote, quote))
}

fn read_whitespace(lex: &mut Lexer) -> Option<Token> {
    let ws = lex.read_while(&|c: &char| c.is_whitespace())?;

    Some(Token::new(TokenKind::WhiteSpace, ws))
}

fn read_number(lex: &mut Lexer) -> Option<Token> {
    let number = lex.read_while(is_number)?;

    Some(Token::new(TokenKind::Number, number))
}

fn read_identifier(lex: &mut Lexer) -> Option<Token> {
    let id = lex.read_while(is_identifier)?;

    if is_bool_literal(&id) {
        return Some(Token::new(TokenKind::Boolean, id));
    }

    Some(Token::new(TokenKind::Identifier, id))
}

fn read_delim(lex: &mut Lexer) -> Option<Token> {
    let delim = lex.read()?;

    Some(Token::new(TokenKind::Delimiter, delim.to_string()))
}

fn read_operator(lex: &mut Lexer) -> Option<Token> {
    let op = lex.read_while(is_operator)?;

    Some(Token::new(TokenKind::Operator, op))
}

fn is_atom(ch: &char) -> bool {
    ch.eq(&':') || ch.eq(&'"') || is_extra_literal(ch) || ch.is_alphanumeric()
}

fn is_bool_literal(b: &str) -> bool {
    b.eq("true") || b.eq("false") || b.eq("nil")
}

fn is_char(ch: &char) -> bool {
    ch.eq(&'?') || ch.is_alphanumeric()
}

fn is_delim(ch: &char) -> bool {
    ch.eq(&'(')
        || ch.eq(&')')
        || ch.eq(&'[')
        || ch.eq(&']')
        || ch.eq(&'{')
        || ch.eq(&'}')
        || ch.eq(&'%')
}

fn is_quote(ch: &char) -> bool {
    ch.eq(&'\'') || ch.eq(&'"')
}

fn is_operator(ch: &char) -> bool {
    ch.is_ascii_punctuation()
        && !ch.eq(&'`')
        && !ch.eq(&'_')
        && !ch.eq(&'@')
        && !ch.eq(&',')
        && !ch.eq(&';')
        && !ch.eq(&'#')
        || ch.eq(&':')
}

fn is_identifier(ch: &char) -> bool {
    ((ch.is_alphanumeric() || is_extra_literal(ch)) || !ch.is_ascii_punctuation())
        && !ch.is_whitespace()
}

fn is_number(ch: &char) -> bool {
    ch.is_ascii_alphanumeric() || ch.eq(&'.')
}

fn is_extra_literal(ch: &char) -> bool {
    ch.eq(&'_')
        || ch.eq(&'@')
        || ch.eq(&'?')
        || ch.eq(&'!')
        || ch.eq(&'{')
        || ch.eq(&'%')
        || ch.eq(&'}')
        || ch.eq(&'.')
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

    #[test]
    fn id_done() {
        let mut lex = Lexer::new("abc");
        lex.read();
        lex.read();
        lex.read();

        assert_eq!(lex.peek(), None)
    }
}

#[cfg(test)]
mod peek_ahead {
    use super::*;

    #[test]
    fn empty() {
        let lex = Lexer::new("");
        assert_eq!(lex.peek_ahead(1), None);
    }

    #[test]
    fn not_done() {
        let mut lex = Lexer::new("abc213");
        lex.read();
        assert_eq!(lex.peek_ahead(1), Some(&'c'));
        assert_eq!(lex.peek_ahead(2), Some(&'2'));
        assert_eq!(lex.peek_ahead(10), None);
    }

    #[test]
    fn done() {
        let mut lex = Lexer::new("a");
        lex.read();
        assert_eq!(lex.peek_ahead(1), None);
    }
}

#[cfg(test)]
mod read {
    use super::*;

    #[test]
    fn empty() {
        let mut lex = Lexer::new("");

        assert_eq!(lex.read(), None);
        assert_eq!(lex.cursor, 0);
    }

    #[test]
    fn not_done() {
        let mut lex = Lexer::new("abc");

        assert_eq!(lex.read(), Some(&'a'));
        assert_eq!(lex.cursor, 1);
    }

    #[test]
    fn done() {
        let mut lex = Lexer::new("abc");

        lex.read();
        lex.read();
        lex.read();

        assert_eq!(lex.read(), None);
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
mod lexer {
    use super::*;

    #[test]
    fn should_read_atom() {
        let atom = ":enabled?";
        let token = Lexer::new(atom).next().unwrap();
        assert!(token.kind().is_atom());
        assert_eq!(token.lexeme(), atom.to_string());
    }

    #[test]
    fn should_read_quoted_atom() {
        let atom = r#":"enabled?""#;
        let token = Lexer::new(atom).next().unwrap();
        assert!(token.kind().is_atom());
        assert_eq!(token.lexeme(), atom.to_string());
    }

    #[test]
    fn should_read_int() {
        let int = "40";
        let token = Lexer::new(int).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.lexeme(), int.to_string());
    }

    #[test]
    fn should_read_float() {
        let float = "11.45";
        let token = Lexer::new(float).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.lexeme(), float.to_string());
    }

    #[test]
    fn should_read_sci_float() {
        let sci_f = "1.11e10";
        let token = Lexer::new(sci_f).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.lexeme(), sci_f.to_string());
    }

    #[test]
    fn should_read_bin() {
        let bin = "0b1010";
        let token = Lexer::new(bin).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.lexeme(), bin.to_string());
    }

    #[test]
    fn should_read_octal() {
        let oct = "0o17";
        let token = Lexer::new(oct).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.lexeme(), oct.to_string());
    }

    #[test]
    fn should_read_hexa() {
        let hex = "0xFFF";
        let token = Lexer::new(hex).next().unwrap();
        assert!(token.kind().is_number());
        assert_eq!(token.lexeme(), hex.to_string());
    }

    #[test]
    fn should_read_delims() {
        let delims = "{}()[]";
        let mut lex = Lexer::new(delims);

        while !lex.is_done() {
            let token = lex.next().unwrap();
            assert!(token.kind().is_delimiter());
        }
    }

    #[test]
    fn should_read_char() {
        let ch = "?é";
        let token = Lexer::new(ch).next().unwrap();
        assert!(token.kind().is_char());
        assert_eq!(token.lexeme(), ch.to_string());
    }

    #[test]
    fn should_read_bool() {
        let bools = "true false nil";
        let mut lex = Lexer::new(bools);

        let t = lex.next().unwrap();
        assert!(t.kind().is_boolean());
        assert_eq!(t.lexeme(), "true".to_string());

        let f = lex.next().unwrap();
        assert!(f.kind().is_boolean());
        assert_eq!(f.lexeme(), "false".to_string());

        let n = lex.next().unwrap();
        assert!(n.kind().is_boolean());
        assert_eq!(n.lexeme(), "nil".to_string());
    }

    #[test]
    fn should_read_identifier() {
        let id = "defmodule";
        let token = Lexer::new(id).next().unwrap();
        assert!(token.kind().is_identifier());
        assert_eq!(token.lexeme(), id.to_string());
    }

    #[test]
    fn should_read_module_identifier() {
        let id = "@doc";
        let token = Lexer::new(id).next().unwrap();
        assert!(token.kind().is_identifier());
        assert_eq!(token.lexeme(), id.to_string());
    }

    #[test]
    fn should_read_ignored_identifier() {
        let id = "_vroom";
        let token = Lexer::new(id).next().unwrap();
        assert!(token.kind().is_identifier());
        assert_eq!(token.lexeme(), id.to_string());
    }

    #[test]
    fn should_read_quotes() {
        let quotes = "'\"";
        let mut lex = Lexer::new(quotes);

        let single = lex.next().unwrap();
        assert!(single.kind().is_quote());
        assert_eq!(single.lexeme(), "'");

        let double = lex.next().unwrap();
        assert!(double.kind().is_quote());
        assert_eq!(double.lexeme(), "\"");
    }

    #[test]
    fn should_read_operator() {
        let ops = r##"
            \- + / ^ ^^^ &&& & \\\ * ** 
            \! && <- || ||| == != =~ === 
            \!== < > <= >= |> <<< >>> <<~
            \~>> <~ ~> <~> <|> +++ --- <> 
            \++ -- => :: | // .. .
            "##;
        let mut lex = Lexer::new(ops);

        while !lex.is_done() {
            let token = lex.next().unwrap();

            if !token.kind().is_whitespace() {
                assert!(token.kind().is_operator());
            }
        }
    }

    // use std::path::Path;

    // for manual token checking
    // #[test]
    // fn read_mix_exs() {
    //     let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    //     let p = root.join("priv").join("mix.exs");
    //     let c = std::fs::read_to_string(p).unwrap();
    //     let tokens = tokenize(&mut Lexer::new(&c)).unwrap();
    //     std::fs::write(root.join("tokens.txt"), format!("{:?}", tokens)).unwrap();
    //
    //     assert!(true);
    // }
}
