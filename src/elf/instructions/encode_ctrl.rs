use super::enums::*;

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
            }
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

        // 🔥 bonus très utile
        Operand::Imm(val) => {
            let mut v = Vec::new();

            if let Some(p) = prefix {
                v.push(p);
            }

            v.push(opcode);

            let rel = *val as i32; // attention : doit être validé ailleurs
            v.extend(&rel.to_le_bytes());

            EncodeInformation {
                data: v,
                relocations: vec![],
            }
        }

        _ => unimplemented!("unsupported target for rel32: {:?}", target),
    }
}
