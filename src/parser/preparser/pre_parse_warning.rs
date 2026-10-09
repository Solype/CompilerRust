use std::fmt;

use crate::lexer::span::Span;

/// Something suspicious the pre-parser accepts anyway
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreParseWarningKind {
    /// A word in 다 is a verb, but neither `(` nor `{` follows it: probably a name ending in 다
    /// (바다)
    VerbWithoutParentheses(String),
}

/// A pre-parsing warning and where it happened
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreParseWarning {
    pub kind: PreParseWarningKind,
    pub span: Span,
}

impl PreParseWarning {
    pub fn new(kind: PreParseWarningKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for PreParseWarningKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VerbWithoutParentheses(word) => write!(
                f,
                "'{word}' ends in 다, so it is a function, but no '(' or '{{' follows: \
                 a name cannot end in 다"
            ),
        }
    }
}

impl fmt::Display for PreParseWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
