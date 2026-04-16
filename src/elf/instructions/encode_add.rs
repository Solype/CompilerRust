use super::modrm::*;
use super::enums::*;

pub(super) fn encode_add(dst: &Operand, src: &Operand, size: Size) -> EncodeInformation {
    match (dst, src) {
        // add reg, reg
        (Operand::Reg(dst_reg), Operand::Reg(src_reg)) => {
            let mut v = Vec::new();
            if let Size::U16 = size { v.push(0x66); } // Préfixe pour 16 bits
            emit_rex(&mut v, size, Some(*src_reg as u8), Some(*dst_reg as u8));
            v.push(0x01); // Opcode pour add reg, reg (16/32/64 bits)
            let modrm = mod_rm_encode(dst, src);
            v.extend(modrm.data);
            EncodeInformation { data: v, relocations: modrm.relocations }
        },

        // add reg, imm (valeur 8 bits signée)
        (Operand::Reg(reg), Operand::Imm(val)) if *val <= i8::MAX as usize && *val >= i8::MIN as usize => {
            let mut v = Vec::new();
            if let Size::U16 = size { v.push(0x66); }
            emit_rex(&mut v, size, None, Some(*reg as u8));
            v.push(0x83); // Opcode pour add reg, imm8
            let modrm = mod_rm_encode(dst, &Operand::NoOperand);
            v.extend(modrm.data);
            v.push(*val as u8); // Valeur immédiate 8 bits
            EncodeInformation { data: v, relocations: modrm.relocations }
        },

        // add reg, imm (valeur 16/32/64 bits)
        (Operand::Reg(reg), Operand::Imm(val)) => {
            let mut v = Vec::new();
            if let Size::U16 = size { v.push(0x66); }
            emit_rex(&mut v, size, None, Some(*reg as u8));
            v.push(0x05); // Opcode pour add reg, imm16/32/64
            match size {
                Size::U16 => v.extend(&(*val as u16).to_le_bytes()),
                Size::U32 => v.extend(&(*val as u32).to_le_bytes()),
                Size::U64 => v.extend(&(*val as u32).to_le_bytes()), // Sign-extension en mode 64 bits
                _ => panic!("Unsupported size for add reg, imm"),
            }
            EncodeInformation { data: v, ..Default::default() }
        },

        // add [mem], reg
        (Operand::MemoryAddress(_), Operand::Reg(reg)) => {
            let mut v = Vec::new();
            if let Size::U16 = size { v.push(0x66); }
            emit_rex(&mut v, size, Some(*reg as u8), None);
            v.push(0x01); // Opcode pour add [mem], reg
            let modrm_info = mod_rm_encode(dst, src);
            let base_offset = v.len();
            v.extend(modrm_info.data);
            let mut relocations = modrm_info.relocations;
            for reloc in &mut relocations {
                reloc.offset += base_offset;
            }
            EncodeInformation { data: v, relocations }
        },

        // add reg, [mem]
        (Operand::Reg(reg), Operand::MemoryAddress(_)) => {
            let mut v = Vec::new();
            if let Size::U16 = size { v.push(0x66); }
            emit_rex(&mut v, size, Some(*reg as u8), None);
            v.push(0x03); // Opcode pour add reg, [mem]
            let modrm_info = mod_rm_encode(src, dst);
            let base_offset = v.len();
            v.extend(modrm_info.data);
            let mut relocations = modrm_info.relocations;
            for reloc in &mut relocations {
                reloc.offset += base_offset;
            }
            EncodeInformation { data: v, relocations }
        },

        // add [mem], imm (valeur 8 bits signée)
        (Operand::MemoryAddress(_), Operand::Imm(val)) if *val <= i8::MAX as usize && *val >= i8::MIN as usize => {
            let mut v = Vec::new();
            if let Size::U16 = size { v.push(0x66); }
            emit_rex(&mut v, size, None, None);
            v.push(0x83); // Opcode pour add [mem], imm8
            let modrm_info = mod_rm_encode(dst, &Operand::NoOperand);
            let base_offset = v.len();
            v.extend(modrm_info.data);
            v.push(*val as u8); // Valeur immédiate 8 bits
            let mut relocations = modrm_info.relocations;
            for reloc in &mut relocations {
                reloc.offset += base_offset;
            }
            EncodeInformation { data: v, relocations }
        },

        // add [mem], imm (valeur 16/32 bits)
        (Operand::MemoryAddress(_), Operand::Imm(val)) => {
            let mut v = Vec::new();
            if let Size::U16 = size { v.push(0x66); }
            emit_rex(&mut v, size, None, None);
            v.push(0x81); // Opcode pour add [mem], imm16/32
            let modrm_info = mod_rm_encode(dst, &Operand::NoOperand);
            let base_offset = v.len();
            v.extend(modrm_info.data);
            match size {
                Size::U16 => v.extend(&(*val as u16).to_le_bytes()),
                Size::U32 => v.extend(&(*val as u32).to_le_bytes()),
                Size::U64 => v.extend(&(*val as u32).to_le_bytes()), // Sign-extension en mode 64 bits
                _ => panic!("Unsupported size for add [mem], imm"),
            }
            let mut relocations = modrm_info.relocations;
            for reloc in &mut relocations {
                reloc.offset += base_offset;
            }
            EncodeInformation { data: v, relocations }
        },

        _ => unimplemented!(),
    }
}
