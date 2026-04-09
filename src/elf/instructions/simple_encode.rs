use super::enums::*;
use super::modrm::*;

#[allow(dead_code)]
impl Instruction {
    fn encode_move(&self, op1: &Operand, op2: &Operand) -> EncodeInformation {
        match (op1, op2) {
            // mov reg, imm
            (Operand::Reg(reg), Operand::Imm(val)) => {
                let mut v = vec![0xB8 + *reg as u8];
                v.extend(&val.to_le_bytes());
                return EncodeInformation { data: v, ..Default::default() };
            }

            // mov reg, symbol
            (Operand::Reg(reg), Operand::Sym(sym)) => {
                let mut v = vec![0xB8 + *reg as u8];
                v.extend(&0u32.to_le_bytes());

                return EncodeInformation {
                    data: v,
                    relocations: vec![Relocation {
                        sym: sym.clone(),
                        offset: 1,
                        size: 4,
                        kind: RelocKind::Absolute,
                        addend: 0,
                    }],
                };
            }

            (
                Operand::Reg(_) | Operand::MemoryAddress(_),
                Operand::Reg(_) | Operand::MemoryAddress(_),
            ) => {
                let opcode = match (op1, op2) {
                    (Operand::Reg(_), _) => 0x8B,
                    (_, Operand::Reg(_)) => 0x89,
                    _ => panic!("x86 cannot move memory to memory directly"),
                };
                let modrm_info = match (op1, op2) {
                    (Operand::Reg(_), _) => mod_rm_encode(op2, op1),
                    _ => mod_rm_encode(op1, op2)
                };

                let mut v = vec![opcode];
                v.extend(modrm_info.data);
                println!("data : {:?}", v);

                EncodeInformation {
                    data: v,
                    relocations: modrm_info.relocations,
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

    pub fn encode(&self) -> EncodeInformation {
        match self {
            Instruction::Mov { dst, src } => self.encode_move(dst, src),
            Instruction::Int(n) => EncodeInformation { data: vec![0xCD, *n], ..Default::default() },
            Instruction::Jmp(op) => self.encode_jmp(op),
            Instruction::Call(op) => self.encode_call(op),
            Instruction::Ret => self.encode_ret(),
            _ => unimplemented!(),
        }
    }
}