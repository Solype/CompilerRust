#[derive(Debug, Clone, Copy)]
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
#[allow(dead_code)]
pub enum RegisterArch {
    X32(Register),
    X64(Register)
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Operand {
    Reg(RegisterArch),
    Imm(u32),
    Sym(String)
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
    pub symbol: String,
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

#[derive(Default)]
#[allow(dead_code)]
pub struct EncodeInformation {
    pub data: Vec<u8>,
    pub relocations: Vec<Relocation>,
}

#[allow(dead_code)]
impl Instruction {
    fn encode_move(&self, op1: &Operand, op2: &Operand) -> EncodeInformation
    {
        match (op1, op2) {
            (Operand::Reg(RegisterArch::X32(reg)), Operand::Imm(val)) => {
                let mut v = vec![0xB8 + *reg as u8];
                v.extend(&val.to_le_bytes());
                return EncodeInformation { data: v, ..Default::default() };
            }

            (Operand::Reg(RegisterArch::X32(reg)), Operand::Sym(sym)) => {
                let mut v = vec![0xB8 + *reg as u8];

                let offset = v.len();
                v.extend(&0u32.to_le_bytes());

                EncodeInformation { data: v, relocations: vec![ Relocation {
                            symbol: sym.clone(), offset, size: 4, kind: RelocKind::Absolute, addend: 0
                    }],
                }
            }
            _ => unimplemented!(),
        }
    }

    fn encode_jmp(&self, op: &Operand) -> EncodeInformation {
        match op {
            Operand::Sym(sym) => {
                let mut v = vec![0xE9];

                let offset = v.len();
                v.extend(&(0u32).to_le_bytes());

                EncodeInformation {
                    data: v,
                    relocations: vec![Relocation {
                        symbol: sym.clone(),
                        offset,
                        size: 4,
                        kind: RelocKind::Relative,
                        addend: -4
                    }],
                }
            }

            _ => unimplemented!(),
        }
    }

    pub fn encode(&self) -> EncodeInformation {
        match self {
            Instruction::Mov { dst, src } => self.encode_move(dst, src),
            Instruction::Int(n) => EncodeInformation { data: vec![0xCD, *n], ..Default::default() },
            Instruction::Jmp(op) => self.encode_jmp(op),
            _ => unimplemented!(),
        }
    }
}
