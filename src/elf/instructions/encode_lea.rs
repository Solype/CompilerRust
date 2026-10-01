use super::{
    EncodeInformation, Operand, Size,
    modrm::mod_rm_encode,
    register::*,
    utils::{emit_rex, emit_size_prefix},
};

pub(super) fn encode_lea(dst: &Register, src: &Operand, size: Size) -> EncodeInformation {
    let rm = match src {
        Operand::Reg(r) => Some(*r),
        _ => None,
    };

    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, Some(*dst), rm);

    v.push(0x8D);
    let modrm = mod_rm_encode(src, &Operand::Reg(*dst));

    v.append(modrm);
    v
}
