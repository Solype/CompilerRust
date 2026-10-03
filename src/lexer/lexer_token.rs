use std::fmt;

use super::span::Span;

/// What a token is; the lexer knows no keyword: 정수를, 만약, 줘요 are all `HangulWord`,
/// the pre-parser turns them into `ParserToken`s
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerTokenKind {
    /// Hangul syllables only, particle included: `나를`, `정수를`, `해요`
    HangulWord(String),
    /// Latin letters and `_`, no digits: `main`, `printf`
    LatinWord(String),
    /// Unsigned: `-` is its own token
    Int(i64),
    /// Decoded content: `"%ld\n"` holds a real newline
    Str(String),

    Punctuation(Punctuation),

    Operator(Operator),
    /// End of the source, span `len..len`
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punctuation {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    /// `.` after a final `요` (checked by the parser)
    Dot,
    /// `…` or `...`
    Ellipsis,
}

/// C operators, with C precedence (decided by the parser)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// `+`
    Plus,
    /// `-`, binary or unary: `-5` is `Minus` then `Int(5)`
    Minus,
    /// `*`
    Star,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `&`
    Amp,
    /// `|`
    Pipe,
    /// `^`
    Caret,
    /// `~`
    Tilde,
    /// `<<`
    Shl,
    /// `>>`
    Shr,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `==`
    EqEq,
    /// `!=`
    NotEq,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerToken {
    pub kind: LexerTokenKind,
    pub span: Span,
}

impl LexerToken {
    pub fn new(kind: LexerTokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl LexerTokenKind {
    /// The variant name, without its value
    pub fn name(&self) -> &'static str {
        match self {
            Self::HangulWord(_) => "HangulWord",
            Self::LatinWord(_) => "LatinWord",
            Self::Int(_) => "Int",
            Self::Str(_) => "Str",
            Self::Punctuation(_) => "Punctuation",
            Self::Operator(_) => "Operator",
            Self::Eof => "Eof",
        }
    }
}

/// `HangulWord  외부`, `Punctuation LParen`, `Eof`: the format of `--tokens`
impl fmt::Display for LexerTokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name();
        match self {
            Self::HangulWord(word) | Self::LatinWord(word) => write!(f, "{name:<11} {word}"),
            Self::Int(value) => write!(f, "{name:<11} {value}"),
            Self::Str(value) => write!(f, "{name:<11} {value:?}"),
            Self::Punctuation(punctuation) => write!(f, "{name:<11} {punctuation:?}"),
            Self::Operator(operator) => write!(f, "{name:<11} {operator:?}"),
            _ => write!(f, "{name}"),
        }
    }
}
