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

#[derive(Debug, Clone, Copy)]
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
        size: Option<Size>
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
