use std::fmt;

use super::span::Span;

/// Everything that can go wrong while cutting the source into tokens
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexErrorKind {
    /// A character that starts no token (`#`, `@`, `;`, ...)
    UnexpectedChar(char),
    /// `/*` without its `*/`
    UnterminatedComment,
    /// `"` without its closing `"` before the end of the line
    UnterminatedString,
    /// `\` followed by something else than `n`, `t`, `0`, `\` or `"`
    InvalidEscape(char),
    /// An integer literal above `i64::MAX`
    IntegerOverflow,
    /// `=` alone: there is no assignment operator, it is written 넣어요
    LoneEquals,
    /// `!` alone: there is no negation operator, it is written 아니면
    LoneBang,
    /// A lone jamo (`ㄱ`, `ㅏ`, ...): an unfinished syllable, often an IME typo
    LooseJamo(char),
}

/// A lexing error and where it happened
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

impl LexError {
    pub fn new(kind: LexErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for LexErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedChar(c) => write!(f, "unexpected character '{c}'"),
            Self::UnterminatedComment => write!(f, "unterminated comment: '/*' without '*/'"),
            Self::UnterminatedString => write!(
                f,
                "unterminated string: missing '\"' before the end of the line"
            ),
            Self::InvalidEscape(c) => {
                write!(
                    f,
                    "invalid escape '\\{c}' (expected \\n, \\t, \\0, \\\\ or \\\")"
                )
            }
            Self::IntegerOverflow => write!(f, "integer too large (maximum {})", i64::MAX),
            Self::LoneEquals => write!(
                f,
                "'=' alone does not exist: assignment is written 넣어요, equality =="
            ),
            Self::LoneBang => write!(
                f,
                "'!' alone does not exist: negation is written 아니면, difference !="
            ),
            Self::LooseJamo(c) => write!(f, "incomplete syllable '{c}'"),
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for LexError {}
