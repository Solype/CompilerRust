use super::{
    modrm::*,
    enums::*,
    encode_alu::emit_size_prefix,
};

pub(super) fn encode_bit(
    op: BitOp,
    dst: &Operand,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let (opcode_rr, modrm_ext) = match op {
        BitOp::Bt  => (0xA3, 4),
        BitOp::Bts => (0xAB, 5),
        BitOp::Btr => (0xB3, 6),
        BitOp::Btc => (0xBB, 7),
    };

    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);

    match src {
        // =====================================================
        // bit r/m, reg
        // 0F A3/AB/B3/BB /r
        // =====================================================
        Operand::Reg(reg) => {
            let reg_u8 = *reg as u8;

            let rm_u8 = match dst {
                Operand::Reg(r) => *r as u8,
                Operand::MemoryAddress(_) => 0,
                _ => unimplemented!("invalid BT destination"),
            };

            emit_rex(&mut v, size, Some(reg_u8), Some(rm_u8));

            v.push(0x0F);
            v.push(opcode_rr);

            let base = v.len();

            let modrm = mod_rm_encode(dst, src);

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

        // =====================================================
        // bit r/m, imm8
        // 0F BA /4..7 ib
        // =====================================================
        Operand::Imm(bit) => {
            let rm_u8 = match dst {
                Operand::Reg(r) => *r as u8,
                Operand::MemoryAddress(_) => 0,
                _ => unimplemented!("invalid BT destination"),
            };

            emit_rex(&mut v, size, None, Some(rm_u8));

            v.push(0x0F);
            v.push(0xBA);

            let reg_field =
                Operand::Reg(Register::try_from(modrm_ext).unwrap());

            let base = v.len();

            let modrm = mod_rm_encode(dst, &reg_field);

            v.extend(modrm.data);

            v.push(*bit as u8);

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

        _ => unimplemented!("invalid BT source {:?}", src),
    }
}