use std::fmt;

use crate::lexer::span::Span;

/// What the grammar accepts but the AST cannot hold
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostParseErrorKind {
    /// `짧은 논리`: only 정수 and 소수 have a 32-bit form
    ShortType,
    /// `부호 없는 소수`: only 정수 has an unsigned form
    UnsignedType,
    /// `고정된 수는 정수예요`: a constant gets its value where it is declared
    ConstantWithoutValue(String),
    /// `정수 수를` out of a `세면서`: only the loop declares its variable in its arguments
    DeclaredOutOfCount(String),
    /// `세면서` without its variable, its `부터` or its `까지`, or with another argument
    CountArguments,
    /// `반환해요` with no value, two values, or a value without `을/를`
    GiveArguments,
    /// `넣어요` without its value or its variable with `에`, or with another argument
    PutArguments,
    /// `정수 가와 나를`: a name alone after a parameter without counter
    ParameterWithoutType(String),
    /// `두 정수 가와 나와 다를`: the counter announces `expected` names, the group has `found`
    CounterMismatch { expected: usize, found: usize },
}

/// A post-parsing error and where it happened
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostParseError {
    pub kind: PostParseErrorKind,
    pub span: Span,
}

impl PostParseError {
    pub fn new(kind: PostParseErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for PostParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShortType => write!(f, "only 정수 and 소수 can be 짧은"),
            Self::UnsignedType => write!(f, "only 정수 can be 부호 없는"),
            Self::ConstantWithoutValue(name) => {
                write!(f, "the constant '{name}' needs a value where it is declared")
            }
            Self::DeclaredOutOfCount(name) => {
                write!(f, "'{name}' can only be declared by a 세면서 loop")
            }
            Self::CountArguments => write!(
                f,
                "세면서 takes a variable with 을/를, a start with 부터 and an end with 까지"
            ),
            Self::GiveArguments => write!(f, "반환해요 takes one value, with 을/를"),
            Self::PutArguments => {
                write!(f, "넣어요 takes a value with 을/를 and a variable with 에")
            }
            Self::ParameterWithoutType(name) => write!(
                f,
                "the parameter '{name}' has no type: write it before the name, or count the \
                 parameters of one type (두 정수 가와 {name})"
            ),
            Self::CounterMismatch { expected, found } => write!(
                f,
                "the counter announces {expected} parameters of this type, but {found} follow"
            ),
        }
    }
}

impl fmt::Display for PostParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for PostParseError {}
