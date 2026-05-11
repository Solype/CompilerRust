
use super::{
    ComplexBinOp,
    EncodeInformation,
    Operand,
    register::*,
    Size,
    utils::{emit_size_prefix, emit_rex, emit_imm},
    encode_xadd_cmp::encode_xadd_cmpxchg,
    modrm::{mod_rm_encode}
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
    let mut v = EncodeInformation::new();

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
        Operand::Reg(r) => Some(*r),
        _ => None,
    };

    emit_rex(&mut v, size, None, rm);

    v.push(opcode);

    let modrm = mod_rm_encode(
        src,
        &Operand::Reg(Register { class: RegisterClass::Gpr, index: reg_field, }),
    );
    v.append(modrm);
    return v
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

    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);

    let rm = match src {
        Operand::Reg(r) => Some(*r),
        _ => None,
    };

    emit_rex(&mut v, size, None, rm);

    v.push(0x0F);
    v.push(0xAF);

    let modrm = mod_rm_encode(
        src,
        &Operand::Reg(dst_reg),
    );

    v.append(modrm);
    v
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

    let fits_i8 = (imm as i32) >= -128 && (imm as i32) <= 127;

    let opcode = if fits_i8 { 0x6B } else { 0x69 };

    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);

    let rm = match src {
        Operand::Reg(r) => Some(*r),
        _ => None,
    };

    emit_rex(&mut v, size, None, rm);

    v.push(opcode);

    let modrm = mod_rm_encode( src, &Operand::Reg(dst_reg),);

    v.append(modrm);
    v.extend_vec(emit_imm(imm, if opcode == 0x6B { Size::U8 } else { size }));
    v
}
