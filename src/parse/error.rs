use codespan_reporting::diagnostic::Diagnostic;

use super::{SourceId, Span};

#[derive(Clone, Debug)]
pub enum Error {
    UnexpectedEof(Span),
    UnexpectedChar(Span, char),
}

impl Error {
    pub fn to_diagnostic(&self) -> Diagnostic<SourceId> {
        match self {
            Error::UnexpectedEof(span) => Diagnostic::error()
                .with_message("Unexpected end of file")
                .with_labels(vec![span
                    .to_label_primary()
                    .with_message("expected more input here")]),
            Error::UnexpectedChar(span, c) => Diagnostic::error()
                .with_message(format!("Unexpected character: '{}'", c))
                .with_labels(vec![span
                    .to_label_primary()
                    .with_message("this character is not expected here")]),
        }
    }
}
