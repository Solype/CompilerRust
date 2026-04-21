#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Register {
    Eax = 0,
    Ecx = 1,
    Edx = 2,
    Ebx = 3,
    Esp = 4,
    Ebp = 5,
    Esi = 6,
    Edi = 7,
}

impl TryFrom<u8> for Register {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::Eax),
            1 => Ok(Register::Ecx),
            2 => Ok(Register::Edx),
            3 => Ok(Register::Ebx),
            4 => Ok(Register::Esp),
            5 => Ok(Register::Ebp),
            6 => Ok(Register::Esi),
            7 => Ok(Register::Edi),
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
pub enum CMovCC {
    Cmove,
    Cmovne,
    Cmovg,
    Cmovl,
    Cmovge,
    Cmovle,

    Cmova,
    Cmovae,
    Cmovb,
    Cmovbe,

    Cmovs,
    Cmovns,

    Cmovo,
    Cmovno,

    Cmovp,
    Cmovnp,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    // Data movement
    Mov,
    Movzx,
    Movsx,
    Xchg,

    // Conditional mov
    CondMov(CMovCC),


    // Arithmetic
    Add,
    Sub,

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

/*
mul r/m8
mul r/m16
mul r/m32
mul r/m64

imul r/m8
imul r/m16
imul r/m32
imul r/m64

imul r16, r/m16
imul r32, r/m32
imul r64, r/m64

imul r16, r/m16, imm8
imul r16, r/m16, imm16

imul r32, r/m32, imm8
imul r32, r/m32, imm32

imul r64, r/m64, imm8
imul r64, r/m64, imm32

div r/m8
div r/m16
div r/m32
div r/m64

idiv r/m8
idiv r/m16
idiv r/m32
idiv r/m64
 */

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

#[derive(Debug, Clone, Copy)]
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

    // conditionnels (optionnel mais utile)
    Je,
    Jne,
    Jg,
    Jl,
    Jge,
    Jle,
    Ja,
    Jae,
    Jb,
    Jbe,

    Js,
    Jns,

    Jo,
    Jno,

    Jp,
    Jnp,
}

#[derive(Debug, Clone, Copy)]
pub enum SysOp {
    Int(u8),
    Syscall,
    Sysenter,
}


pub enum Instruction {
    Binary {
        op: BinOp,
        dst: Operand,
        src: Operand,
        size: Option<Size>,
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
