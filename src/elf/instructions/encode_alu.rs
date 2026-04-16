use super::modrm::*;
use super::enums::*;

impl BinOp {
    pub fn encoding(self, size: Size) -> BinaryEncoding {
        match self {
            BinOp::Mov => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x88 } else { 0x89 },
                opcode_r_rm: if size == Size::U8 { 0x8A } else { 0x8B },
                opcode_imm:  if size == Size::U8 { 0xC6 } else { 0xC7 },
                modrm_ext: 0,
            },

            BinOp::Add => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x00 } else { 0x01 },
                opcode_r_rm: if size == Size::U8 { 0x02 } else { 0x03 },
                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                modrm_ext: 0,
            },

            _ => unimplemented!(),
        }
    }
}


pub(super) fn encode_binary(
    op: BinOp,
    dst: &Operand,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let enc = op.encoding(size);

    match (dst, src) {

        // =============================
        // reg, imm
        // =============================
        (Operand::Reg(reg), Operand::Imm(val)) => {
            let mut v = Vec::new();
            let reg_u8 = *reg as u8;

            if let Size::U16 = size {
                v.push(0x66);
            }

            emit_rex(&mut v, size, None, Some(reg_u8));

            match op {
                BinOp::Mov => {
                    // cas spécial x86
                    match size {
                        Size::U8 => v.push(0xB0 + (reg_u8 & 7)),
                        _ => v.push(0xB8 + (reg_u8 & 7)),
                    }
                }
                _ => {
                    v.push(enc.opcode_imm);

                    let modrm = 0b11_000_000
                        | ((enc.modrm_ext & 7) << 3)
                        | (reg_u8 & 7);

                    v.push(modrm);
                }
            }

            match size {
                Size::U8 => v.push(*val as u8),
                Size::U16 => v.extend(&(*val as u16).to_le_bytes()),
                Size::U32 => v.extend(&(*val as u32).to_le_bytes()),
                Size::U64 => v.extend(&(*val as u64).to_le_bytes()),
            }

            EncodeInformation { data: v, ..Default::default() }
        }

        // =============================
        // reg, symbol
        // =============================
        (Operand::Reg(reg), Operand::Sym(sym)) => {
            let mut v = Vec::new();
            let reg_u8 = *reg as u8;

            if let Size::U16 = size {
                v.push(0x66);
            }

            emit_rex(&mut v, size, None, Some(reg_u8));

            match op {
                BinOp::Mov => {
                    match size {
                        Size::U8 => v.push(0xB0 + (reg_u8 & 7)),
                        _ => v.push(0xB8 + (reg_u8 & 7)),
                    }
                }
                _ => {
                    v.push(enc.opcode_imm);

                    let modrm = 0b11_000_000
                        | ((enc.modrm_ext & 7) << 3)
                        | (reg_u8 & 7);

                    v.push(modrm);
                }
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

        // =============================
        // mem, imm
        // =============================
        (Operand::MemoryAddress(_), Operand::Imm(val)) => {
            let mut v = Vec::new();

            if let Size::U16 = size {
                v.push(0x66);
            }

            emit_rex(&mut v, size, None, None);

            v.push(enc.opcode_imm);

            let modrm_info = mod_rm_encode(dst, &Operand::NoOperand);

            let base_offset = v.len();
            v.extend(modrm_info.data);

            let offset_after_modrm = v.len();

            match size {
                Size::U8 => v.push(*val as u8),
                Size::U16 => v.extend(&(*val as u16).to_le_bytes()),
                Size::U32 => v.extend(&(*val as u32).to_le_bytes()),
                Size::U64 => unimplemented!(),
            }

            let mut relocations = modrm_info.relocations;

            for reloc in &mut relocations {
                reloc.offset += base_offset;
                reloc.addend -= (v.len() - offset_after_modrm) as i32;
            }

            EncodeInformation { data: v, relocations }
        }

        // =============================
        // reg/mem, reg/mem
        // =============================
        (
            Operand::Reg(_) | Operand::MemoryAddress(_),
            Operand::Reg(_) | Operand::MemoryAddress(_),
        ) => {
            let (is_reg_dest, reg_op, rm_op) = match (dst, src) {
                (Operand::Reg(_), _) => (true, dst, src),
                (_, Operand::Reg(_)) => (false, src, dst),
                _ => panic!("x86 cannot operate memory to memory"),
            };

            let reg_u8 = match reg_op {
                Operand::Reg(r) => *r as u8,
                _ => unreachable!(),
            };

            let rm_u8 = match rm_op {
                Operand::Reg(r) => *r as u8,
                _ => 0,
            };

            let opcode = if is_reg_dest {
                enc.opcode_r_rm
            } else {
                enc.opcode_rm_r
            };

            let mut v = Vec::new();

            if let Size::U16 = size {
                v.push(0x66);
            }

            emit_rex(&mut v, size, Some(reg_u8), Some(rm_u8));

            v.push(opcode);

            let base_offset = v.len();
            let modrm_info = mod_rm_encode(rm_op, reg_op);

            v.extend(modrm_info.data);

            let relocations = modrm_info
                .relocations.into_iter().map(|mut r| { r.offset += base_offset; r }).collect();

            EncodeInformation { data: v, relocations }
        }

        _ => unimplemented!("unsupported operands: {:?}, {:?}", dst, src),
    }
}
