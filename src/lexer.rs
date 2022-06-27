use self::token::{Token, TokenKind};
use anyhow::Result;
use std::char;

pub mod token;

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
        P: FnMut(&char) -> bool,
    {
        let mut string = String::new();

        while let Some(ch) = self.read() {
            if !pred(ch) {
                // self.cursor -= 1;
                break;
            }

            string.push(*ch);
        }

        if string.is_empty() {
            return None;
        }

        Some(string)
    }

    // read N chars
    fn take(&mut self, n: usize) -> Option<String> {
        let mut buff = String::new();

        for _ in 0..n {
            let ch = *self.read()?;
            buff.push(ch);
        }

        if buff.is_empty() {
            return None;
        }

        Some(buff)
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

        // Order here matters
        match lex.peek() {
            Some(c) if c.eq(&',') => read_comma(lex, &mut tokens),
            Some(c) if c.eq(&':') => read_atom(lex, &mut tokens),
            Some(c) if is_operator(c) => read_operator(lex, &mut tokens),
            Some(c) if c.eq(&'?') => read_char(lex, &mut tokens),
            Some(c) if is_quote(c) => read_quoted(lex, &mut tokens),
            Some(c) if c.is_numeric() => read_number(lex, &mut tokens),
            Some(c) if is_delim(c) => read_delim(lex, &mut tokens),
            Some(c) if is_identifier(c) => read_identifier(lex, &mut tokens),
            Some(c) if c.is_whitespace() => read_whitespace(lex, &mut tokens),
            Some(_) => read_illegal(lex, &mut tokens),
            None => continue,
        }
    }
}

fn read_comma(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(c) = lex.read() {
        tokens.push(Token::new(TokenKind::Comma, Some(c.to_string())))
    }
}

fn read_char(lex: &mut Lexer, tokens: &mut Tokens) {
    lex.read().unwrap();
    if let Some(c) = lex.read_while(|c| c.is_alphanumeric() && !c.eq(&'?')) {
        let s = '?'.to_string() + &c;
        tokens.push(Token::new(TokenKind::Char, Some(s)))
    }
}

fn read_atom(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(a) = lex.read_while(is_identifier) {
        tokens.push(Token::new(TokenKind::Atom, Some(a)))
    }
}

fn read_quoted(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(charlist) = read_charlist(lex) {
        tokens.push(Token::new(TokenKind::Charlist, Some(charlist)))
    } else if let Some(string) = read_string(lex) {
        tokens.push(Token::new(TokenKind::String, Some(string)))
    }
}

// IMPROVE ME strings and charlists reading are basically the same
fn read_charlist(lex: &mut Lexer) -> Option<String> {
    match (lex.peek(), lex.peek_ahead(1)) {
        (Some('\''), Some('\'')) => {
            let init = lex.take(3)?;
            let charlist = lex.read_while(|c| !c.eq(&'\''))?;
            let end = lex.take(3)?;

            Some(init + &charlist + &end)
        }
        (Some('\''), _) => {
            let quote = lex.read()?.to_string();
            let charlist = lex.read_while(|c| !c.eq(&'\''))?;

            Some(quote.clone() + &charlist + &quote)
        }
        _ => None,
    }
}

fn read_string(lex: &mut Lexer) -> Option<String> {
    match (lex.peek(), lex.peek_ahead(1)) {
        (Some('"'), Some('"')) => {
            let init = lex.take(3)?;
            let string = lex.read_while(|c| !c.eq(&'"'))?;
            let end = lex.take(3)?;

            Some(init + &string + &end)
        }
        (Some('"'), _) => {
            let quote = lex.read()?.to_string();
            let string = lex.read_while(|c| !c.eq(&'"'))?;

            Some(quote.clone() + &string + &quote)
        }
        _ => None,
    }
}

fn read_whitespace(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(s) = lex.read_while(|c| c.is_whitespace()) {
        tokens.push(Token::new(TokenKind::WhiteSpace, Some(s)))
    }
}

fn read_number(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(n) = lex.read_while(is_number) {
        tokens.push(Token::new(TokenKind::Number, Some(n)))
    }
}

fn read_identifier(lex: &mut Lexer, tokens: &mut Tokens) {
    match lex.read_while(is_identifier) {
        Some(b) if is_bool(&b) => tokens.push(Token::new(TokenKind::Boolean, Some(b))),
        Some(s) => tokens.push(Token::new(TokenKind::Identifier, Some(s))),
        _ => (),
    }
}

fn read_delim(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(c) = lex.read() {
        tokens.push(Token::new(TokenKind::Delimiter, Some(c.to_string())))
    }
}

fn read_illegal(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(s) = lex.read_while(|c| !c.is_whitespace()) {
        tokens.push(Token::new(TokenKind::Illegal, Some(s)))
    }
}

fn read_operator(lex: &mut Lexer, tokens: &mut Tokens) {
    if let Some(o) = lex.read_while(is_operator) {
        tokens.push(Token::new(TokenKind::Operator, Some(o)))
    }
}

