use std::fmt;

use crate::lexer::span::Span;

/// What the parser accepts but the program cannot run
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertErrorKind {
    /// Two functions with the same name and the same parameter types
    SameSignature(String),
    /// Two `외부` or `main` with the same name: their symbol is the name alone
    SameSymbol(String),
    /// A variable or parameter declared twice in the same block
    AlreadyDeclared(String),
    /// A name that is no variable of the blocks around it
    UnknownVariable(String),
    /// `넣어요` or `세면서` on a `고정된` variable
    AssignToConstant(String),
    /// `반환해요` in a function without `TYPE을 반환하는`
    ReturnWithoutType(String),
    /// The condition of `만약` or `동안` is not a 논리
    NotBool,
    /// A bound or the variable of `세면서` is not a 정수
    NotInteger,
    /// `그만해요` out of a loop
    BreakOutOfLoop,
    /// `넘어가요` out of a loop
    ContinueOutOfLoop,
}

/// A conversion error and where it happened
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConvertError {
    pub kind: ConvertErrorKind,
    pub span: Span,
}

impl ConvertError {
    pub fn new(kind: ConvertErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for ConvertErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SameSignature(name) => write!(
                f,
                "the function '{name}' is already defined with these parameter types"
            ),
            Self::SameSymbol(name) => write!(
                f,
                "'{name}' is already declared: 외부 functions and main keep their name alone, \
                 whatever their parameters"
            ),
            Self::AlreadyDeclared(name) => write!(f, "'{name}' is already declared in this block"),
            Self::UnknownVariable(name) => write!(f, "'{name}' is not a variable declared here"),
            Self::AssignToConstant(name) => {
                write!(f, "'{name}' is 고정된: its value cannot change")
            }
            Self::ReturnWithoutType(name) => write!(
                f,
                "'{name}' returns nothing: write TYPE을 반환하는 to return a value"
            ),
            Self::NotBool => write!(f, "a condition must be a 논리"),
            Self::NotInteger => write!(f, "the bounds and the variable of 세면서 must be 정수"),
            Self::BreakOutOfLoop => write!(f, "그만해요 can only be used in a loop"),
            Self::ContinueOutOfLoop => write!(f, "넘어가요 can only be used in a loop"),
        }
    }
}

impl fmt::Display for ConvertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for ConvertError {}
