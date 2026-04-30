#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Register {
    A = 0,
    C = 1,
    D = 2,
    B = 3,
    Sp = 4,
    Bp = 5,
    Si = 6,
    Di = 7,
}

impl TryFrom<u8> for Register {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::A),
            1 => Ok(Register::C),
            2 => Ok(Register::D),
            3 => Ok(Register::B),
            4 => Ok(Register::Sp),
            5 => Ok(Register::Bp),
            6 => Ok(Register::Si),
            7 => Ok(Register::Di),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Scale {
    One = 0,
    Two = 1,
    Four = 2,
    Eight = 3,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Size {
    U8 = 1,
    U16 = 2,
    U32 = 4,
    U64 = 8
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
pub enum MemDisplacement {
    Imm(i32),
    Sym(String),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MemAddress {
    /// Absolute memory address: [disp]
    /// Examples:
    /// [0x401000]
    /// [symbol]
    Direct {
        disp: MemDisplacement,
    },

    /// Base register only: [base]
    /// Examples:
    /// [eax]
    /// [rbx]
    Base {
        base: Register,
    },

    /// Base register + displacement: [base + disp]
    /// Examples:
    /// [ebp - 4]
    /// [rbx + symbol]
    BaseDisp {
        base: Register,
        disp: MemDisplacement,
    },

    /// Index register * scale + displacement: [index * scale + disp]
    /// Examples:
    /// [ecx * 4 + 8]
    /// [rdx * 8 + array]
    IndexScaleDisp {
        index: Register,
        scale: Scale,
        disp: MemDisplacement,
    },

    /// Base register + index register: [base + index]
    /// Examples:
    /// [eax + ecx]
    /// [rbx + rsi]
    BaseIndex {
        base: Register,
        index: Register,
    },

    /// Base register + index register * scale: [base + index * scale]
    /// Examples:
    /// [rax + rcx * 4]
    /// [rbx + rdx * 8]
    BaseIndexScale {
        base: Register,
        index: Register,
        scale: Scale,
    },

    /// Full SIB addressing: [base + index * scale + disp]
    /// Examples:
    /// [rax + rcx * 4 + 16]
    /// [rbx + rsi * 8 + symbol]
    BaseIndexScaleDisp {
        base: Register,
        index: Register,
        scale: Scale,
        disp: MemDisplacement,
    },
}


#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Operand {
    NoOperand,
    Reg(Register),
    Imm(usize),
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
    Movzx,
    Movsx,
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
}

pub enum ShiftOp {
    // Shifts
    Shl,
    Shr,
    Sar,

    // Rotations (souvent utile)
    Rol,
    Ror,
}

#[derive(Debug)]
pub enum ComplexBinOp {
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

#[derive(Debug, Clone, Copy)]
pub enum SetCC {
    Sete,
    Setne,
    Setg,
    Setl,
    Setge,
    Setle,
    Seta,
    Setb,

    Setae,
    Setbe,

    Sets,
    Setns,

    Seto,
    Setno,

    Setp,
    Setnp,
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
    Stc,
    Cmc,

    Cli,
    Sti
}

#[derive(Debug, Clone, Copy)]
pub enum StackOp {
    Push,
    Pop,
}

#[derive(Debug, Clone, Copy)]
pub enum CtrlOp {
    Jmp,
    Call,
    Ret,
    JmpCC(ConditionCode)
}

#[derive(Debug, Clone, Copy)]
pub enum SysOp {
    Int(u8),
    Syscall,
    Sysenter,
}

pub enum BitScanOp {
    Bsf,
    Bsr,
}

pub enum Instruction {
    Binary {
        op: BinOp,
        dst: Operand,
        src: Operand,
        size: Option<Size>,
    },

    CMovCC {
        cc: ConditionCode,
        dst: Register,
        src: Operand,
        size: Option<Size>
    },

    Lea {
        src: Operand,
        dst: Register,
        size: Option<Size>
    },

    Shift {
        op: ShiftOp,
        dst: Operand,
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

    Unary {
        op: UnaryOp,
        dst: Operand,
        size: Option<Size>,
    },

    Stack {
        op: StackOp,
        value: Operand,
        size: Option<Size>,
    },

    Bit {
        op: BitOp,
        dst: Operand,
        src: Operand,
        size: Option<Size>
    },

    SetCC {
        op: SetCC,
        dst: Operand,
    },

    BitScan {
        op: BitScanOp,
        dst: Register,
        src: Operand,
        size: Option<Size>,
    },

    Ctrl {
        op: CtrlOp,
        target: Operand,
    },

    Sys {
        op: SysOp,
    },

    LocalSym(String),
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
    Absolute
}

pub const MEMNODISP : u8 = 0b00;
pub const MEMDISP8 : u8 = 0b01;
pub const MEMDISP32 : u8 = 0b10;
pub const REG : u8 = 0b11;


#[derive(Default)]
#[allow(dead_code)]
pub struct EncodeInformation {
    pub data: Vec<u8>,
    pub relocations: Vec<Relocation>,
}
