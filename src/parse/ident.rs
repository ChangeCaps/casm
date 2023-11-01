use std::{
    fmt::{self, Debug, Display},
    sync::Arc,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    ident: Arc<str>,
}

impl Ident {
    pub fn new(ident: impl Into<Arc<str>>) -> Self {
        Self {
            ident: ident.into(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.ident
    }
}

impl Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.ident)
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.ident)
    }
}
