// The IR is built step by step and the codegen does not read it yet
#![allow(dead_code)]

use crate::parser::postparser::structs::{BinaryOp, Type, UnaryOp};

pub struct Function {
    pub symbol: String, // déjà manglé : "더하다$int$int", "printf", "main"
    pub params: Vec<VarId>,
    pub vars: Vec<Type>, // type de chaque VarId : params, variables, temporaires
    pub labels: Vec<String>, // nom de chaque Label : "loop_0_1_start"
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
            labels: Vec::new(),
            return_type,
            body: None,
            variadic,
        }
    }

    /// A new variable of type `ty`: its id is its index in `vars`, unique in the whole function
    pub fn new_var(&mut self, ty: Type) -> VarId {
        self.vars.push(ty);
        VarId::new(self.vars.len() - 1)
    }

    /// The label `name`: its id is its index in `labels`, added the first time it is asked
    pub fn label_id(&mut self, name: &str) -> Label {
        if let Some(id) = self.labels.iter().position(|label| label == name) {
            return Label::new(id);
        }
        self.labels.push(name.to_string());
        Label::new(self.labels.len() - 1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VarId(pub usize);

impl VarId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub struct Label(pub usize);

impl Label {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub enum Operand {
    Var(VarId),
    Int(i64),
    Float(usize),
    Bool(bool),
    Str(usize),
    Null,
}

#[derive(Debug)]
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
    /// The source statement of the instructions after it, for the display; no code
    Comment(String),
}

#[derive(Default)]
pub struct Program {
    pub funcs: Vec<Function>,
    pub strings: Vec<String>,
    pub floats: Vec<(f64, bool)>, // valeur et short : écrite sur 4 octets si 짧은 소수
}

impl Program {
    pub fn new() -> Self {
        Self::default()
    }

    /// The string `value`: its id is its index in `strings`, added the first time it is asked
    pub fn string_id(&mut self, value: &str) -> usize {
        if let Some(id) = self.strings.iter().position(|string| string == value) {
            return id;
        }
        self.strings.push(value.to_string());
        self.strings.len() - 1
    }

    /// The float `value` on 4 bytes if `short`, on 8 otherwise: its id is its index in `floats`,
    /// added the first time it is asked; compared by its bits, as f64 is not Eq
    pub fn float_id(&mut self, value: f64, short: bool) -> usize {
        let key = (value.to_bits(), short);
        if let Some(id) = self
            .floats
            .iter()
            .position(|(float, float_short)| (float.to_bits(), *float_short) == key)
        {
            return id;
        }
        self.floats.push((value, short));
        self.floats.len() - 1
    }
}

/// The size in bytes of a value of type `value`
pub fn get_size_of(value: &Type) -> i64 {
    return match value {
        Type::Int { short: true, .. } => 4,
        Type::Int { short: false, .. } => 8,
        Type::Float { short: true } => 4,
        Type::Float { short: false } => 8,
        Type::Address(_) => 8,
        Type::Array(ty, nb) => get_size_of(ty.as_ref()) * (*nb as i64),
        Type::Char => 1,
        Type::Byte => 1,
        _ => unimplemented!(),
    };
}