fn is_quote(c: &char) -> bool {
    c.eq(&'\'') || c.eq(&'"')
}

fn is_bool(b: &str) -> bool {
    b.eq("true") || b.eq("false") || b.eq("nil")
}

fn is_delim(c: &char) -> bool {
    c.eq(&'(')
        || c.eq(&')')
        || c.eq(&'[')
        || c.eq(&']')
        || c.eq(&'{')
        || c.eq(&'}')
        || c.eq(&'%')
        || c.eq(&'#')
}

fn is_operator(o: &char) -> bool {
    o.is_ascii_punctuation()
        && !o.eq(&'`')
        && !o.eq(&'_')
        && !o.eq(&'@')
        && !o.eq(&',')
        && !o.eq(&';')
        && !o.eq(&'#')
}

fn is_identifier(c: &char) -> bool {
    (c.is_alphanumeric() || is_extra_literal(c)) || !c.is_ascii_punctuation()
}

fn is_number(c: &char) -> bool {
    c.is_ascii_alphanumeric() || c.eq(&'.')
}

fn is_extra_literal(c: &char) -> bool {
    c.eq(&'_') || c.eq(&'@')
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
mod take {
    use super::*;

    #[test]
    fn empty() {
        let mut lex = Lexer::new("");
        assert_eq!(lex.take(1), None);
        assert_eq!(lex.cursor, 0);
    }

    #[test]
    fn not_done() {
        let mut lex = Lexer::new("abc");
        assert_eq!(lex.take(2), Some("ab".to_string()));
        assert_eq!(lex.cursor, 2);
    }

    #[test]
    fn done() {
        let mut lex = Lexer::new("abc");
        lex.read();
        lex.read();
        lex.read();
        assert_eq!(lex.take(1), None);
        assert_eq!(lex.cursor, 3);
    }
}

#[cfg(test)]
mod tokenize {
    use super::*;

    #[test]
    fn should_read_atoms() {
        let single = ":enabled?";
        let pair = ":enabled? :disabled?";

        let single_tokens = &tokenize(&mut Lexer::new(single)).unwrap();
        let pair_tokens = &tokenize(&mut Lexer::new(pair)).unwrap();

        println!("{:?}", pair_tokens);
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

    #[test]
    fn should_read_delims() {
        let delims = "%#{}()[]";
        let tokens = tokenize(&mut Lexer::new(delims)).unwrap();
        assert!(tokens[..tokens.len() - 1]
            .iter()
            .all(|t| t.kind().is_delimiter()));
    }

    #[test]
    fn should_read_char() {
        let ch = "?a ?é ?aaa";
        let tokens = tokenize(&mut Lexer::new(ch)).unwrap();
        assert!(tokens[0].kind().is_char());
        assert!(tokens[1].kind().is_char());
        assert!(tokens[2].kind().is_illegal());
    }

    #[test]
    fn should_read_bool() {
        let b = "true false nil";
        let tokens = tokenize(&mut Lexer::new(b)).unwrap();
        assert!(tokens[0].kind().is_boolean());
        assert!(tokens[1].kind().is_boolean());
        assert!(tokens[2].kind().is_boolean());
        assert!(tokens[3].kind().is_eof());
    }

    #[test]
    fn should_read_identifier() {
        let i = "hello ola12 _vrum @doc defmodule";
        let tokens = tokenize(&mut Lexer::new(i)).unwrap();
        assert!(tokens[..tokens.len() - 1]
            .iter()
            .all(|t| t.kind().is_identifier()));
    }

    #[test]
    fn should_read_simple_string() {
        let simple = r#""ola""#;
        let tokens = tokenize(&mut Lexer::new(simple)).unwrap();
        assert!(tokens.iter().any(|t| t.kind().is_string()));
    }

    #[test]
    fn should_read_complex_string() {
        let complex = r#"
            """\nola\n"""
            "#;
        let tokens = tokenize(&mut Lexer::new(complex)).unwrap();
        assert!(tokens.iter().any(|t| t.kind().is_string()));
    }

    #[test]
    fn should_read_simple_charlist() {
        let simple = r#"'ola'"#;
        let tokens = tokenize(&mut Lexer::new(simple)).unwrap();
        assert!(tokens.iter().any(|t| t.kind().is_charlist()));
    }

    #[test]
    fn should_read_complex_charlist() {
        let complex = r#"
            '''\nola\n'''
            "#;
        let tokens = tokenize(&mut Lexer::new(complex)).unwrap();
        assert!(tokens.iter().any(|t| t.kind().is_charlist()));
    }

    #[test]
    fn should_read_operator() {
        let ops = r#"
            - + / ^ ^^^ &&& & \\\ * ** !
            && <- || ||| == != =~ === !==
            < > <= >= |> <<< >>> <<~ ~>>
            <~ ~> <~> <|> +++ --- <> ++ --
            => :: | // .. .
            "#;
        let tokens = tokenize(&mut Lexer::new(ops)).unwrap();
        assert!(tokens
            .iter()
            .filter(|t| !t.kind().is_whitespace())
            .any(|t| t.kind().is_operator()));
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
