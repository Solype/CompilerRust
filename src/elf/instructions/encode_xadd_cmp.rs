use crate::elf::instructions::{ComplexBinOp, EncodeInformation, Operand, Size, encode_alu::emit_size_prefix, modrm::{emit_rex, mod_rm_encode}};

pub(super) fn encode_xadd_cmpxchg(
    op: &ComplexBinOp,
    dst: &Operand,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let mut v = Vec::new();

    // ==========================================
    // Préfixe taille + REX
    // ==========================================
    emit_size_prefix(&mut v, size);

    let reg = match src {
        Operand::Reg(r) => *r as u8,
        _ => panic!("src must be a register"),
    };

    let rm = match dst {
        Operand::Reg(r) => *r as u8,
        Operand::MemoryAddress(_) => 0,
        _ => panic!("invalid dst for xadd/cmpxchg"),
    };

    emit_rex(&mut v, size, Some(reg), Some(rm));

    // ==========================================
    // Opcode
    // ==========================================
    v.push(0x0F);

    let opcode = match op {
        ComplexBinOp::Xadd => {
            if size == Size::U8 { 0xC0 } else { 0xC1 }
        }

        ComplexBinOp::Cmpxchg => {
            if size == Size::U8 { 0xB0 } else { 0xB1 }
        }

        _ => unreachable!(),
    };

    v.push(opcode);

    // ==========================================
    // ModRM
    // dst = r/m, src = reg
    // ==========================================
    let base = v.len();

    let modrm = mod_rm_encode(dst, src);

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
