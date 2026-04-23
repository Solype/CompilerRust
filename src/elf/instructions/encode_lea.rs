use super::{
    EncodeInformation,
    Operand,
    Register,
    Size,
    encode_alu::emit_size_prefix,
    modrm::{emit_rex, mod_rm_encode},
};

pub(super) fn encode_lea(
    dst: &Register,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let dst_u8 = (*dst) as u8;

    let rm_u8 = match src {
        Operand::Reg(r) => *r as u8,
        _ => 0,
    };

    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, Some(dst_u8), Some(rm_u8));

    // LEA opcode
    v.push(0x8D);

    let base = v.len();

    // reg = destination register
    // r/m = memory source
    let modrm = mod_rm_encode(src, &Operand::Reg(*dst));

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
