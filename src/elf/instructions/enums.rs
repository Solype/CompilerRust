use super::{MemAddress, register::*};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Size {
    U8 = 1,
    U16 = 2,
    U32 = 4,
    U64 = 8,
}

impl From<usize> for Size {
    fn from(value: usize) -> Self {
        match value {
            4 => Size::U32,
            8 => Size::U64,
            _ => panic!("invalid scale value {}", value),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    NoOperand,
    Reg(Register),
    Imm(i64),
    Sym(String),
    MemoryAddress(MemAddress),
}

#[derive(Clone, Copy, Debug)]
pub enum ConditionCode {
    // Equal / Zero
    E,
    NE,

    // Signed comparisons
    G,
    GE,
    L,
    LE,

    // Unsigned comparisons
    A,
    AE,
    B,
    BE,

    // Sign flag
    S,
    NS,

    // Overflow flag
    O,
    NO,

    // Parity flag
    P,
    NP,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    // Data movement
    Mov,
    Xchg,

    // Arithmetic
    Add,
    Sub,

    // Arithmetic with carry
    Adc,
    Sbb,

    // Logic
    And,
    Or,
    Xor,

    // Comparaison
    Cmp,
    Test,

    // SSE scalar float
    LoadF,
    StoreF,
    AddF,
    SubF,
    MulF,
    DivF,
    ComiF,
    UcomiF,
}

#[derive(Debug, Clone, Copy)]
pub enum ShiftOp {
    // Shifts
    Shl,
    Shr,
    Sar,

    // Rotations (souvent utile)
    Rol,
    Ror,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ComplexBinOp {
    Xadd,
    Cmpxchg,
    Mul,
    Div,
    Imul,
    Idiv,
}

#[derive(Debug, Clone, Copy)]
pub enum BitOp {
    Bt,
    Bts,
    Btr,
    Btc,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    Inc,
    Dec,
    Nop,

    Cdq,
    Cqo,
    Cwd,

    Clc,
    Cld,
    Stc,
    Std,
    Cmc,

    Ud2,
    Hlt,
    Pause,
    Fwait,

    Cli,
    Sti,

    Lahf,
    Sahf,

    Cbw,
    Cwde,
    Cdqe,
}

#[derive(Debug, Clone, Copy)]
pub enum StackOp {
    Push,
    Pop,
    Pushf,
    Popf,
    Leave,
    Enter(u8),
}

#[derive(Debug, Clone, Copy)]
pub enum CtrlOp {
    Jmp,
    Call,
    Ret,
    IRet,
    JmpCC(ConditionCode),
    Loop,
    Loope,
    Loopne,
}

#[derive(Debug, Clone, Copy)]
pub enum SysOp {
    Int(u8),
    Syscall,
    Sysenter,
}

#[derive(Debug, Clone, Copy)]
pub enum BitScanOp {
    Bsf,
    Bsr,
}

#[derive(Debug)]
pub enum StringOp {
    Movs,
    Cmps,
    Scas,
    Lods,
    Stos,
}

#[derive(Debug, Clone, Copy)]
pub enum Prefix {
    Lock,
    Rep,
    Repe,
    Repne,

    Cs,
    Ds,
    Es,
    Ss,
    Fs,
    Gs,
}

/// Extension d'un entier vers un registre plus large
#[derive(Debug, Clone, Copy)]
pub enum ExtendOp {
    Movsx,
    Movsxd,
    Movzx,
}

#[derive(Debug, Clone, Copy)]
pub enum ConvOp {
    Cvtsd2si,
    Cvtsd2ss,
    Cvtsi2sd,
    Cvtsi2ss,
    Cvtss2sd,
    Cvtss2si,
    Cvttsd2si,
    Cvttss2si,
}

#[derive(Debug)]
pub enum Instruction {
    Binary {
        op: BinOp,
        dst: Operand,
        src: Operand,
        size: Option<Size>,
    },

    Bit {
        op: BitOp,
        dst: Operand,
        src: Operand,
        size: Option<Size>,
    },

    BitScan {
        op: BitScanOp,
        dst: Register,
        src: Operand,
        size: Option<Size>,
    },

    CMovCC {
        cc: ConditionCode,
        dst: Register,
        src: Operand,
        size: Option<Size>,
    },

    ComplexBinary {
        op: ComplexBinOp,
        dst: Operand,
        src: Operand,
        extra: Option<Operand>,
        size: Option<Size>,
    },

    Convert {
        op: ConvOp,
        dst: Register,
        src: Operand,
        size: Option<Size>,
    },

    Ctrl {
        op: CtrlOp,
        target: Operand,
    },

    /// `src_size` est la taille de la source, `size` celle de la destination
    Extend {
        op: ExtendOp,
        dst: Register,
        src: Operand,
        src_size: Option<Size>,
        size: Option<Size>,
    },

    Lea {
        src: Operand,
        dst: Register,
        size: Option<Size>,
    },

    LocalSym(String),

    Nop(u8),

    Prefix {
        prefix: Vec<Prefix>,
        ins: Box<Instruction>,
    },

    SetCC {
        op: ConditionCode,
        dst: Operand,
    },

    Shift {
        op: ShiftOp,
        dst: Operand,
        src: Operand,
        size: Option<Size>,
    },

    Stack {
        op: StackOp,
        value: Operand,
        size: Option<Size>,
    },

    String {
        op: StringOp,
        size: Option<Size>,
    },

    Sys {
        op: SysOp,
    },

    Unary {
        op: UnaryOp,
        dst: Operand,
        size: Option<Size>,
    },
}

#[derive(Debug, Clone, Default)]
pub struct Relocation {
    pub sym: String,
    pub offset: usize,
    pub size: u8, // en bytes (1, 2, 4, 8)
    pub kind: RelocKind,
    pub addend: i32,
}

#[derive(Debug, Clone, Default)]
pub enum RelocKind {
    Relative,
    #[default]
    Absolute,
}

pub const MEMNODISP: u8 = 0b00;
pub const MEMDISP8: u8 = 0b01;
pub const MEMDISP32: u8 = 0b10;
pub const REG: u8 = 0b11;
