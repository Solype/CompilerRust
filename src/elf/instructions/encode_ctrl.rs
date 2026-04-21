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
            CtrlOp::Je  => encode_rel32_with_prefix(Some(0x0F), 0x84, target),
            CtrlOp::Jne => encode_rel32_with_prefix(Some(0x0F), 0x85, target),

            // =====================================
            // signed comparisons
            // =====================================
            CtrlOp::Jl  => encode_rel32_with_prefix(Some(0x0F), 0x8C, target), // SF != OF
            CtrlOp::Jge => encode_rel32_with_prefix(Some(0x0F), 0x8D, target), // SF == OF
            CtrlOp::Jle => encode_rel32_with_prefix(Some(0x0F), 0x8E, target), // ZF=1 || SF!=OF
            CtrlOp::Jg  => encode_rel32_with_prefix(Some(0x0F), 0x8F, target), // ZF=0 && SF==OF

            // =====================================
            // unsigned comparisons
            // =====================================
            CtrlOp::Ja  => encode_rel32_with_prefix(Some(0x0F), 0x87, target), // CF=0 && ZF=0
            CtrlOp::Jae => encode_rel32_with_prefix(Some(0x0F), 0x83, target), // CF=0
            CtrlOp::Jb  => encode_rel32_with_prefix(Some(0x0F), 0x82, target), // CF=1
            CtrlOp::Jbe => encode_rel32_with_prefix(Some(0x0F), 0x86, target), // CF=1 || ZF=1

            // =====================================
            // sign flag
            // =====================================
            CtrlOp::Js  => encode_rel32_with_prefix(Some(0x0F), 0x88, target),
            CtrlOp::Jns => encode_rel32_with_prefix(Some(0x0F), 0x89, target),

            // =====================================
            // overflow flag
            // =====================================
            CtrlOp::Jo  => encode_rel32_with_prefix(Some(0x0F), 0x80, target),
            CtrlOp::Jno => encode_rel32_with_prefix(Some(0x0F), 0x81, target),

            // =====================================
            // parity flag
            // =====================================
            CtrlOp::Jp  => encode_rel32_with_prefix(Some(0x0F), 0x8A, target),
            CtrlOp::Jnp => encode_rel32_with_prefix(Some(0x0F), 0x8B, target),
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
