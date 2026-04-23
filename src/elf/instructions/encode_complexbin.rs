use crate::elf::instructions::{
    ComplexBinOp,
    EncodeInformation,
    Operand,
    Register,
    Size,
    encode_alu::emit_size_prefix,
    modrm::{emit_rex, mod_rm_encode},
};

pub fn encode_complex_binary(
    op: &ComplexBinOp,
    _dst: &Operand,
    src: &Operand,
    _extra: &Option<Operand>,
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
    };

    // IMPORTANT:
    // REX must be emitted BEFORE opcode
    let rm_u8 = match src {
        Operand::Reg(r) => *r as u8,
        _ => 0,
    };

    emit_rex(&mut v, size, None, Some(rm_u8));

    // opcode after prefixes
    v.push(opcode);

    // ModRM:
    // reg field = /4 /5 /6 /7 selector
    // r/m field = source operand
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

    EncodeInformation {
        data: v,
        relocations,
    }
}
