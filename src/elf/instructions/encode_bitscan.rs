use std::vec;
use super::{
    enums::*,
    modrm::mod_rm_encode,
};

pub(super) fn encode_bitscan(
    op: &BitScanOp,
    dst: &Register,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let mut buf: Vec<u8> = vec![];

    // Préfixes selon la taille
    match size {
        Size::U16 => buf.push(0x66),
        Size::U32 => {}
        Size::U64 => {
            // REX.W = 0x48 (minimum)
            // ⚠️ à adapter si tu gères REX dynamiquement (R/X/B bits)
            buf.push(0x48);
        }
        Size::U8 => panic!("BSF/BSR n'existent pas en 8 bits"),
    }

    // Préfixe 0x0F obligatoire
    buf.push(0x0F);

    // Opcode
    let opcode = match op {
        BitScanOp::Bsf => 0xBC,
        BitScanOp::Bsr => 0xBD,
    };
    buf.push(opcode);

    // ModRM (reg = dst, r/m = src)
    let modrm = mod_rm_encode(&src, &Operand::Reg(*dst));
    buf.extend(modrm.data);

    EncodeInformation {
        data: buf,
        ..Default::default()
    }
}
