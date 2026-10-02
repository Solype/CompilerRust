use super::{
    enums::*,
    modrm::mod_rm_encode,
    register::*,
    struct_encode_information::*,
    utils::{emit_rex, emit_size_prefix},
};

pub(super) fn encode_bitscan(
    op: &BitScanOp,
    dst: &Register,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    if let Size::U8 = size {
        panic!("BSF/BSR do not exist in 8 bits");
    }

    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, Some(*dst), src);

    // Mandatory 0x0F prefix
    v.push(0x0F);

    let opcode = match op {
        BitScanOp::Bsf => 0xBC,
        BitScanOp::Bsr => 0xBD,
    };
    v.push(opcode);

    // ModRM (reg = dst, r/m = src)
    let modrm = mod_rm_encode(src, &Operand::Reg(*dst));
    v.append(modrm);

    v
}
