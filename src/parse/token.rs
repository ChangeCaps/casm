use std::sync::Arc;

use codespan_reporting::diagnostic::Diagnostic;

use super::{Error, Ident, Lexer, ParseError, SourceId, Span};

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Ident(Ident),
    Comment(String),
    Integer(i64),
    Colon,
    Comma,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenStream {
    tokens: Arc<[Token]>,
    span: Span,
    index: usize,
}

impl TokenStream {
    pub fn new(tokens: impl Into<Arc<[Token]>>, span: Span) -> Self {
        Self {
            tokens: tokens.into(),
            span,
            index: 0,
        }
    }

    pub fn lex(input: &str, source: SourceId) -> Result<Self, Error> {
        let mut lexer = Lexer::new(input, source);
        let mut tokens = Vec::new();

        lexer.skip_whitespace();
        while !lexer.is_empty() {
            tokens.push(lexer.lex()?);
            lexer.skip_whitespace();
        }

        let span = Span::new(source, 0, input.len());

        Ok(Self::new(tokens, span))
    }

    pub fn is_empty(&self) -> bool {
        self.index >= self.tokens.len()
    }

    pub fn eof_span(&self) -> Span {
        self.span.with_start(self.span.end)
    }

    pub fn try_peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn try_peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.index + n)
    }

    pub fn peek(&self) -> Result<&Token, ParseError> {
        match self.try_peek() {
            Some(token) => Ok(token),
            None => Err(Diagnostic::error()
                .with_message("Unexpected end of file")
                .with_labels(vec![self.eof_span().to_label_primary()])),
        }
    }

    pub fn peek_nth(&self, n: usize) -> Result<&Token, ParseError> {
        match self.try_peek_nth(n) {
            Some(token) => Ok(token),
            None => Err(Diagnostic::error()
                .with_message("Unexpected end of file")
                .with_labels(vec![self.eof_span().to_label_primary()])),
        }
    }

    pub fn try_next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index)?;
        self.index += 1;
        Some(token)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<&Token, ParseError> {
        if self.is_empty() {
            return Err(Diagnostic::error()
                .with_message("Unexpected end of file")
                .with_labels(vec![self.eof_span().to_label_primary()]));
        }

        Ok(self.tokens.get(self.index).unwrap())
    }
}
