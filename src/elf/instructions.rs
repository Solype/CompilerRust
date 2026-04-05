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
    Symbol(String)
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

#[allow(dead_code)]
impl Instruction {
    fn encode_move(&self, op1: &Operand, op2: &Operand) -> Vec<u8>
    {
        match (op1, op2) {
            (Operand::Reg(RegisterArch::X32(reg)), Operand::Imm(val)) => {
                let mut v = vec![0xB8 + *reg as u8];
                v.extend(&val.to_le_bytes());
                return v;
            }
            _ => unimplemented!(),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        match self {
            Instruction::Mov { dst, src } => self.encode_move(dst, src),
            Instruction::Int(n) => vec![0xCD, *n],
            _ => unimplemented!(),
        }
    }
}
