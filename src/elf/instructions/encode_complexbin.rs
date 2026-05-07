use crate::elf::instructions::{
    ComplexBinOp, EncodeInformation, Operand, Register, Size, encode_alu::emit_size_prefix, encode_xadd_cmp::encode_xadd_cmpxchg, modrm::{emit_rex, mod_rm_encode}
};

pub fn encode_complex_binary(
    op: &ComplexBinOp,
    dst: &Operand,
    src: &Operand,
    extra: &Option<Operand>,
    size: Size,
) -> EncodeInformation {
    match op {
        // MUL / DIV / IDIV unary
        ComplexBinOp::Mul
        | ComplexBinOp::Div
        | ComplexBinOp::Idiv
            if matches!(dst, Operand::NoOperand) && extra.is_none() =>
        {
            encode_group_f6_f7(op, src, size)
        }

        // IMUL unary (implicit AX/EAX/RAX)
        ComplexBinOp::Imul
            if matches!(dst, Operand::NoOperand) && extra.is_none() =>
        {
            encode_group_f6_f7(op, src, size)
        }

        // IMUL r, r/m
        ComplexBinOp::Imul
            if !matches!(dst, Operand::NoOperand) && extra.is_none() =>
        {
            encode_imul_two_operands(dst, src, size)
        }

        // IMUL r, r/m, imm
        ComplexBinOp::Imul
            if !matches!(dst, Operand::NoOperand) && extra.is_some() =>
        {
            encode_imul_three_operands(dst, src, extra, size)
        }
        ComplexBinOp::Xadd => encode_xadd_cmpxchg(op, dst, src, size),
        ComplexBinOp::Cmpxchg => encode_xadd_cmpxchg(op, dst, src, size),
        _ => {
            panic!(
                concat!(
                    "unsupported complex binary form:\n",
                    "  op    = {:?}\n",
                    "  dst   = {:?}\n",
                    "  src   = {:?}\n",
                    "  extra = {:?}\n",
                    "  size  = {:?}\n",
                    "\nExpected forms:\n",
                    "  Mul/Div/Idiv/Imul unary : dst=NoOperand, extra=None\n",
                    "  Imul two operands       : dst=Reg, extra=None\n",
                    "  Imul three operands     : dst=Reg, extra=Imm"
                ),
                op, dst, src, extra, size
            );
        }
    }
}

fn encode_group_f6_f7(
    op: &ComplexBinOp,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);

    let (opcode, reg_field) = match op {
        ComplexBinOp::Mul => match size {
            Size::U8 => (0xF6, 4u8),
            _ => (0xF7, 4u8),
        },

        ComplexBinOp::Imul => match size {
            Size::U8 => (0xF6, 5u8),
            _ => (0xF7, 5u8),
        },

        ComplexBinOp::Div => match size {
            Size::U8 => (0xF6, 6u8),
            _ => (0xF7, 6u8),
        },

        ComplexBinOp::Idiv => match size {
            Size::U8 => (0xF6, 7u8),
            _ => (0xF7, 7u8),
        },

        _ => unreachable!(),
    };

    let rm = match src {
        Operand::Reg(r) => *r as u8,
        _ => 0,
    };

    emit_rex(&mut v, size, None, Some(rm));

    v.push(opcode);

    let base = v.len();

    let modrm = mod_rm_encode(
        src,
        &Operand::Reg(Register::try_from(reg_field).unwrap()),
    );

    v.extend(modrm.data);

    let relocations = modrm
        .relocations
        .into_iter()
        .map(|mut r| {
            r.offset += base;
            r
        })
        .collect();

    EncodeInformation { data: v, relocations }
}

fn encode_imul_two_operands(
    dst: &Operand,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let dst_reg = match dst {
        Operand::Reg(r) => *r,
        _ => panic!("imul dst must be register"),
    };

    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);

    let rm = match src {
        Operand::Reg(r) => *r as u8,
        _ => 0,
    };

    emit_rex(&mut v, size, Some(dst_reg as u8), Some(rm));

    v.push(0x0F);
    v.push(0xAF);

    let base = v.len();

    let modrm = mod_rm_encode(
        src,
        &Operand::Reg(dst_reg),
    );

    v.extend(modrm.data);

    let relocations = modrm
        .relocations
        .into_iter()
        .map(|mut r| {
            r.offset += base;
            r
        })
        .collect();

    EncodeInformation { data: v, relocations }
}

pub fn emit_imm(v: &mut Vec<u8>, imm: i64, size: Size) {
    match size {
        Size::U8  => v.push(imm as i8 as u8),
        Size::U16 => v.extend_from_slice(&(imm as i16).to_le_bytes()),
        Size::U32 => v.extend_from_slice(&(imm as i32).to_le_bytes()),
        Size::U64 => v.extend_from_slice(&(imm as i64).to_le_bytes()),
    }
}

fn encode_imul_three_operands(
    dst: &Operand,
    src: &Operand,
    extra: &Option<Operand>,
    size: Size,
) -> EncodeInformation {
    let dst_reg = match dst {
        Operand::Reg(r) => *r,
        _ => panic!("imul dst must be register"),
    };

    let imm = match extra {
        Some(Operand::Imm(v)) => *v,
        _ => panic!("imul extra must be immediate"),
    };

    let fits_i8 = (imm as i64) >= -128 && (imm as i64) <= 127;

    let opcode = if fits_i8 { 0x6B } else { 0x69 };

    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);

    let rm = match src {
        Operand::Reg(r) => *r as u8,
        _ => 0,
    };

    emit_rex(&mut v, size, Some(dst_reg as u8), Some(rm));

    v.push(opcode);

    let base = v.len();

    let modrm = mod_rm_encode(
        src,
        &Operand::Reg(dst_reg),
    );

    v.extend(modrm.data);

    emit_imm(&mut v, imm, if opcode == 0x6B { Size::U8 } else { size });

    let relocations = modrm
        .relocations
        .into_iter()
        .map(|mut r| {
            r.offset += base;
            r
        })
        .collect();

    EncodeInformation { data: v, relocations }
}
