use super::enums::*;
use super::encode_mov::encode_move;

#[allow(dead_code)]
impl Instruction {

    fn encode_jmp(&self, op: &Operand) -> EncodeInformation {
        match op {
            Operand::Sym(sym) => {
                let mut v = vec![0xE9];

                let offset = v.len();
                v.extend(&(0u32).to_le_bytes());

                EncodeInformation {
                    data: v,
                    relocations: vec![Relocation { sym: sym.clone(), offset, size: 4, kind: RelocKind::Relative, addend: -4 }],
                }
            }

            _ => unimplemented!(),
        }
    }

    fn encode_ret(&self) -> EncodeInformation {
        EncodeInformation {
            data: vec![0xC3],
            relocations: vec![],
        }
    }

    fn encode_call(&self, op: &Operand) -> EncodeInformation {
        match op {
            Operand::Sym(sym) => {
                let mut v = vec![0xE8]; // opcode CALL

                let offset = v.len();
                v.extend(&(0u32).to_le_bytes());

                EncodeInformation {
                    data: v,
                    relocations: vec![Relocation {
                        sym: sym.clone(), offset, size: 4, kind: RelocKind::Relative, addend: -4,
                    }],
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn encode(&self, size: Size) -> EncodeInformation {
        match self {
            Instruction::Mov { dst, src } => encode_move(dst, src, size),
            Instruction::Int(n) => EncodeInformation { data: vec![0xCD, *n], ..Default::default() },
            Instruction::Jmp(op) => self.encode_jmp(op),
            Instruction::Call(op) => self.encode_call(op),
            Instruction::Ret => self.encode_ret(),
            _ => unimplemented!(),
        }
    }
}