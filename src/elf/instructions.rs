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
#[repr(u8)]
#[allow(dead_code)]
pub enum Register64 {
    Rax = 0,
    Rdi = 7, // seulement pour exit
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Operand {
    Reg(Register),
    Imm(u32),
}

#[derive(Debug, Clone, Copy)]
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
}

#[allow(dead_code)]
impl Instruction {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            Instruction::Mov { dst, src } => {
                match (dst, src) {
                    (Operand::Reg(reg), Operand::Imm(val)) => {
                        let mut v = vec![0xB8 + *reg as u8];
                        v.extend(&val.to_le_bytes());
                        return v;
                    }
                    _ => unimplemented!(),
                }
            }
            Instruction::Int(n) => vec![0xCD, *n],
            _ => unimplemented!(),
        }
    }

    pub fn encode_x86_64(&self) -> Vec<u8> {
        match self {
            Instruction::Mov { dst, src } => match (dst, src) {
                (Operand::Reg(Register::Eax), Operand::Imm(val)) => {
                    // 32-bit mov eax, imm32 → opcode 0xB8
                    let mut v = vec![0xB8];
                    v.extend(&val.to_le_bytes());
                    v
                }
                _ => unimplemented!(),
            },
            Instruction::Int(n) => vec![0xCD, *n], // encore utile pour compat 32-bit
            _ => unimplemented!(),
        }
    }
}
