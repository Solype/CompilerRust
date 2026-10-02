use super::{
    enums::*,
    modrm::mod_rm_encode,
    register::{Register, RegisterClass},
    struct_encode_information::*,
    utils::emit_rex,
};

impl CtrlOp {
    pub(super) fn encode(self, target: &Operand, size: Size) -> EncodeInformation {
        match self {
            // =====================================
            // unconditional
            // =====================================
            CtrlOp::Jmp => match target {
                Operand::Reg(_) | Operand::MemoryAddress(_) => encode_indirect(4, target),
                _ => encode_rel32_with_prefix(None, 0xE9, target),
            },
            CtrlOp::Call => match target {
                Operand::Reg(_) | Operand::MemoryAddress(_) => encode_indirect(2, target),
                _ => encode_rel32_with_prefix(None, 0xE8, target),
            },
            CtrlOp::Ret => EncodeInformation {
                data: vec![0xC3],
                relocations: vec![],
            },
            CtrlOp::IRet => EncodeInformation {
                data: match size {
                    Size::U32 => vec![0xCF],
                    Size::U64 => vec![0x48, 0xCF],
                    _ => unimplemented!("does not support IRet for U8 and U16"),
                },
                relocations: vec![],
            },
            // =====================================
            // equality / zero flag
            // =====================================
            CtrlOp::JmpCC(cc) => match cc {
                ConditionCode::E => encode_rel32_with_prefix(Some(0x0F), 0x84, target),
                ConditionCode::NE => encode_rel32_with_prefix(Some(0x0F), 0x85, target),
                ConditionCode::G => encode_rel32_with_prefix(Some(0x0F), 0x8F, target),
                ConditionCode::GE => encode_rel32_with_prefix(Some(0x0F), 0x8D, target),
                ConditionCode::L => encode_rel32_with_prefix(Some(0x0F), 0x8C, target),
                ConditionCode::LE => encode_rel32_with_prefix(Some(0x0F), 0x8E, target),
                ConditionCode::A => encode_rel32_with_prefix(Some(0x0F), 0x87, target),
                ConditionCode::AE => encode_rel32_with_prefix(Some(0x0F), 0x83, target),
                ConditionCode::B => encode_rel32_with_prefix(Some(0x0F), 0x82, target),
                ConditionCode::BE => encode_rel32_with_prefix(Some(0x0F), 0x86, target),
                ConditionCode::S => encode_rel32_with_prefix(Some(0x0F), 0x88, target),
                ConditionCode::NS => encode_rel32_with_prefix(Some(0x0F), 0x89, target),
                ConditionCode::O => encode_rel32_with_prefix(Some(0x0F), 0x80, target),
                ConditionCode::NO => encode_rel32_with_prefix(Some(0x0F), 0x81, target),
                ConditionCode::P => encode_rel32_with_prefix(Some(0x0F), 0x8A, target),
                ConditionCode::NP => encode_rel32_with_prefix(Some(0x0F), 0x8B, target),
            },
            // =====================================
            // loop family (rel8 uniquement)
            // =====================================
            CtrlOp::Loop => encode_rel8(0xE2, target),
            CtrlOp::Loope => encode_rel8(0xE1, target),
            CtrlOp::Loopne => encode_rel8(0xE0, target),
        }
    }
}

/// A label is resolved in place by the assembler (PC-relative); a symbol may
/// be external and goes through the PLT, like GNU as >= 2.31
fn branch_kind(target: &Target) -> RelocKind {
    match target {
        Target::Sym(_) => RelocKind::Plt32,
        Target::Label(_) => RelocKind::Relative,
    }
}

fn encode_rel32_with_prefix(prefix: Option<u8>, opcode: u8, target: &Operand) -> EncodeInformation {
    match target {
        Operand::Sym(_) | Operand::Label(_) => {
            let target = target.target().unwrap();
            let mut v = Vec::with_capacity(6); // micro-opt

            if let Some(p) = prefix {
                v.push(p);
            }

            v.push(opcode);

            let offset = v.len();
            v.extend(&(0u32).to_le_bytes());

            EncodeInformation {
                data: v,
                relocations: vec![Relocation {
                    kind: branch_kind(&target),
                    target,
                    offset,
                    size: 4,
                    addend: -4,
                }],
            }
        }

        Operand::Imm(val) => {
            let mut v: Vec<u8> = Vec::new();

            if let Some(p) = prefix {
                v.push(p);
            }

            v.push(opcode);
            v.extend((*val as i32).to_le_bytes());

            EncodeInformation {
                data: v,
                relocations: vec![],
            }
        }

        _ => unimplemented!("unsupported target for rel32: {:?}", target),
    }
}

/// Indirect call / jmp: FF /2 and FF /4, target in r/m (64 bits without REX.W)
fn encode_indirect(ext: u8, target: &Operand) -> EncodeInformation {
    let mut v = EncodeInformation::new();

    emit_rex(&mut v, Size::U32, None, target);
    v.push(0xFF);

    let reg_field = Operand::Reg(Register::new(RegisterClass::Gpr, ext));
    v.append(mod_rm_encode(target, &reg_field));
    v
}

fn encode_rel8(opcode: u8, target: &Operand) -> EncodeInformation {
    match target {
        Operand::Sym(_) | Operand::Label(_) => {
            let mut v = Vec::with_capacity(2);

            v.push(opcode);

            let offset = v.len();
            v.push(0); // placeholder i8

            EncodeInformation {
                data: v,
                relocations: vec![Relocation {
                    target: target.target().unwrap(),
                    offset,
                    size: 1,
                    kind: RelocKind::Relative,
                    addend: -1,
                }],
            }
        }

        Operand::Imm(val) => {
            let mut v = Vec::with_capacity(2);

            v.push(opcode);

            let rel = *val as i8; // ⚠️ must be validated elsewhere
            v.push(rel as u8);

            EncodeInformation {
                data: v,
                relocations: vec![],
            }
        }

        _ => unimplemented!("unsupported target for rel8: {:?}", target),
    }
}
