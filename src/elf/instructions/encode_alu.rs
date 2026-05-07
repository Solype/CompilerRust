use super::modrm::*;
use super::enums::*;

// ======================================================
// Encoding families
// ======================================================

#[derive(Debug)]
enum BinaryEncoding {
    Alu {
        opcode_rm_r: u8,
        opcode_r_rm: u8,

        // imm full size
        opcode_imm: u8,

        // imm8 sign-extended
        opcode_imm8: Option<u8>,

        modrm_ext: u8,
    },

    Mov {
        opcode_rm_r: u8,
        opcode_r_rm: u8,
        opcode_imm: u8,
    },

    MovExtend {
        prefix: u8,
        opcode: u8,
    },

    Xchg {
        opcode: u8,
    },
}

impl BinOp {
    fn encoding(self, size: Size) -> BinaryEncoding {
        match self {

            // --------------------------------------------------
            // MOV
            // --------------------------------------------------
            BinOp::Mov => BinaryEncoding::Mov {
                opcode_rm_r: if size == Size::U8 { 0x88 } else { 0x89 },
                opcode_r_rm: if size == Size::U8 { 0x8A } else { 0x8B },
                opcode_imm:  if size == Size::U8 { 0xC6 } else { 0xC7 },
            },

            // --------------------------------------------------
            // MOVZX / MOVSX
            // --------------------------------------------------
            BinOp::Movzx => BinaryEncoding::MovExtend {
                prefix: 0x0F,
                opcode: if size == Size::U8 { 0xB6 } else { 0xB7 },
            },

            BinOp::Movsx => BinaryEncoding::MovExtend {
                prefix: 0x0F,
                opcode: if size == Size::U8 { 0xBE } else { 0xBF },
            },

            // --------------------------------------------------
            // XCHG
            // --------------------------------------------------
            BinOp::Xchg => BinaryEncoding::Xchg {
                opcode: if size == Size::U8 { 0x86 } else { 0x87 },
            },

            // --------------------------------------------------
            // ALU
            // --------------------------------------------------
            BinOp::Add => BinaryEncoding::Alu {
                opcode_rm_r: if size == Size::U8 { 0x00 } else { 0x01 },
                opcode_r_rm: if size == Size::U8 { 0x02 } else { 0x03 },

                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },

                modrm_ext: 0,
            },

            BinOp::Adc => BinaryEncoding::Alu {
                opcode_rm_r: if size == Size::U8 { 0x10 } else { 0x11 },
                opcode_r_rm: if size == Size::U8 { 0x12 } else { 0x13 },

                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },

                modrm_ext: 2,
            },

            BinOp::Sbb => BinaryEncoding::Alu {
                opcode_rm_r: if size == Size::U8 { 0x18 } else { 0x19 },
                opcode_r_rm: if size == Size::U8 { 0x1A } else { 0x1B },

                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },

                modrm_ext: 3,
            },

            BinOp::Or => BinaryEncoding::Alu {
                opcode_rm_r: if size == Size::U8 { 0x08 } else { 0x09 },
                opcode_r_rm: if size == Size::U8 { 0x0A } else { 0x0B },

                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },

                modrm_ext: 1,
            },

            BinOp::And => BinaryEncoding::Alu {
                opcode_rm_r: if size == Size::U8 { 0x20 } else { 0x21 },
                opcode_r_rm: if size == Size::U8 { 0x22 } else { 0x23 },

                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },

                modrm_ext: 4,
            },

            BinOp::Sub => BinaryEncoding::Alu {
                opcode_rm_r: if size == Size::U8 { 0x28 } else { 0x29 },
                opcode_r_rm: if size == Size::U8 { 0x2A } else { 0x2B },

                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },

                modrm_ext: 5,
            },

            BinOp::Xor => BinaryEncoding::Alu {
                opcode_rm_r: if size == Size::U8 { 0x30 } else { 0x31 },
                opcode_r_rm: if size == Size::U8 { 0x32 } else { 0x33 },

                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },

                modrm_ext: 6,
            },

            BinOp::Cmp => BinaryEncoding::Alu {
                opcode_rm_r: if size == Size::U8 { 0x38 } else { 0x39 },
                opcode_r_rm: if size == Size::U8 { 0x3A } else { 0x3B },

                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                opcode_imm8: if size == Size::U8 { None } else { Some(0x83) },

                modrm_ext: 7,
            },

            BinOp::Test => BinaryEncoding::Alu {
                opcode_rm_r: if size == Size::U8 { 0x84 } else { 0x85 },
                opcode_r_rm: if size == Size::U8 { 0x84 } else { 0x85 },

                opcode_imm:  if size == Size::U8 { 0xF6 } else { 0xF7 },
                opcode_imm8: None,

                modrm_ext: 0,
            },
        }
    }
}

pub fn emit_size_prefix(v: &mut Vec<u8>, size: Size) {
    if let Size::U16 = size {
        v.push(0x66);
    }
}

fn emit_imm(v: &mut Vec<u8>, val: i64, size: Size) {
    match size {
        Size::U8 => v.push(val as u8),
        Size::U16 => v.extend(&(val as u16).to_le_bytes()),
        Size::U32 => v.extend(&(val as u32).to_le_bytes()),
        Size::U64 => v.extend(&(val as u64).to_le_bytes()),
    }
}

