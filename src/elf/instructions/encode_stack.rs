use super::modrm::*;
use super::enums::*;
use super::encode_alu::emit_size_prefix;


pub(super) fn encode_stack(
    op: StackOp,
    value: &Operand,
    size: Size,
) -> EncodeInformation {
    let mut v = Vec::new();

    match op {
        StackOp::Pushf => {
            emit_size_prefix(&mut v, size);

            v.push(0x9C);

            EncodeInformation {
                data: v,
                ..Default::default()
            }
        }

        StackOp::Popf => {
            emit_size_prefix(&mut v, size);

            v.push(0x9D);

            EncodeInformation {
                data: v,
                ..Default::default()
            }
        }

        // =====================================================
        // PUSH
        // =====================================================
        StackOp::Push => match value {
            // ---------------------------------------------
            // push reg
            // opcode = 50 + reg
            // ---------------------------------------------
            Operand::Reg(reg) => {
                let reg_u8 = *reg as u8;

                emit_size_prefix(&mut v, size);
                emit_rex(&mut v, size, None, Some(reg_u8));

                v.push(0x50 + (reg_u8 & 7));

                EncodeInformation {
                    data: v,
                    ..Default::default()
                }
            }

            // ---------------------------------------------
            // push imm
            // 6A ib
            // 68 iw/id
            // ---------------------------------------------
            Operand::Imm(val) => {
                emit_size_prefix(&mut v, size);

                if *val <= 0x7F {
                    v.push(0x6A);
                    v.push(*val as u8);
                } else {
                    v.push(0x68);

                    match size {
                        Size::U16 => v.extend((*val as u16).to_le_bytes()),
                        _ => v.extend((*val as u32).to_le_bytes()),
                    }
                }

                EncodeInformation {
                    data: v,
                    ..Default::default()
                }
            }

            // ---------------------------------------------
            // push symbol
            // push imm32 reloc
            // ---------------------------------------------
            Operand::Sym(sym) => {
                v.push(0x68);

                let offset = v.len();
                v.extend(0u32.to_le_bytes());

                EncodeInformation {
                    data: v,
                    relocations: vec![
                        Relocation {
                            sym: sym.clone(),
                            offset,
                            size: 4,
                            kind: RelocKind::Absolute,
                            addend: 0,
                        }
                    ],
                }
            }

            // ---------------------------------------------
            // push r/m
            // FF /6
            // ---------------------------------------------
            Operand::MemoryAddress(_) => {
                emit_size_prefix(&mut v, size);
                emit_rex(&mut v, size, None, None);

                v.push(0xFF);

                let reg_field = Operand::Reg(Register::try_from(6).unwrap());

                let base = v.len();
                let modrm = mod_rm_encode(value, &reg_field);

                v.extend(modrm.data);

                let relocations = modrm
                    .relocations
                    .into_iter()
                    .map(|mut r| {
                        r.offset += base;
                        r
                    })
                    .collect();

                EncodeInformation {
                    data: v,
                    relocations,
                }
            }

            _ => unimplemented!("unsupported PUSH operand {:?}", value),
        },

        // =====================================================
        // POP
        // =====================================================
        StackOp::Pop => match value {
            // ---------------------------------------------
            // pop reg
            // 58 + reg
            // ---------------------------------------------
            Operand::Reg(reg) => {
                let reg_u8 = *reg as u8;

                emit_size_prefix(&mut v, size);
                emit_rex(&mut v, size, None, Some(reg_u8));

                v.push(0x58 + (reg_u8 & 7));

                EncodeInformation {
                    data: v,
                    ..Default::default()
                }
            }

            // ---------------------------------------------
            // pop r/m
            // 8F /0
            // ---------------------------------------------
            Operand::MemoryAddress(_) => {
                emit_size_prefix(&mut v, size);
                emit_rex(&mut v, size, None, None);

                v.push(0x8F);

                let reg_field = Operand::Reg(Register::A); // /0

                let base = v.len();
                let modrm = mod_rm_encode(value, &reg_field);

                v.extend(modrm.data);

                let relocations = modrm
                    .relocations
                    .into_iter()
                    .map(|mut r| {
                        r.offset += base;
                        r
                    })
                    .collect();

                EncodeInformation {
                    data: v,
                    relocations,
                }
            }

            _ => unimplemented!("unsupported POP operand {:?}", value),
        },
    }
}