use crate::parser::postparser::structs::{BinaryOp, Type, UnaryOp};

pub struct Function {
    pub symbol: String, // déjà manglé : "더하다$int$int", "printf", "main"
    pub params: Vec<VarId>,
    pub vars: Vec<Type>, // type de chaque VarId : params, variables, temporaires
    pub return_type: Option<Type>,
    pub body: Option<Vec<Instr>>, // None pour un 외부
    pub variadic: bool,
}

impl Function {
    /// A function without parameters, variables nor body: a `외부` until its body is added
    pub fn new(symbol: String, return_type: Option<Type>, variadic: bool) -> Self {
        Self {
            symbol,
            params: Vec::new(),
            vars: Vec::new(),
            return_type,
            body: None,
            variadic,
        }
    }
}

pub struct VarId(pub usize);

impl VarId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

pub struct Label(pub usize);

impl Label {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

pub enum Operand {
    Var(VarId),
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(usize),
    Null,
}

pub enum Instr {
    Copy {
        dest: VarId,
        src: Operand,
    },
    Unary {
        dest: VarId,
        op: UnaryOp,
        src: Operand,
    },
    Binary {
        dest: VarId,
        op: BinaryOp,
        left: Operand,
        right: Operand,
    },
    Cast {
        dest: VarId,
        src: Operand,
    }, // types lus dans vars
    Call {
        dest: Option<VarId>,
        symbol: String,
        args: Vec<Operand>,
    },
    Label(Label),
    Jump(Label),
    JumpIfNot {
        cond: Operand,
        target: Label,
    },
    Return(Operand),
}

#[derive(Default)]
pub struct Program {
    pub funcs: Vec<Function>,
    pub strings: Vec<String>,
    pub floats: Vec<f64>,
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }
}
