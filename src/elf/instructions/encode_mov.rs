use super::modrm::*;
use super::enums::*;

pub(super) fn encode_move(op1: &Operand, op2: &Operand, size: Size) -> EncodeInformation {
    fn emit_rex(v: &mut Vec<u8>, size: Size, reg: Option<u8>, rm: Option<u8>) {
        let mut rex = 0x40;

        if let Size::U64 = size {
            rex |= 1 << 3; // W
        }

        if let Some(r) = reg {
            if r >= 8 {
                rex |= 1 << 2; // R
            }
        }

        if let Some(b) = rm {
            if b >= 8 {
                rex |= 1; // B
            }
        }

        if rex != 0x40 {
            v.push(rex);
        }
    }

    match (op1, op2) {
        // -----------------------------
        // mov reg, imm
        // -----------------------------
        (Operand::Reg(reg), Operand::Imm(val)) => {
            let mut v = Vec::new();
            let reg_u8 = *reg as u8;

            emit_rex(&mut v, size, None, Some(reg_u8));

            v.push(0xB8 + (reg_u8 & 7));

            match size {
                Size::U32 => v.extend(&(*val as u32).to_le_bytes()),
                Size::U64 => v.extend(&(*val as u64).to_le_bytes()),
            }

            EncodeInformation {
                data: v,
                ..Default::default()
            }
        }

        // -----------------------------
        // mov reg, symbol
        // -----------------------------
        (Operand::Reg(reg), Operand::Sym(sym)) => {
            let mut v = Vec::new();
            let reg_u8 = *reg as u8;

            emit_rex(&mut v, size, None, Some(reg_u8));
            v.push(0xB8 + (reg_u8 & 7));
            match size {
                Size::U32 => v.extend(&0u32.to_le_bytes()),
                Size::U64 => v.extend(&0u64.to_le_bytes()),
            }
            let length = v.len();

            EncodeInformation {
                data: v,
                relocations: vec![Relocation {
                    sym: sym.clone(),
                    offset: (length - (size as u8 as usize)),
                    size: match size { Size::U32 => 4, Size::U64 => 8 },
                    kind: RelocKind::Absolute,
                    addend: 0,
                }],
            }
        }

        // -----------------------------
        // mov reg/mem, reg/mem
        // -----------------------------
        (
            Operand::Reg(_) | Operand::MemoryAddress(_),
            Operand::Reg(_) | Operand::MemoryAddress(_),
        ) => {
            let (opcode, reg_op, rm_op) = match (op1, op2) {
                (Operand::Reg(_), _) => (0x8B, op1, op2),
                (_, Operand::Reg(_)) => (0x89, op2, op1),
                _ => panic!("x86 cannot move memory to memory directly"),
            };

            let reg_u8 = match reg_op {
                Operand::Reg(r) => *r as u8,
                _ => 0,
            };

            let rm_u8 = match rm_op {
                Operand::Reg(r) => *r as u8,
                _ => 0,
            };

            let mut v = Vec::new();

            emit_rex(&mut v, size, Some(reg_u8), Some(rm_u8));

            v.push(opcode);

            let base_offset = v.len();

            let modrm_info = mod_rm_encode(rm_op, reg_op);

            v.extend(modrm_info.data);

            let relocations = modrm_info
                .relocations
                .into_iter()
                .map(|mut reloc| {
                    reloc.offset += base_offset;
                    reloc.kind = RelocKind::Absolute;
                    reloc.addend = 0;
                    reloc
                })
                .collect();

            EncodeInformation {
                data: v,
                relocations,
            }
        }

        _ => unimplemented!(),
    }
}