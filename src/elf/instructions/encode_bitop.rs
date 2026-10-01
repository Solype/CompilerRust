use super::{
    enums::*,
    modrm::*,
    register::*,
    struct_encode_information::*,
    utils::{emit_rex, emit_size_prefix},
};

pub(super) fn encode_bit(op: BitOp, dst: &Operand, src: &Operand, size: Size) -> EncodeInformation {
    let (opcode_rr, modrm_ext) = match op {
        BitOp::Bt => (0xA3, 4),
        BitOp::Bts => (0xAB, 5),
        BitOp::Btr => (0xB3, 6),
        BitOp::Btc => (0xBB, 7),
    };

    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);

    match src {
        // =====================================================
        // bit r/m, reg
        // 0F A3/AB/B3/BB /r
        // =====================================================
        Operand::Reg(reg) => {
            let rm = match dst {
                Operand::Reg(r) => Some(*r),
                Operand::MemoryAddress(_) => None,
                _ => unimplemented!("invalid BT destination"),
            };

            emit_rex(&mut v, size, Some(*reg), rm);

            v.push(0x0F);
            v.push(opcode_rr);

            let modrm = mod_rm_encode(dst, src);

            v.append(modrm);
            return v;
        }

        // =====================================================
        // bit r/m, imm8
        // 0F BA /4..7 ib
        // =====================================================
        Operand::Imm(bit) => {
            let rm = match dst {
                Operand::Reg(r) => Some(*r),
                Operand::MemoryAddress(_) => None,

                _ => {
                    unimplemented!("invalid BT destination")
                }
            };

            emit_rex(&mut v, size, None, rm);

            v.push(0x0F);
            v.push(0xBA);

            let reg_field = Operand::Reg(Register {
                class: RegisterClass::Gpr,
                index: modrm_ext,
            });

            let modrm = mod_rm_encode(dst, &reg_field);

            v.append(modrm);

            v.push(*bit as u8);

            return v;
        }

        _ => unimplemented!("invalid BT source {:?}", src),
    }
}
