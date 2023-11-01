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

    fn skip_whitespace(&mut self) {
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

    pub fn lex(&mut self) -> Result<Token, Error> {
        self.skip_whitespace();

        let Some(c) = self.peek() else {
            return Err(Error::UnexpectedEof(self.span()));
        };

        if Self::is_ident_start(c) {
            let start = self.offset;
            let ident = self.lex_ident();
            let end = self.offset;

            let span = Span::new(self.source, start, end);

            return Ok(Token::new(TokenKind::Ident(ident), span));
        }

        let span = self.span().with_end(self.offset + c.len_utf8());
        Err(Error::UnexpectedChar(span, c))
    }
}
