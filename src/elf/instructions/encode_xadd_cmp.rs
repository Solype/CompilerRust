use super::{
    ComplexBinOp, EncodeInformation, Operand, Size,
    modrm::mod_rm_encode,
    utils::{emit_rex, emit_size_prefix},
};

pub(super) fn encode_xadd_cmpxchg(
    op: &ComplexBinOp,
    dst: &Operand,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let mut v = EncodeInformation::new();

    // ==========================================
    // Préfixe taille + REX
    // ==========================================
    emit_size_prefix(&mut v, size);

    let reg = match src {
        Operand::Reg(r) => *r,
        _ => panic!("src must be a register"),
    };

    let rm = match dst {
        Operand::Reg(r) => Some(*r),
        Operand::MemoryAddress(_) => None,
        _ => panic!("invalid dst for xadd/cmpxchg"),
    };

    emit_rex(&mut v, size, Some(reg), rm);

    // ==========================================
    // Opcode
    // ==========================================
    v.push(0x0F);

    let opcode = match op {
        ComplexBinOp::Xadd => {
            if size == Size::U8 {
                0xC0
            } else {
                0xC1
            }
        }

        ComplexBinOp::Cmpxchg => {
            if size == Size::U8 {
                0xB0
            } else {
                0xB1
            }
        }

        _ => unreachable!(),
    };

    v.push(opcode);

    // ==========================================
    // ModRM
    // dst = r/m, src = reg
    // ==========================================

    let modrm = mod_rm_encode(dst, src);

    v.append(modrm);
    v
}
