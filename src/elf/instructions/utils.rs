use super::{EncodeInformation, Size, register::Register};

pub(super) fn emit_rex(
    v: &mut EncodeInformation,
    size: Size,
    reg: Option<Register>,
    rm: Option<Register>,
) {
    let mut rex = 0x40;

    if let Size::U64 = size {
        rex |= 1 << 3; // W
    }

    if let Some(r) = reg {
        rex |= r.rex_bit() << 2; // R
    }

    if let Some(b) = rm {
        rex |= b.rex_bit(); // B
    }

    if rex != 0x40 {
        v.push(rex);
    }
}

pub(super) fn emit_size_prefix(v: &mut EncodeInformation, size: Size) {
    if let Size::U16 = size {
        v.push(0x66);
    }
}

pub(super) fn emit_imm(val: i64, size: Size) -> Vec<u8> {
    match size {
        Size::U8 => vec![val as u8],
        Size::U16 => (val as u16).to_le_bytes().to_vec(),
        Size::U32 => (val as u32).to_le_bytes().to_vec(),
        Size::U64 => (val as u64).to_le_bytes().to_vec(),
    }
}
