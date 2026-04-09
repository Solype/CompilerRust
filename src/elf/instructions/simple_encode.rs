use super::enums::*;
use super::modrm::*;

#[allow(dead_code)]
impl Instruction {
    fn encode_move(&self, op1: &Operand, op2: &Operand) -> EncodeInformation {
        match (op1, op2) {
            (Operand::Reg(reg), Operand::Imm(val)) => {
                let mut v = vec![0xB8 + *reg as u8];
                v.extend(&val.to_le_bytes());
                EncodeInformation { data: v, ..Default::default() }
            }

            (Operand::Reg(reg), Operand::Sym(sym)) => {
                let mut v = vec![0xB8 + *reg as u8];
                let offset = v.len();
                v.extend(&0u32.to_le_bytes());

                EncodeInformation {
                    data: v,
                    relocations: vec![Relocation {
                        sym: sym.clone(), offset, size: 4, kind: RelocKind::Absolute, addend: 0,
                    }],
                }
            }

            // mov reg, mem ou mem, reg → nécessite ModRM
            (Operand::Reg(_), Operand::RegMemory(_))
            | (Operand::Reg(_), Operand::Reg(_))
            | (Operand::RegMemory(_), Operand::Reg(_))
            | (Operand::RegMemory(_), Operand::RegMemory(_)) => {
                match (op1, op2) {
                    // Reg <- Reg
                    (Operand::Reg(_), Operand::Reg(_)) => {
                        let opcode = 0x89; // mov r/m32, r32
                        let modrm_info = mod_rm_encode(op1, op2);
                        let mut v = vec![opcode];
                        v.extend(modrm_info.data);
                        EncodeInformation { data: v, relocations: modrm_info.relocations }
                    }

                    // Reg <- Mem
                    (Operand::Reg(_), Operand::RegMemory(_)) => {
                        let opcode = 0x8B;
                        let modrm_info = mod_rm_encode(op1, op2);
                        let mut v = vec![opcode];
                        v.extend(modrm_info.data);
                        EncodeInformation { data: v, relocations: modrm_info.relocations }
                    }

                    // Mem <- Reg
                    (Operand::RegMemory(_), Operand::Reg(_)) => {
                        let opcode = 0x89;
                        let modrm_info = mod_rm_encode(op1, op2);
                        let mut v = vec![opcode];
                        v.extend(modrm_info.data);
                        EncodeInformation { data: v, relocations: modrm_info.relocations }
                    }

                    // Mem <- Mem (rare, nécessite un registre temporaire)
                    (Operand::RegMemory(_), Operand::RegMemory(_)) => {
                        panic!("x86 cannot move memory to memory directly");
                    }

                    _ => unimplemented!(),
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