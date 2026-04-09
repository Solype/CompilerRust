use super::enums::*;

fn encode_modrm_field_reg(reg_op: &Operand) -> u8 {
    match reg_op {
        Operand::Reg(r) => *r as u8,
        Operand::NoOperand => 0, // si instruction utilise /digit au lieu de reg
        _ => panic!("ModRM register field must be a register or noop!"),
    }
}

fn encode_sib(scale: Scale, index: Option<Register>, base: Option<Register>) -> u8 {
    let scale_bits = match scale {
        Scale::One => 0b00,
        Scale::Two => 0b01,
        Scale::Four => 0b10,
        Scale::Eight => 0b11,
    };

    let index_bits = match index {
        Some(r) if r != Register::Esp => r as u8,
        _ => 0b100, // no index
    };

    let base_bits = match base {
        Some(r) => r as u8,
        None => 0b101, // disp32
    };

    (scale_bits << 6) | (index_bits << 3) | base_bits
}

fn encode_disp(disp: i32) -> (u8, Vec<u8>) {
    if disp == 0 {
        (0b00, vec![])
    } else if (-128..=127).contains(&disp) {
        (0b01, vec![disp as u8])
    } else {
        (0b10, disp.to_le_bytes().to_vec())
    }
}

pub(super) fn mod_rm_encode(op1: &Operand, op2: &Operand) -> EncodeInformation {
    let mut data = vec![0u8];
    let mut relocs: Vec<Relocation> = vec![];

    let reg_field = encode_modrm_field_reg(op2);

    match op1 {
        // =========================
        // REG → mod = 11
        // =========================
        Operand::Reg(rm) => {
            data[0] = (0b11 << 6) | (*rm as u8);
        }

        // =========================
        // MEMORY
        // =========================
        Operand::MemoryAddress(mem) => {
            match mem {
                // -------------------------
                // [disp32]
                // -------------------------
                MemAddress::Direct { disp } => {
                    data[0] = (0b00 << 6) | 0b101;
                    data.extend(&(*disp as u32).to_le_bytes());
                }

                // -------------------------
                // [symbol]
                // -------------------------
                // MemAddress::Direct { disp: _ } => {} // déjà géré au-dessus

                // -------------------------
                // [base]
                // -------------------------
                MemAddress::Base { base } => {
                    if *base == Register::Esp {
                        // SIB obligatoire
                        data[0] = (0b00 << 6) | 0b100;
                        data.push(encode_sib(Scale::One, None, Some(*base)));
                    } else if *base == Register::Ebp {
                        // EBP → disp8 obligatoire
                        data[0] = (0b01 << 6) | (*base as u8);
                        data.push(0);
                    } else {
                        data[0] = (0b00 << 6) | (*base as u8);
                    }
                }

                // -------------------------
                // [base + disp]
                // -------------------------
                MemAddress::BaseDisp { base, disp } => {
                    let (mod_bits, disp_bytes) = encode_disp(*disp);

                    if *base == Register::Esp {
                        data[0] = (mod_bits << 6) | 0b100;
                        data.push(encode_sib(Scale::One, None, Some(*base)));
                    } else {
                        data[0] = (mod_bits << 6) | (*base as u8);
                    }

                    data.extend(disp_bytes);
                }

                // -------------------------
                // [index*scale + disp]
                // -------------------------
                MemAddress::IndexScaleDisp { index, scale, disp } => {
                    let (mod_bits, disp_bytes) = encode_disp(*disp);

                    data[0] = (mod_bits << 6) | 0b100;

                    data.push(encode_sib(*scale, Some(*index), None));

                    data.extend(disp_bytes);
                }

                // -------------------------
                // [base + index]
                // -------------------------
                MemAddress::BaseIndex { base, index } => {
                    data[0] = (0b00 << 6) | 0b100;

                    data.push(encode_sib(Scale::One, Some(*index), Some(*base)));
                }

                // -------------------------
                // [base + index * scale]
                // -------------------------
                MemAddress::BaseIndexScale { base, index, scale } => {
                    data[0] = (0b00 << 6) | 0b100;

                    data.push(encode_sib(*scale, Some(*index), Some(*base)));
                }

                // -------------------------
                // [base + index * scale + disp]
                // -------------------------
                MemAddress::BaseIndexScaleDisp { base, index, scale, disp } => {
                    let (mod_bits, disp_bytes) = encode_disp(*disp);

                    data[0] = (mod_bits << 6) | 0b100;

                    data.push(encode_sib(*scale, Some(*index), Some(*base)));

                    data.extend(disp_bytes);
                }
            }
        }

        // =========================
        // SYMBOL (mov reg, sym)
        // =========================
        Operand::Sym(sym) => {
            data[0] = (0b00 << 6) | 0b101;

            relocs.push(Relocation {
                sym: sym.clone(),
                offset: 1,
                size: 4,
                ..Default::default()
            });

            data.extend(&0u32.to_le_bytes());
        }

        _ => panic!("Invalid operand for ModRM"),
    }

    // Inject reg field
    data[0] = (data[0] & 0b11000111) | ((reg_field & 0b111) << 3);

    EncodeInformation {
        data,
        relocations: relocs,
    }
}
