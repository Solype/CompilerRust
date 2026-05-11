use super::{
    enums::*,
    struct_encode_information::*,
};

impl CtrlOp {
    pub(super) fn encode(self, target: &Operand) -> EncodeInformation {
        match self {
            // =====================================
            // unconditional
            // =====================================
            CtrlOp::Jmp  => encode_rel32_with_prefix(None,       0xE9, target),
            CtrlOp::Call => encode_rel32_with_prefix(None,       0xE8, target),
            CtrlOp::Ret  => EncodeInformation {
                data: vec![0xC3],
                relocations: vec![],
            },
            CtrlOp::IRet => EncodeInformation {
                data: vec![0xCF],
                relocations: vec![],
            },
            // =====================================
            // equality / zero flag
            // =====================================
            CtrlOp::JmpCC(cc) => {
                match cc {
                    ConditionCode::E  => encode_rel32_with_prefix(Some(0x0F), 0x84, target),
                    ConditionCode::NE => encode_rel32_with_prefix(Some(0x0F), 0x85, target),
                    ConditionCode::G  => encode_rel32_with_prefix(Some(0x0F), 0x8C, target),
                    ConditionCode::GE => encode_rel32_with_prefix(Some(0x0F), 0x8D, target),
                    ConditionCode::L  => encode_rel32_with_prefix(Some(0x0F), 0x8E, target),
                    ConditionCode::LE => encode_rel32_with_prefix(Some(0x0F), 0x8F, target),
                    ConditionCode::A  => encode_rel32_with_prefix(Some(0x0F), 0x87, target),
                    ConditionCode::AE => encode_rel32_with_prefix(Some(0x0F), 0x83, target),
                    ConditionCode::B  => encode_rel32_with_prefix(Some(0x0F), 0x82, target),
                    ConditionCode::BE => encode_rel32_with_prefix(Some(0x0F), 0x86, target),
                    ConditionCode::S  => encode_rel32_with_prefix(Some(0x0F), 0x88, target),
                    ConditionCode::NS => encode_rel32_with_prefix(Some(0x0F), 0x89, target),
                    ConditionCode::O  => encode_rel32_with_prefix(Some(0x0F), 0x80, target),
                    ConditionCode::NO => encode_rel32_with_prefix(Some(0x0F), 0x81, target),
                    ConditionCode::P  => encode_rel32_with_prefix(Some(0x0F), 0x8A, target),
                    ConditionCode::NP => encode_rel32_with_prefix(Some(0x0F), 0x8B, target),
                }
            },
            // =====================================
            // loop family (rel8 uniquement)
            // =====================================
            CtrlOp::Loop   => encode_rel8(0xE2, target),
            CtrlOp::Loope  => encode_rel8(0xE1, target),
            CtrlOp::Loopne => encode_rel8(0xE0, target),
        }
    }
}

fn encode_rel32_with_prefix(
    prefix: Option<u8>,
    opcode: u8,
    target: &Operand,
) -> EncodeInformation {
    match target {
        Operand::Sym(sym) => {
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
                    sym: sym.clone(),
                    offset,
                    size: 4,
                    kind: RelocKind::Relative,
                    addend: -4,
                }],
            }
        }

        Operand::Imm(val) => {
            let mut v: Vec<u8> = Vec::new();

            if let Some(p) = prefix { v.push(p); }

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

fn encode_rel8(
    opcode: u8,
    target: &Operand,
) -> EncodeInformation {
    match target {
        Operand::Sym(sym) => {
            let mut v = Vec::with_capacity(2);

            v.push(opcode);

            let offset = v.len();
            v.push(0); // placeholder i8

            EncodeInformation {
                data: v,
                relocations: vec![Relocation {
                    sym: sym.clone(),
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

            let rel = *val as i8; // ⚠️ doit être validé ailleurs
            v.push(rel as u8);

            EncodeInformation {
                data: v,
                relocations: vec![],
            }
        }

        _ => unimplemented!("unsupported target for rel8: {:?}", target),
    }
}
