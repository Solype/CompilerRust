use super::modrm::*;
use super::enums::*;

struct BinaryEncoding {
    pub opcode_rm_r : u8,
    pub opcode_r_rm : u8,
    pub opcode_imm : u8,
    pub modrm_ext : u8,
}

impl BinOp {
    fn encoding(self, size: Size) -> BinaryEncoding {
        match self {

            // =========================
            // MOV (déjà OK)
            // =========================
            BinOp::Mov => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x88 } else { 0x89 },
                opcode_r_rm: if size == Size::U8 { 0x8A } else { 0x8B },
                opcode_imm:  if size == Size::U8 { 0xC6 } else { 0xC7 },
                modrm_ext: 0,
            },

            // =========================
            // ADD
            // =========================
            BinOp::Add => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x00 } else { 0x01 },
                opcode_r_rm: if size == Size::U8 { 0x02 } else { 0x03 },
                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                modrm_ext: 0, // /0
            },

            // =========================
            // SUB
            // =========================
            BinOp::Sub => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x28 } else { 0x29 },
                opcode_r_rm: if size == Size::U8 { 0x2A } else { 0x2B },
                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                modrm_ext: 5, // /5
            },

            // =========================
            // CMP
            // =========================
            BinOp::Cmp => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x38 } else { 0x39 },
                opcode_r_rm: if size == Size::U8 { 0x3A } else { 0x3B },
                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                modrm_ext: 7, // /7
            },

            // =========================
            // AND
            // =========================
            BinOp::And => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x20 } else { 0x21 },
                opcode_r_rm: if size == Size::U8 { 0x22 } else { 0x23 },
                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                modrm_ext: 4, // /4
            },

            // =========================
            // OR
            // =========================
            BinOp::Or => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x08 } else { 0x09 },
                opcode_r_rm: if size == Size::U8 { 0x0A } else { 0x0B },
                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                modrm_ext: 1, // /1
            },

            // =========================
            // XOR
            // =========================
            BinOp::Xor => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x30 } else { 0x31 },
                opcode_r_rm: if size == Size::U8 { 0x32 } else { 0x33 },
                opcode_imm:  if size == Size::U8 { 0x80 } else { 0x81 },
                modrm_ext: 6, // /6
            },

            // =========================
            // TEST (pas de write)
            // =========================
            BinOp::Test => BinaryEncoding {
                opcode_rm_r: if size == Size::U8 { 0x84 } else { 0x85 },
                opcode_r_rm: if size == Size::U8 { 0x84 } else { 0x85 }, // symétrique
                opcode_imm:  if size == Size::U8 { 0xF6 } else { 0xF7 },
                modrm_ext: 0, // /0
            },

            _ => unimplemented!("BinOp {:?} not implemented", self),
        }
    }
}

fn emit_size_prefix(v: &mut Vec<u8>, size: Size) {
    if let Size::U16 = size {
        v.push(0x66);
    }
}

fn emit_imm(v: &mut Vec<u8>, val: usize, size: Size) {
    match size {
        Size::U8 => v.push(val as u8),
        Size::U16 => v.extend(&(val as u16).to_le_bytes()),
        Size::U32 => v.extend(&(val as u32).to_le_bytes()),
        Size::U64 => v.extend(&(val as u64).to_le_bytes()),
    }
}

fn encode_reg_imm(
    op: BinOp,
    reg: Register,
    val: usize,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {
    let mut v = Vec::new();
    let reg_u8 = reg as u8;

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, Some(reg_u8));

    match op {
        BinOp::Mov => {
            let base = match size {
                Size::U8 => 0xB0,
                _ => 0xB8,
            };
            v.push(base + (reg_u8 & 7));
        }

        _ => {
            v.push(enc.opcode_imm);
            let modrm = 0b11_000_000
                | ((enc.modrm_ext & 7) << 3)
                | (reg_u8 & 7);
            v.push(modrm);
        }
    }

    emit_imm(&mut v, val, size);

    EncodeInformation {
        data: v,
        ..Default::default()
    }
}

fn encode_reg_sym(
    op: BinOp,
    reg: Register,
    sym: &str,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {
    let mut v = Vec::new();
    let reg_u8 = reg as u8;

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, Some(reg_u8));

    match op {
        BinOp::Mov => {
            let base = match size {
                Size::U8 => 0xB0,
                _ => 0xB8,
            };
            v.push(base + (reg_u8 & 7));
        }

        _ => {
            v.push(enc.opcode_imm);
            let modrm = 0b11_000_000
                | ((enc.modrm_ext & 7) << 3)
                | (reg_u8 & 7);
            v.push(modrm);
        }
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
    val: usize,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {
    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, None);

    v.push(enc.opcode_imm);

    let modrm_info = mod_rm_encode(dst, &Operand::NoOperand);

    let base = v.len();
    v.extend(modrm_info.data);

    let after = v.len();

    emit_imm(&mut v, val, size);

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
        _ => 0,
    };

    let rm_u8 = match rm_op {
        Operand::Reg(r) => *r as u8,
        _ => 0,
    };

    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, Some(reg_u8), Some(rm_u8));

    let opcode = if is_reg_dst {
        enc.opcode_r_rm
    } else {
        enc.opcode_rm_r
    };

    v.push(opcode);

    let base = v.len();
    let modrm = mod_rm_encode(rm_op, reg_op);

    v.extend(modrm.data);

    let relocations = modrm.relocations
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

        (Operand::Reg(reg), Operand::Imm(val)) => {
            encode_reg_imm(op, *reg, *val, size, &enc)
        }

        (Operand::Reg(reg), Operand::Sym(sym)) => {
            encode_reg_sym(op, *reg, sym, size, &enc)
        }

        (Operand::MemoryAddress(_), Operand::Imm(val)) => {
            encode_mem_imm(dst, *val, size, &enc)
        }

        (
            Operand::Reg(_) | Operand::MemoryAddress(_),
            Operand::Reg(_) | Operand::MemoryAddress(_),
        ) => {
            encode_reg_mem(dst, src, size, &enc)
        }

        _ => unimplemented!("unsupported operands: {:?}, {:?}", dst, src),
    }
}
