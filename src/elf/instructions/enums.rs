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

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
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

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    // Data movement
    Mov,

    // Arithmetic
    Add,
    Sub,

    // Logic
    And,
    Or,
    Xor,

    // Comparaison (flags only)
    Cmp,
    Test,

    // Shifts
    Shl,
    Shr,
}

pub struct BinaryEncoding {
    pub opcode_rm_r: u8,
    pub opcode_r_rm: u8,
    pub opcode_imm: u8,
    pub modrm_ext: u8,
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
}

#[derive(Debug, Clone, Copy)]
pub enum SysOp {
    Int(u8),
    Syscall,
    Sysenter,
}


pub enum Instruction {
    Binary {
        op: BinOp, // Mov, Add, Sub, And, etc.
        dst: Operand,
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

    LocalSym (String)
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
