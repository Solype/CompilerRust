use super::super::parser_token::Particle;

/// The whole file
#[derive(Default)]
pub struct Program {
    pub functions: Vec<Function>,
}

/// `정수를 주는 더하다(정수 가, 정수 나) { … }`, or `외부 …(…)` without a body
pub struct Function {
    pub name: String,
    /// `None` without `TYPE을 주는`: the function returns nothing
    pub return_type: Option<Type>,
    pub params: Vec<Param>,
    /// `, …` after the last parameter: any number of arguments (`printf`)
    pub variadic: bool,
    /// `None` for `외부`: declared here, defined elsewhere (libc)
    pub body: Option<Vec<Statement>>,
}

/// `정수 가`
pub struct Param {
    pub ty: Type,
    pub name: String,
}

pub enum Type {
    /// 정수: 64 bits; `짧은 정수`: 32 bits; `부호 없는 정수`: unsigned
    Int {
        short: bool,
        unsigned: bool,
    },
    /// 실수: 64 bits; `짧은 실수`: 32 bits
    Float {
        short: bool,
    },
    /// 논리
    Bool,
    /// 문자
    Char,
    /// 바이트
    Byte,
    /// `TYPE&` or `TYPE 주소`: a pointer to TYPE
    Address(Box<Type>),
    /// `TYPE N개`: N values of TYPE
    Array(Box<Type>, usize),
    Custom(String),
}

pub enum Statement {
    /// `수는 정수예요.`: a variable without a value yet
    Declare { name: String, ty: Type },
    /// `결과는 0이에요.`, `고정된 끝은 10이에요.`, `합은 3과 4를 더한 값이에요.`
    Init {
        name: String,
        value: Expr,
        /// `고정된`
        constant: bool,
    },
    /// A chain ending with `넣어요`: `3과 4를 더해서 결과에 넣어요.`
    Assign { target: String, value: Expr },
    /// A chain ending with `줘요`: `(가 + 나)를 줘요.`
    Return(Expr),
    /// A chain ending with any other verb, its result unused: `"%ld\n"과 결과를 printf해요.`
    Call(Expr),
    /// `만약 COND면 { … } 아니면 { … }`; `아니면 만약` is an `If` alone in `otherwise`
    If {
        condition: Expr,
        then: Vec<Statement>,
        otherwise: Option<Vec<Statement>>,
    },
    /// `COND인 동안 { … }`; `COND이 아닌 동안` has its condition negated
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    /// `수를 1부터 끝까지 세면서 { … }`: from `from` included to `until` excluded
    For {
        variable: String,
        /// `정수 수를 …`: the variable is declared by the loop
        ty: Option<Type>,
        from: Expr,
        until: Expr,
        body: Vec<Statement>,
    },
    /// 그만해요
    Break,
    /// 넘어가요
    Continue,
}

pub enum Expr {
    Int(i64),
    Str(String),
    Bool(bool),
    Name(String),
    /// `빈 주소`
    Null,
    /// `정수의 크기`
    SizeOf(Type),
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// One step of a chain: `수를 제곱해서` → `제곱하다(수)`. The step before it gives its first
    /// argument: `수를 제곱해서 결과를 더해서` → `더하다(제곱하다(수), 결과)`
    Call {
        verb: String,
        args: Vec<Argument>,
    },
}

/// An argument of a call, with its particle: the particle says its role (`결과에`: into 결과)
pub enum Argument {
    /// `가를`, `3과`, `결과에`, `1부터`, `끝까지`
    Value { value: Expr, particle: Particle },
    /// The result of the step before, always the first argument: in `수를 제곱해서 결과를 더해서`,
    /// `제곱하다(수)` for 더하다
    Previous(Expr),
    /// `정수로`: a target type (`바꾸다`)
    ToType(Type),
}

pub enum UnaryOp {
    /// `-`
    Neg,
    /// `~`
    BitNot,
    /// No token: `아니면`, `아닌 동안`
    Not,
}

pub enum BinaryOp {
    /// 또는
    Or,
    /// 그리고
    And,
    BitOr,
    BitXor,
    BitAnd,
    Eq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
    Shl,
    Shr,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}
