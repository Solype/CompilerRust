#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
#[allow(dead_code)]
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
    One = 0,   // ×1
    Two = 1,   // ×2
    Four = 2,  // ×4
    Eight = 3, // ×8
}

#[derive(Debug, Clone)]
pub enum MemDisplacement {
    Imm(i32),
    Sym(String),
}#[derive(Debug, Clone)]

pub enum MemAddress {
    Direct {
        disp: MemDisplacement,
    },

    Base {
        base: Register,
    },

    BaseDisp {
        base: Register,
        disp: MemDisplacement,
    },

    IndexScaleDisp {
        index: Register,
        scale: Scale,
        disp: MemDisplacement,
    },

    BaseIndex {
        base: Register,
        index: Register,
    },

    BaseIndexScale {
        base: Register,
        index: Register,
        scale: Scale,
    },

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
    Imm(u32),
    Sym(String),
    MemoryAddress(MemAddress),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Mov {
        dst: Operand,
        src: Operand,
    },
    Add {
        dst: Operand,
        src: Operand,
    },
    Sub {
        dst: Operand,
        src: Operand,
    },
    Push(Operand),
    Int(u8), // ex: int 0x80
    Syscall,
    Ret,
    Call(Operand),
    Jmp(Operand),
    Cmp((Operand, Operand))
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
