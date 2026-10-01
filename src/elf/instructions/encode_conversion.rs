use super::{
    enums::*, modrm::mod_rm_encode, register::*, struct_encode_information::*, utils::emit_rex,
};

/// (mandatory prefix, opcode after 0x0F)
fn get_opcode(op: ConvOp) -> (u8, u8) {
    match op {
        // CVTSI2SD xmm, r/m32 | r/m64 : F2 [REX.W] 0F 2A /r
        ConvOp::Cvtsi2sd => (0xF2, 0x2A),
    }
}

/// `size` is the size of the integer source (U32 or U64 -> REX.W)
pub(super) fn encode_conversion(
    op: ConvOp,
    dst: &Register,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    if !dst.is_xmm() {
        panic!("{:?}: destination must be an XMM register", op);
    }

    match src {
        Operand::Reg(r) if r.is_gpr() => {}
        Operand::MemoryAddress(_) => {}
        _ => panic!("{:?}: source must be a GPR or a memory address", op),
    }

    if let Size::U8 | Size::U16 = size {
        panic!("{:?}: integer source must be 32 or 64 bits", op);
    }

    let (prefix, opcode) = get_opcode(op);

    let mut v = EncodeInformation::new();

    // mandatory prefix must come BEFORE REX
    v.push(prefix);
    emit_rex(&mut v, size, Some(*dst), src);
    v.push(0x0F);
    v.push(opcode);

    let modrm = mod_rm_encode(src, &Operand::Reg(*dst));
    v.append(modrm);
    v
}
