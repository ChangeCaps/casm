use std::ops::{BitOr, BitOrAssign};

use codespan_reporting::diagnostic::{Label, LabelStyle};

use super::SourceId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    pub source: SourceId,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub const NULL: Self = Self::null();

    pub const fn null() -> Self {
        Self {
            source: SourceId::NULL,
            start: usize::MAX,
            end: usize::MAX,
        }
    }

    pub const fn new(source: SourceId, start: usize, end: usize) -> Self {
        Self { source, start, end }
    }

    pub fn with_start(self, start: usize) -> Self {
        Self::new(self.source, start, self.end)
    }

    pub fn with_end(self, end: usize) -> Self {
        Self::new(self.source, self.start, end)
    }

    pub fn join(self, other: Self) -> Self {
        debug_assert_eq!(self.source, other.source);

        let start = self.start.min(other.start);
        let end = self.end.max(other.end);

        Self::new(self.source, start, end)
    }

    pub fn to_label(&self, style: LabelStyle) -> Label<SourceId> {
        Label::new(style, self.source, self.start..self.end)
    }

    pub fn to_label_primary(&self) -> Label<SourceId> {
        self.to_label(LabelStyle::Primary)
    }

    pub fn to_label_secondary(&self) -> Label<SourceId> {
        self.to_label(LabelStyle::Secondary)
    }
}

impl BitOr for Span {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.join(rhs)
    }
}

impl BitOrAssign for Span {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.join(rhs);
    }
}
