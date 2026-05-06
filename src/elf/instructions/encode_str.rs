use super::enums::*;

pub(super) fn encode_str(op: &StringOp, size: Size) -> EncodeInformation {
    let mut v = Vec::new();

    // ===============================
    // Size prefix
    // ===============================
    match size {
        Size::U16 => v.push(0x66),
        Size::U64 => v.push(0x48), // REX.W
        _ => {}
    }

    // ===============================
    // Opcode
    // ===============================
    let opcode = match op {
        StringOp::Movs => match size {
            Size::U8 => 0xA4,
            _ => 0xA5,
        },

        StringOp::Cmps => match size {
            Size::U8 => 0xA6,
            _ => 0xA7,
        },

        StringOp::Scas => match size {
            Size::U8 => 0xAE,
            _ => 0xAF,
        },

        StringOp::Lods => match size {
            Size::U8 => 0xAC,
            _ => 0xAD,
        },

        StringOp::Stos => match size {
            Size::U8 => 0xAA,
            _ => 0xAB,
        },
    };

    v.push(opcode);

    EncodeInformation {
        data: v,
        ..Default::default()
    }
}