fn encode_reg_imm(
    reg: Register,
    val: i64,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {
    let mut v = Vec::new();
    let reg_u8 = reg as u8;

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, Some(reg_u8));

    match *enc {
        BinaryEncoding::Mov { .. } => {
            let base = match size {
                Size::U8 => 0xB0,
                _ => 0xB8,
            };
            v.push(base + (reg_u8 & 7));
        }

        BinaryEncoding::Alu { opcode_imm, modrm_ext, .. } => {
            v.push(opcode_imm);
            let modrm = 0b11_000_000 | ((modrm_ext & 7) << 3) | (reg_u8 & 7);
            v.push(modrm);
        }

        _ => unimplemented!("encode reg <- imm not implemented for{:?}", *enc),
    }

    emit_imm(&mut v, val, size);

    EncodeInformation {
        data: v,
        ..Default::default()
    }
}

fn encode_reg_sym(
    reg: Register,
    sym: &str,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {
    let mut v = Vec::new();
    let reg_u8 = reg as u8;

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, Some(reg_u8));

    match enc {
        BinaryEncoding::Mov { .. } => {
            let base = match size {
                Size::U8 => 0xB0,
                _ => 0xB8,
            };
            v.push(base + (reg_u8 & 7));
        }

        BinaryEncoding::Alu { opcode_imm, modrm_ext , .. } => {
            v.push(*opcode_imm);
            let modrm = 0b11_000_000 | ((modrm_ext & 7) << 3) | (reg_u8 & 7);
            v.push(modrm);
        }
        _ => unimplemented!()
    }

    let offset = v.len();
    emit_imm(&mut v, 0, size);

    EncodeInformation {
        data: v,
        relocations: vec![Relocation {
            sym: sym.to_string(),
            offset,
            size: size as u8,
            kind: RelocKind::Absolute,
            addend: 0,
        }],
    }
}

fn encode_mem_imm(
    dst: &Operand,
    val: i64,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {

    let (opcode_imm, opcode_imm8, modrm_ext) = match *enc {
        BinaryEncoding::Mov { opcode_imm, .. } => {
            (opcode_imm, None, 0)
        }

        BinaryEncoding::Alu {
            opcode_imm,
            opcode_imm8,
            modrm_ext,
            ..
        } => {
            (opcode_imm, opcode_imm8, modrm_ext)
        }

        _ => unimplemented!("mem, imm unsupported for this instruction"),
    };

    let fits_i8 = val >= -128 && val <= 127;

    let use_imm8 = fits_i8 && opcode_imm8.is_some() && size != Size::U8;
    let opcode = if use_imm8 { opcode_imm8.unwrap() } else { opcode_imm };

    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, None);

    v.push(opcode);

    let reg_field = Operand::Reg(Register::try_from(modrm_ext).unwrap());

    let modrm_info = mod_rm_encode(dst, &reg_field);

    let base = v.len();

    v.extend(modrm_info.data);

    let after = v.len();

    if use_imm8 {
        v.push(val as i8 as u8);
    } else {
        emit_imm(&mut v, val, size);
    }

    let mut relocations = modrm_info.relocations;

    for r in &mut relocations {
        r.offset += base;
        r.addend -= (v.len() - after) as i32;
    }

    EncodeInformation {
        data: v,
        relocations,
    }
}

fn encode_reg_mem(
    dst: &Operand,
    src: &Operand,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {
    let (is_reg_dst, reg_op, rm_op) = match (dst, src) {
        (Operand::Reg(_), _) => (true, dst, src),
        (_, Operand::Reg(_)) => (false, src, dst),
        _ => unreachable!(),
    };

    let reg_u8 = match reg_op {
        Operand::Reg(r) => *r as u8,
        _ => unreachable!(),
    };

    let rm_u8 = match rm_op {
        Operand::Reg(r) => *r as u8,
        _ => 0, // memory operand
    };

    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, Some(reg_u8), Some(rm_u8));

    let opcode = match *enc {
        BinaryEncoding::Mov { opcode_rm_r, opcode_r_rm, .. }
        | BinaryEncoding::Alu { opcode_rm_r, opcode_r_rm, .. } => {
            if is_reg_dst {  opcode_r_rm } else { opcode_rm_r }
        }

        BinaryEncoding::Xchg { opcode } => opcode,

        BinaryEncoding::MovExtend { prefix, opcode } => {
            v.push(prefix);
            opcode
        }
    };

    v.push(opcode);

    let base = v.len();

    let modrm = mod_rm_encode(rm_op, reg_op);

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

pub(super) fn encode_binary(
    op: BinOp,
    dst: &Operand,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let enc = op.encoding(size);

    match (dst, src) {

        (Operand::Reg(reg), Operand::Imm(val)) => encode_reg_imm(*reg, *val, size, &enc),
        (Operand::Reg(reg), Operand::Sym(sym)) => encode_reg_sym(*reg, sym, size, &enc),
        (Operand::MemoryAddress(_), Operand::Imm(val)) => encode_mem_imm(dst, *val, size, &enc),

        (
            Operand::Reg(_) | Operand::MemoryAddress(_),
            Operand::Reg(_) | Operand::MemoryAddress(_),
        ) => encode_reg_mem(dst, src, size, &enc),

        _ => unimplemented!("unsupported operands: {:?}, {:?}", dst, src),
    }
}
// Von 18 bis 6
