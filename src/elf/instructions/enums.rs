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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum MemDisplacement {
    Imm(i32),
    Sym(String)
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Operand {
    NoOperand,
    Reg(Register),
    Imm(u32),
    Sym(String),
    RegMemory(Register),
    RegMemDisp(Register, i32),
    ComplexMemDisp(Vec<Operand>),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Relocation {
    pub sym: String,
    pub offset: usize,   // offset dans data
    pub size: u8,        // en bytes (1, 2, 4, 8)
    pub kind: RelocKind,
    pub addend: i32,
}

#[derive(Debug, Clone)]
pub enum RelocKind {
    Relative,
    Absolute
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ModRMModField {
    MemNoDisp = 0b00,
    MemDisp8 = 0b01,
    MemDisp32 = 0b10,
    Reg = 0b11,
}

#[derive(Default)]
#[allow(dead_code)]
pub struct EncodeInformation {
    pub data: Vec<u8>,
    pub relocations: Vec<Relocation>,
}
