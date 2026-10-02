use super::{
    enums::*, modrm::mod_rm_encode, register::*, struct_encode_information::*,
    utils::{emit_rex, emit_size_prefix, is_rex_byte_register},
};

/// (0x0F prefix or not, opcode) depending on the source size
fn get_opcode(op: ExtendOp, src_size: Size) -> (Option<u8>, u8) {
    match (op, src_size) {
        // MOVSX r16 | r32 | r64, r/m8 | r/m16 : [66] [REX.W] 0F BE | BF /r
        (ExtendOp::Movsx, Size::U8) => (Some(0x0F), 0xBE),
        (ExtendOp::Movsx, _) => (Some(0x0F), 0xBF),
        // MOVSXD r64, r/m32 : REX.W 63 /r
        (ExtendOp::Movsxd, _) => (None, 0x63),
        // MOVZX r16 | r32 | r64, r/m8 | r/m16 : [66] [REX.W] 0F B6 | B7 /r
        (ExtendOp::Movzx, Size::U8) => (Some(0x0F), 0xB6),
        (ExtendOp::Movzx, _) => (Some(0x0F), 0xB7),
    }
}

/// `src_size` is the source size, `size` the destination size
pub(super) fn encode_extend(
    op: ExtendOp,
    dst: &Register,
    src: &Operand,
    src_size: Option<Size>,
    size: Size,
) -> EncodeInformation {
    if !dst.is_gpr() {
        panic!("{:?}: destination must be a GPR", op);
    }

    match src {
        Operand::Reg(r) if r.is_gpr() => {}
        Operand::MemoryAddress(_) => {}
        _ => panic!("{:?}: source must be a GPR or a memory address", op),
    }

    let src_size = match (op, src_size) {
        (ExtendOp::Movsxd, None | Some(Size::U32)) => Size::U32,
        (ExtendOp::Movsxd, Some(_)) => panic!("{:?}: source must be 32 bits", op),
        (_, Some(s @ (Size::U8 | Size::U16))) => s,
        (_, _) => panic!("{:?}: source must be 8 or 16 bits", op),
    };

    match (op, size) {
        (ExtendOp::Movsxd, Size::U64) => {}
        (ExtendOp::Movsxd, _) => panic!("{:?}: destination must be 64 bits", op),
        (_, Size::U8) => panic!("{:?}: destination must be 16, 32 or 64 bits", op),
        (_, _) if size as u8 <= src_size as u8 => {
            panic!("{:?}: destination must be wider than the source", op)
        }
        _ => {}
    }

    let (prefix, opcode) = get_opcode(op, src_size);

    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);

    // REX.W comes from the destination; an empty REX is only needed for an
    // spl / bpl / sil / dil source (the destination is never 8 bits)
    let before = v.len();
    emit_rex(&mut v, size, Some(*dst), src);
    let byte_src = matches!(src, Operand::Reg(r) if src_size == Size::U8 && is_rex_byte_register(*r));
    if v.len() == before && byte_src {
        v.push(0x40);
    }

    if let Some(prefix) = prefix {
        v.push(prefix);
    }
    v.push(opcode);

    let modrm = mod_rm_encode(src, &Operand::Reg(*dst));
    v.append(modrm);
    v
}
