use super::modrm::*;
use super::enums::*;
use super::encode_alu::emit_size_prefix;

#[derive(Debug, Clone, Copy)]
struct UnaryEncoding {
    opcode: u8,
    modrm_ext: u8,
}

impl UnaryOp {
    fn encoding(self, size: Size) -> UnaryEncoding {
        match self {
            UnaryOp::Inc => UnaryEncoding {
                opcode: if size == Size::U8 { 0xFE } else { 0xFF },
                modrm_ext: 0,
            },

            UnaryOp::Dec => UnaryEncoding {
                opcode: if size == Size::U8 { 0xFE } else { 0xFF },
                modrm_ext: 1,
            },

            UnaryOp::Not => UnaryEncoding {
                opcode: if size == Size::U8 { 0xF6 } else { 0xF7 },
                modrm_ext: 2,
            },

            UnaryOp::Neg => UnaryEncoding {
                opcode: if size == Size::U8 { 0xF6 } else { 0xF7 },
                modrm_ext: 3,
            },
        }
    }
}

pub(super) fn encode_unary(
    op: UnaryOp,
    dst: &Operand,
    size: Size,
) -> EncodeInformation {
    let enc = op.encoding(size);

    let rm_u8 = match dst {
        Operand::Reg(r) => *r as u8,
        _ => 0,
    };

    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, Some(rm_u8));

    v.push(enc.opcode);

    let reg_field = Operand::Reg(Register::try_from(enc.modrm_ext).unwrap());

    let base = v.len();
    let modrm = mod_rm_encode(dst, &reg_field);

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
