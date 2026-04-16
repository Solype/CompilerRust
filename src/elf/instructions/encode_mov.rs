use super::modrm::*;
use super::enums::*;

pub(super) fn encode_move(op1: &Operand, op2: &Operand, size: Size) -> EncodeInformation {
    match (op1, op2) {
        // -----------------------------
        // mov reg, imm
        // -----------------------------
        (Operand::Reg(reg), Operand::Imm(val)) => {
            let mut v = Vec::new();
            let reg_u8 = *reg as u8;

            if let Size::U16 = size { v.push(0x66); }
            
            emit_rex(&mut v, size, None, Some(reg_u8));

            match size {
                Size::U8 => v.push(0xB0 + (reg_u8 & 7)),
                _ => v.push(0xB8 + (reg_u8 & 7)),
            }

            match size {
                Size::U8 => v.extend(&(*val as u8).to_le_bytes()),
                Size::U16 => v.extend(&(*val as u16).to_le_bytes()),
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

            if let Size::U16 = size {
                v.push(0x66);
            }

            emit_rex(&mut v, size, None, Some(reg_u8));
            match size {
                Size::U8 => v.push(0xB0 + (reg_u8 & 7)),
                _ => v.push(0xB8 + (reg_u8 & 7)),
            }
            let imm_offset = v.len();

            match size {
                Size::U8 => v.push(0),
                Size::U16 => v.extend(&0u16.to_le_bytes()),
                Size::U32 => v.extend(&0u32.to_le_bytes()),
                Size::U64 => v.extend(&0u64.to_le_bytes()),
            }

            EncodeInformation {
                data: v,
                relocations: vec![Relocation {
                    sym: sym.clone(),
                    offset: imm_offset,
                    size: size as u8,
                    kind: RelocKind::Absolute,
                    addend: 0,
                }],
            }
        }

        (Operand::MemoryAddress(_), Operand::Imm(val)) => {
            let mut v = Vec::new();

            if let Size::U16 = size { v.push(0x66); }

            let opcode = match size { Size::U8 => 0xC6, _ => 0xC7, };

            emit_rex(&mut v, size, None, None);

            v.push(opcode);

            let modrm_info = mod_rm_encode(
                op1, // rm
                &Operand::NoOperand    // reg = 0 (/0)
            );

            let base_offset = v.len();
            v.extend(modrm_info.data);

            let offset_with_modrm = v.len();

            match size {
                Size::U8 => v.push(*val as u8),
                Size::U16 => v.extend(&(*val as u16).to_le_bytes()),
                Size::U32 => v.extend(&(*val as u32).to_le_bytes()),
                Size::U64 => unimplemented!(),
            }

            let mut relocations = modrm_info.relocations;

            for reloc in &mut relocations {
                reloc.offset += base_offset;
                reloc.addend -= (v.len() - offset_with_modrm) as i32
            }

            EncodeInformation {
                data: v,
                relocations,
            }
        }

        // -----------------------------
        // mov reg/mem, reg/mem
        // -----------------------------
        (
            Operand::Reg(_) | Operand::MemoryAddress(_),
            Operand::Reg(_) | Operand::MemoryAddress(_),
        ) => {
            let (is_reg_dest, reg_op, rm_op) = match (op1, op2) {
                (Operand::Reg(_), _) => (true, op1, op2),
                (_, Operand::Reg(_)) => (false, op2, op1),
                _ => panic!("x86 cannot move memory to memory directly"),
            };

            let opcode = match size {
                Size::U8 => { if is_reg_dest { 0x8A } else { 0x88 } }
                _        => { if is_reg_dest { 0x8B } else { 0x89 } }
            };

            let reg_u8 = match reg_op { Operand::Reg(r) => *r as u8, _ => 0, };

            let rm_u8 = match rm_op { Operand::Reg(r) => *r as u8, _ => 0, };

            let mut v = Vec::new();

            if let Size::U16 = size { v.push(0x66); }

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