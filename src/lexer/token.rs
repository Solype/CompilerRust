use std::fmt;

use super::span::Span;

/// What a token is; the lexer knows no keyword: 정수를, 만약, 줘요 are all `HangulWord`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// Hangul syllables only, particle included: `나를`, `정수를`, `해요`
    HangulWord(String),
    /// Latin letters and `_`, no digits: `main`, `printf`
    LatinWord(String),
    /// Unsigned: `-` is its own token
    Int(i64),
    /// Decoded content: `"%ld\n"` holds a real newline
    Str(String),

    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    /// `.` after a final `요` (checked by the parser)
    Dot,
    /// `…` or `...`
    Ellipsis,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Amp,
    Pipe,
    Caret,
    Tilde,
    Shl,
    Shr,
    Lt,
    Gt,
    Le,
    Ge,
    EqEq,
    NotEq,
    /// End of the source, span `len..len`
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl TokenKind {
    /// The variant name, without its value
    pub fn name(&self) -> &'static str {
        match self {
            Self::HangulWord(_) => "HangulWord",
            Self::LatinWord(_) => "LatinWord",
            Self::Int(_) => "Int",
            Self::Str(_) => "Str",
            Self::LParen => "LParen",
            Self::RParen => "RParen",
            Self::LBrace => "LBrace",
            Self::RBrace => "RBrace",
            Self::Comma => "Comma",
            Self::Dot => "Dot",
            Self::Ellipsis => "Ellipsis",
            Self::Plus => "Plus",
            Self::Minus => "Minus",
            Self::Star => "Star",
            Self::Slash => "Slash",
            Self::Percent => "Percent",
            Self::Amp => "Amp",
            Self::Pipe => "Pipe",
            Self::Caret => "Caret",
            Self::Tilde => "Tilde",
            Self::Shl => "Shl",
            Self::Shr => "Shr",
            Self::Lt => "Lt",
            Self::Gt => "Gt",
            Self::Le => "Le",
            Self::Ge => "Ge",
            Self::EqEq => "EqEq",
            Self::NotEq => "NotEq",
            Self::Eof => "Eof",
        }
    }
}

/// `HangulWord  외부`, `Str  "%ld\n"`, `LParen`: the format of `--tokens`
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.name();
        match self {
            Self::HangulWord(word) | Self::LatinWord(word) => write!(f, "{name:<11} {word}"),
            Self::Int(value) => write!(f, "{name:<11} {value}"),
            Self::Str(value) => write!(f, "{name:<11} {value:?}"),
            _ => write!(f, "{name}"),
        }
    }
}
