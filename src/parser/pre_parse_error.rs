use std::fmt;

use crate::lexer::span::Span;

/// Everything that can go wrong while splitting and classifying the words
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreParseErrorKind {
    /// A Hangul word that is no keyword, type or particle, and not split into them
    UnknownWord(String),
}

/// A pre-parsing error and where it happened
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreParseError {
    pub kind: PreParseErrorKind,
    pub span: Span,
}

impl PreParseError {
    pub fn new(kind: PreParseErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for PreParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownWord(word) => write!(f, "unknown word '{word}'"),
        }
    }
}

impl fmt::Display for PreParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for PreParseError {}
