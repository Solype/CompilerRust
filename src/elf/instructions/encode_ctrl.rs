use super::enums::*;

impl CtrlOp {
    pub(super) fn encode(self, target: &Operand) -> EncodeInformation {
        match self {
            // =========================
            // REL32 (jmp / call)
            // =========================
            CtrlOp::Jmp => encode_rel32_with_prefix(None, 0xE9, target),
            CtrlOp::Call => encode_rel32_with_prefix(None, 0xE8, target),

            // =========================
            // RET (no operand)
            // =========================
            CtrlOp::Ret => EncodeInformation {
                data: vec![0xC3],
                relocations: vec![],
            },

            // =========================
            // FUTURE: conditional jumps
            // =========================
            CtrlOp::Je => encode_rel32_with_prefix(Some(0x0F), 0x84, target),
            CtrlOp::Jne => encode_rel32_with_prefix(Some(0x0F), 0x85, target),
            CtrlOp::Jg => encode_rel32_with_prefix(Some(0x0F), 0x8F, target),
            CtrlOp::Jl => encode_rel32_with_prefix(Some(0x0F), 0x8C, target),
            CtrlOp::Jge => encode_rel32_with_prefix(Some(0x0F), 0x8D, target),
            CtrlOp::Jle => encode_rel32_with_prefix(Some(0x0F), 0x8E, target),
            _ => unimplemented!()
        }
    }
}

pub(super) fn encode_rel32_with_prefix(
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
