use crate::lexer::lexer_token::{Operator, Punctuation};
use crate::lexer::span::Span;

/// What the pre-parser makes of the lexer tokens: every Hangul word is split (나를 → 나 + 를)
/// and classified, so the parser only sees fixed categories
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserTokenKind {
    /// A word of the language: 만약, 주는, 외부…
    Keyword(Keyword),
    /// 정수, 실수…; `주소` and `&` after a type are both `Type(Address)`
    Type(Type),
    /// A declared name: variable, parameter, function, `main`, `printf`
    Name(String),
    /// A conjugated verb, user-defined or built-in: 더해서, 줘요, `printf해요` (two lexer tokens)
    Verb { infinitive: String, ending: Ending },
    /// Detached from the word before it: 나를 → `Name(나)` then `Particle(Object)`
    Particle(Particle),

    Int(i64),
    Str(String),
    /// 참 / 거짓
    Bool(bool),

    Punctuation(Punctuation),
    Operator(Operator),

    /// End of the source, span `len..len`
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserToken {
    pub kind: ParserTokenKind,
    /// For a word split in two, each part has its own span inside the word
    pub span: Span,
}

impl ParserToken {
    pub fn new(kind: ParserTokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    /// 외부 : extern function (libc)
    Extern,
    /// 주는 : `TYPE을 주는 VERBE다(…)`, the function returns TYPE
    Giving,
    /// 고정된 : constant
    Fixed,
    /// 짧은 : 32-bit integer or float (`짧은 정수`)
    Short,
    /// 부호 없는 : unsigned (two lexer tokens)
    Unsigned,
    /// 빈 : `빈 주소`, the null pointer
    Empty,
    /// 만약 : opens a condition
    If,
    /// 아니면 : else, or negation after a condition
    Otherwise,
    /// 동안 : `COND인 동안`, while
    While,
    /// 세면서 : `NOM을 A부터 B까지 세면서`, for
    Counting,
    /// 그만해요 : break
    Stop,
    /// 넘어가요 : continue
    Skip,
    /// 그리고 : &&
    And,
    /// 또는 : ||
    Or,
    /// 값 : `3과 4를 더한 값이에요`
    Value,
    /// 크기 : `TYPE의 크기`, sizeof
    Size,
    /// 개 : `정수 10개`, array of 10
    Count,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    /// 정수 : 64-bit integer
    Int,
    /// 실수 : 64-bit float
    Float,
    /// 논리 : boolean
    Bool,
    /// 문자 : character
    Char,
    /// 바이트 : unsigned 8-bit integer
    Byte,
    /// 주소, or `&` after a type : pointer
    Address,
}

/// The form a verb is conjugated in (see hangeul::conjugaison)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    /// -다 : dictionary form, in a declaration (`더하다(…)`)
    Da,
    /// -아/어서 : links a step of a chain (`더해서`)
    Seo,
    /// -아/어요 : ends a chain (`더해요`, `줘요`)
    Yo,
    /// -(으)ㄴ : qualifies a noun (`더한 값`)
    Adnominal,
}

/// One variant per meaning; both spellings (을/를…) give the same variant,
/// the pre-parser can check the right one against the batchim
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Particle {
    /// 을/를 : object
    Object,
    /// 이/가 : subject
    Subject,
    /// 은/는 : topic, declares a variable
    Topic,
    /// 와/과 : and, links arguments
    With,
    /// 에 : into, destination of 넣다
    In,
    /// (으)로 : to, target type of 바꾸다
    To,
    /// 의 : of (`정수의 크기`)
    Of,
    /// 부터 : from
    From,
    /// 까지 : until
    Until,
    /// 보다 : than
    Than,
    /// 이면/면 : if it is, ends a condition
    IfItIs,
    /// 이에요/예요 : it is, ends a declaration
    ItIs,
    /// 인 : that is (`COND인 동안`)
    ThatIs,
}
