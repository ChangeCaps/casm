use crate::parse::Span;

use super::{Error, Ident, SourceId, Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    source: SourceId,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, source: SourceId) -> Self {
        Lexer {
            input,
            source,
            offset: 0,
        }
    }

    fn char_at_offset(&self, offset: usize) -> Option<char> {
        let offset = self.offset + offset;
        self.input[offset..].chars().next()
    }

    fn input(&self) -> &'a str {
        &self.input[self.offset..]
    }

    pub fn span(&self) -> Span {
        Span::new(self.source, self.offset, self.offset)
    }

    pub fn is_empty(&self) -> bool {
        self.peek().is_none()
    }

    pub fn peek(&self) -> Option<char> {
        self.char_at_offset(0)
    }

    pub fn peek_nth(&self, n: usize) -> Option<char> {
        let mut offset = self.offset;

        for _ in 0..n {
            offset += self.char_at_offset(offset)?.len_utf8();
        }

        self.char_at_offset(offset)
    }

    pub fn consume(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.offset += c.len_utf8();
        Some(c)
    }

    pub fn consume_n(&mut self, n: usize) {
        for _ in 0..n {
            self.consume();
        }
    }

    fn is_ident_start(c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_ident_continue(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    // next character must be a valid identifier start
    fn lex_ident(&mut self) -> Ident {
        let mut ident = String::new();

        loop {
            let Some(c) = self.peek() else {
                return Ident::new(ident);
            };

            if !Self::is_ident_continue(c) {
                return Ident::new(ident);
            }

            ident.push(c);
            self.consume();
        }
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            let Some(c) = self.peek() else {
                return;
            };

            if !c.is_whitespace() {
                return;
            }

            self.consume();
        }
    }

    fn lex_symbol(&mut self) -> Result<Token, Error> {
        let start = self.offset;
        let c = self.consume().unwrap();
        let end = self.offset;

        let span = Span::new(self.source, start, end);

        let kind = match c {
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            _ => return Err(Error::UnexpectedChar(span, c)),
        };

        Ok(Token::new(kind, span))
    }

    fn lex_comment(&mut self) -> Option<Token> {
        if self.peek() != Some(';') {
            return None;
        }

        let start = self.offset;
        let mut comment = String::new();

        loop {
            let c = self.consume().unwrap();
            comment.push(c);

            if c == '\n' {
                break;
            }
        }

        let end = self.offset;
        let span = Span::new(self.source, start, end);
        Some(Token::new(TokenKind::Comment(comment), span))
    }

    fn lex_integer(&mut self) -> Result<Token, Error> {
        let start = self.offset;
        let mut value = 0;
        let mut radix = 10;
        let mut sign = 1;

        if self.input().starts_with('-') {
            self.consume();
            sign = -1;
        }

        if self.input().starts_with("0x") {
            self.consume_n(2);
            radix = 16;
        } else if self.input().starts_with("0b") {
            self.consume_n(2);
            radix = 2;
        } else if self.input().starts_with("0o") {
            self.consume_n(2);
            radix = 8;
        }

        loop {
            let Some(c) = self.peek() else {
                break;
            };

            let digit = match c.to_digit(radix) {
                Some(digit) => digit,
                None => break,
            };

            self.consume();

            value *= radix as i64;
            value += digit as i64 * sign;
        }

        let end = self.offset;

        let span = Span::new(self.source, start, end);
        Ok(Token::new(TokenKind::Integer(value), span))
    }

    pub fn lex(&mut self) -> Result<Token, Error> {
        self.skip_whitespace();

        let Some(c) = self.peek() else {
            return Err(Error::UnexpectedEof(self.span()));
        };

        if let Some(token) = self.lex_comment() {
            return Ok(token);
        }

        #[allow(clippy::is_digit_ascii_radix)]
        if c.is_digit(10) || c == '-' {
            return self.lex_integer();
        }

        if Self::is_ident_start(c) {
            let start = self.offset;
            let ident = self.lex_ident();
            let end = self.offset;

            let span = Span::new(self.source, start, end);

            return Ok(Token::new(TokenKind::Ident(ident), span));
        }

        self.lex_symbol()
    }
}
