use crate::elf::instructions::Operand;

use super::{
    EncodeInformation, Size,
    register::{Register, RegisterClass},
};

/// spl, bpl, sil, dil : sans REX, les index 4 à 7 en 8 bits désignent ah, ch, dh, bh
fn is_rex_byte_register(r: Register) -> bool {
    r.class == RegisterClass::Gpr && (4..8).contains(&r.index)
}

pub(super) fn emit_rex(v: &mut EncodeInformation, size: Size, reg: Option<Register>, rm: &Operand) {
    let mut rex = 0x40;

    if let Size::U64 = size {
        rex |= 1 << 3; // W
    }

    if let Some(r) = reg {
        rex |= r.rex_bit() << 2; // R
    }

    match rm {
        Operand::Reg(r) => rex |= r.rex_bit(), // B
        Operand::MemoryAddress(mem) => {
            if let Some(index) = mem.index {
                rex |= index.rex_bit() << 1; // X
            }
            if let Some(base) = mem.base {
                rex |= base.rex_bit(); // B
            }
        }
        _ => {}
    }

    let byte_reg = matches!(size, Size::U8)
        && (reg.is_some_and(is_rex_byte_register)
            || matches!(rm, Operand::Reg(r) if is_rex_byte_register(*r)));

    if rex != 0x40 || byte_reg {
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

pub(super) fn emit_imm_sx32(val: i64, size: Size) -> Vec<u8> {
    match size {
        Size::U64 => {
            let v = i32::try_from(val).unwrap_or_else(|_| {
                panic!("immédiate {val:#x} hors de i32 : seul `mov reg, imm64` accepte 64 bits")
            });
            v.to_le_bytes().to_vec()
        }
        _ => emit_imm(val, size),
    }
}
