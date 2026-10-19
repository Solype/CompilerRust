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
    /// No function of this verb takes arguments of these types
    UnknownFunction { verb: String, types: String },
    /// `넣어요` or `세면서` on a `고정된` variable
    AssignToConstant(String),
    /// `반환해요` in a function without `TYPE을 반환하는`
    ReturnWithoutType(String),
    /// The result of a function without `TYPE을 반환하는` used as a value
    NoValue(String),
    /// A function with `TYPE을 반환하는` that can end without `반환해요`
    MissingReturn(String),
    /// No `main` in the program
    NoMain,
    /// `main` does not return a 정수
    MainNotInteger,
    /// `반환해요` of a value whose type is not the `TYPE을 반환하는` of its function
    WrongReturnType { expected: String, found: String },
    /// `넣어요` of a value whose type is not the type of its variable
    WrongAssignType {
        name: String,
        expected: String,
        found: String,
    },
    /// An operator on operands it does not take: `1 + 참`, `참 그리고 1`
    WrongOperands { op: String, types: String },
    /// Several functions of this verb take these arguments, through their literals
    AmbiguousCall { verb: String, types: String },
    /// A literal whose value does not fit in the type it goes in: `-1` for a 부호 없는 정수
    LiteralOutOfRange(String),
    /// `바꾸다` between types it cannot convert: a 문자 주소 to a 소수
    InvalidCast { from: String, to: String },
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
            Self::AlreadyDeclared(name) => write!(f, "'{name}' is already declared in this block or a block around it"),
            Self::UnknownVariable(name) => write!(f, "'{name}' is not a variable declared here"),
            Self::UnknownFunction { verb, types } => {
                write!(f, "no function '{verb}' takes the arguments ({types})")
            }
            Self::AssignToConstant(name) => {
                write!(f, "'{name}' is 고정된: its value cannot change")
            }
            Self::ReturnWithoutType(name) => write!(
                f,
                "'{name}' returns nothing: write TYPE을 반환하는 to return a value"
            ),
            Self::NoValue(verb) => {
                write!(f, "'{verb}' returns nothing: its result cannot be used")
            }
            Self::MissingReturn(name) => {
                write!(
                    f,
                    "'{name}' can end without 반환해요: every path must return a value"
                )
            }
            Self::NoMain => write!(f, "the program has no main"),
            Self::MainNotInteger => {
                write!(f, "main must return a 정수: write 정수를 반환하는 main")
            }
            Self::WrongReturnType { expected, found } => {
                write!(f, "this function returns {expected}, not {found}")
            }
            Self::WrongAssignType {
                name,
                expected,
                found,
            } => write!(f, "'{name}' has the type {expected}, not {found}"),
            Self::WrongOperands { op, types } => write!(f, "'{op}' cannot take ({types})"),
            Self::AmbiguousCall { verb, types } => write!(
                f,
                "several functions '{verb}' take the arguments ({types}): give the literals \
                 a variable of the type to call"
            ),
            Self::LiteralOutOfRange(ty) => write!(f, "this value does not fit in a {ty}"),
            Self::InvalidCast { from, to } => {
                write!(f, "바꾸다 cannot convert a {from} to a {to}")
            }
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
