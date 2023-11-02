mod error;
mod ident;
mod lex;
mod source;
mod span;
mod token;

use codespan_reporting::diagnostic::Diagnostic;
pub use error::*;
pub use ident::*;
pub use lex::*;
pub use source::*;
pub use span::*;
pub use token::*;

pub trait Parse: Sized {
    fn parse(input: &mut TokenStream) -> Result<Self, ParseError>;
}

impl Parse for Ident {
    fn parse(input: &mut TokenStream) -> Result<Self, ParseError> {
        let token = input.next()?;

        match token.kind() {
            TokenKind::Ident(ident) => Ok(ident.clone()),
            _ => Err(Diagnostic::error()
                .with_message("expected an identifier")
                .with_labels(vec![token.span().to_label_primary()])),
        }
    }
}
