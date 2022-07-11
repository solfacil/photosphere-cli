pub use self::token::{Token, TokenKind};
use regex::Regex;
use std::char;

mod token;

#[derive(Debug)]
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

    fn tokenize(&mut self) -> Option<Token> {
        while is_blank(&self.peek()?) {
            self.cursor += 1;
        }

        if self.peek()?.eq(&',') {
            self.cursor += 1;

            return self.tokenize();
        }

        let peek = self.peek()?;

        match &peek {
            '@' => self.read_at(),
            '#' => self.read_comment(),
            '?' => self.read_char(),
            '.' => self.read_dot(),
            '"' => self.read_string(),
            '\'' => self.read_charlist(),
            ch if ch.is_uppercase() || ch.eq(&':') => self.read_atom(),
            ch if is_delim(ch) => self.read_delim(),
            ch if is_operator(ch) => self.read_operator(),
            ch if ch.is_numeric() => self.read_number(),
            ch if is_identifier(ch) => self.read_identifier(),
            _ => None,
        }
    }

    // consumes a char and advances to next
    fn read(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.cursor += 1;

            return Some(ch);
        }

        None
    }

    // consumes a range of char while
    // `pred` returns `true`
    fn read_while<P>(&mut self, mut pred: P) -> Option<String>
    where
        P: FnMut(&char) -> bool,
    {
        let mut string = String::new();

        while let Some(next) = self.peek() {
            if !pred(&next) {
                break;
            }

            let ch = self.read()?;
            string.push(ch);
        }

        if string.is_empty() {
            return None;
        }

        Some(string)
    }

    // "look ahead" to a N single char
    fn peek_ahead(&self, cursor: usize) -> Option<char> {
        self.input.get(self.cursor + cursor).cloned()
    }

    // "look ahead" to a single next char
    fn peek(&self) -> Option<char> {
        self.input.get(self.cursor).cloned()
    }

    fn is_done(&self) -> bool {
        self.cursor >= self.input.len()
    }

    fn read_at(&mut self) -> Option<Token> {
        let at = self.read()?;

        Some(Token::new(TokenKind::At, at.to_string()))
    }

    fn read_dot(&mut self) -> Option<Token> {
        // also read `..` for ranges
        let dot = self.read_while(|c| c.eq(&'.'))?;

        Some(Token::new(TokenKind::Dot, dot))
    }

    fn read_charlist(&mut self) -> Option<Token> {
        let re = Regex::new(r"'(.+).").unwrap();

        let content = String::from_iter(self.input[self.cursor..].to_vec());

        if !re.is_match(&content) {
            return None;
        }

        let caps = re.captures(&content)?;
        let charlist = caps.get(0)?.as_str();

        self.cursor += charlist.len();

        Some(Token::new(TokenKind::Charlist, charlist.to_string()))
    }

    fn read_string(&mut self) -> Option<Token> {
        let single_re = Regex::new(r#""(.+)""#).unwrap();
        let heredoc_re = Regex::new(r#""{3}(\s|.)+"{3}"#).unwrap();

        let content = String::from_iter(self.input[self.cursor..].to_vec());

        if heredoc_re.is_match(&content) {
            let caps = heredoc_re.captures(&content)?;
            let multiline = caps.get(0)?.as_str();

            self.cursor += multiline.len();

            return Some(Token::new(TokenKind::String, multiline.to_string()));
        }

        if !single_re.is_match(&content) {
            return None;
        }

        let caps = single_re.captures(&content)?;
        let string = caps.get(0)?.as_str();

        self.cursor += string.len();

        Some(Token::new(TokenKind::String, string.to_string()))
    }

    fn read_comment(&mut self) -> Option<Token> {
        let comment = self.read_while(|ch| !is_newline(ch))?;

        Some(Token::new(TokenKind::Comment, comment))
    }

    fn read_char(&mut self) -> Option<Token> {
        let char = self.read_while(is_char)?;

        Some(Token::new(TokenKind::Char, char))
    }

    fn read_atom(&mut self) -> Option<Token> {
        let next = self.peek_ahead(1)?;
        if next.is_alphanumeric() || is_quote(&next) {
            let atom = self.read_while(is_atom)?;

            return Some(Token::new(TokenKind::Atom, atom));
        }

        // IMPROVE ME double colon is
        // a macro for defining typespecs
        self.read_operator()
    }

    fn read_number(&mut self) -> Option<Token> {
        let number = self.read_while(is_number)?;

        Some(Token::new(TokenKind::Number, number))
    }

    fn read_identifier(&mut self) -> Option<Token> {
        let id = self.read_while(is_identifier)?;

        if is_bool_literal(&id) {
            return Some(Token::new(TokenKind::Boolean, id));
        }

        Some(Token::new(TokenKind::Identifier, id))
    }

    fn read_delim(&mut self) -> Option<Token> {
        let delim = self.read()?;

        Some(Token::new(TokenKind::Delimiter, delim.to_string()))
    }

    fn read_operator(&mut self) -> Option<Token> {
        let op = self.read_while(is_operator)?;

        Some(Token::new(TokenKind::Operator, op))
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize()
    }
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
    ch.eq(&'_') || ch.eq(&'?') || ch.eq(&'!')
}

fn is_newline(ch: &char) -> bool {
    ch.eq(&'\n') || ch.eq(&'\t') || ch.eq(&'\r')
}

fn is_blank(ch: &char) -> bool {
    is_newline(ch) || ch.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

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
            assert!(lex.peek().is_none());
        }

        #[test]
        fn not_done() {
            let mut lex = Lexer::new("abc");
            lex.read();
            assert_eq!(lex.peek(), Some('b'));
        }

        #[test]
        fn id_done() {
            let mut lex = Lexer::new("abc");
            lex.read();
            lex.read();
            lex.read();
            assert!(lex.peek().is_none());
        }
    }

    #[cfg(test)]
    mod peek_ahead {
        use super::*;

        #[test]
        fn empty() {
            let lex = Lexer::new("");
            assert!(lex.peek_ahead(1).is_none());
        }

        #[test]
        fn not_done() {
            let mut lex = Lexer::new("abc213");
            lex.read();
            assert_eq!(lex.peek_ahead(1), Some('c'));
            assert_eq!(lex.peek_ahead(2), Some('2'));
            assert!(lex.peek_ahead(10).is_none());
        }

        #[test]
        fn done() {
            let mut lex = Lexer::new("a");
            lex.read();
            assert!(lex.peek_ahead(1).is_none());
        }
    }

    #[cfg(test)]
    mod read {
        use super::*;

        #[test]
        fn empty() {
            let mut lex = Lexer::new("");
            assert!(lex.read().is_none());
            assert_eq!(lex.cursor, 0);
        }

        #[test]
        fn not_done() {
            let mut lex = Lexer::new("abc");
            assert_eq!(lex.read(), Some('a'));
            assert_eq!(lex.cursor, 1);
        }

        #[test]
        fn done() {
            let mut lex = Lexer::new("abc");
            lex.read();
            lex.read();
            lex.read();
            assert!(lex.read().is_none());
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
        #[should_panic]
        fn not_done() {
            let mut lex = Lexer::new("abc");
            lex.read();
            assert!(lex.is_done())
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
        let tokens = Lexer::new(delims).into_iter();

        for token in tokens {
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
    fn should_read_module_attribute() {
        let id = "@doc";
        let mut lex = Lexer::new(id);
        // @ symbol
        let at = lex.next().unwrap();
        assert!(at.kind().is_at());
        let token = lex.next().unwrap();
        assert!(token.kind().is_identifier());
        assert_eq!(token.lexeme(), "doc".to_string());
    }

    #[test]
    fn should_read_ignored_identifier() {
        let id = "_vroom";
        let token = Lexer::new(id).next().unwrap();
        assert!(token.kind().is_identifier());
        assert_eq!(token.lexeme(), id.to_string());
    }

    #[test]
    fn should_read_operator() {
        let ops = r##"
            \- + / ^ ^^^ &&& & \\\ * ** 
            \! && <- || ||| == != =~ === 
            \!== < > <= >= |> <<< >>> <<~
            \~>> <~ ~> <~> <|> +++ --- <> 
            \++ -- => :: | //
            "##;
        let tokens = Lexer::new(ops).into_iter();

        for token in tokens {
            assert!(token.kind().is_operator());
        }
    }

    #[test]
    fn should_read_comment() {
        let comment = "# hello";
        let token = Lexer::new(comment).next().unwrap();
        assert!(token.kind().is_comment());
        assert_eq!(token.lexeme(), comment.to_string());
    }

    #[test]
    fn should_read_charlist() {
        let charlist = "'hello, world'";
        let token = Lexer::new(charlist).next().unwrap();
        assert!(token.kind().is_charlist());
        assert_eq!(token.lexeme(), charlist.to_string());
    }

    mod strings {
        use super::*;

        #[test]
        fn should_read_single_quote() {
            let string = r#""hello, world""#;
            let token = Lexer::new(string).next().unwrap();
            assert!(token.kind().is_string());
            assert_eq!(token.lexeme(), string.to_string());
        }

        #[test]
        fn should_read_template_literal() {
            let string = r##""hello, #{name}""##;
            let token = Lexer::new(string).next().unwrap();
            assert!(token.kind().is_string());
            assert_eq!(token.lexeme(), string.to_string());
        }

        #[test]
        fn should_read_char_literals() {
            let string = r#""hello, \n\n world""#;
            let token = Lexer::new(string).next().unwrap();
            assert!(token.kind().is_string());
            assert_eq!(token.lexeme(), string.to_string());
        }

        #[test]
        fn should_read_heredoc() {
            let heredoc = r#""""
              hello, world!
            """"#;
            let token = Lexer::new(heredoc).next().unwrap();
            assert!(token.kind().is_string());
            assert_eq!(token.lexeme(), heredoc.to_string());
        }
    }
}
