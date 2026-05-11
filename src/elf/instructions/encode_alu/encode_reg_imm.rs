use super::super::{
    enums::*,
    struct_encode_information::*,
    utils::{emit_size_prefix, emit_rex, emit_imm},
    register::*,
};
use super::structs::*;

pub(super) fn encode_reg_int(
    reg: &Register,
    val: i64,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {

    if !matches!(reg.class, RegisterClass::Gpr) {
        panic!("Invalid register, must be of class GPR");
    }

    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, Some(*reg));

    match *enc {

        // --------------------------------------------------
        // MOV reg, imm
        // --------------------------------------------------
        BinaryEncoding::Mov { .. } => {
            let base = match size {
                Size::U8 => 0xB0,
                _ => 0xB8,
            };

            v.push(base + reg.low3());
        }

        // --------------------------------------------------
        // ALU reg, imm
        // --------------------------------------------------
        BinaryEncoding::Alu {
            opcode_imm,
            opcode_imm8,
            modrm_ext,
            ..
        } => {

            let fits_i8 =
                (val as i64) >= -128 &&
                (val as i64) <= 127;

            let use_imm8 =
                fits_i8 &&
                opcode_imm8.is_some() &&
                size != Size::U8;

            let opcode = if use_imm8 {
                opcode_imm8.unwrap()
            } else {
                opcode_imm
            };

            v.push(opcode);

            let modrm =
                0b11_000_000 |
                ((modrm_ext & 7) << 3) |
                reg.low3();

            v.push(modrm);

            if use_imm8 {
                v.push(val as i8 as u8);
                return v;
            }
        }

        _ => {
            unimplemented!(
                "encode reg <- imm not implemented for {:?}",
                *enc
            )
        }
    }

    v.extend_vec(emit_imm(val, size));

    v
}


pub(super) fn encode_reg_float(
    reg: &Register,
    val: f64,
    _size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {

    if !matches!(reg.class, RegisterClass::Xmm) {
        panic!("Invalid register, must be XMM");
    }

    let mut v = EncodeInformation::new();

    match *enc {

        BinaryEncoding::Sse {
            prefix,
            opcode,
        } => {

            // mandatory SSE prefix
            if prefix != 0 {
                v.push(prefix);
            }

            // REX
            emit_rex(&mut v, Size::U32, Some(*reg), None);

            // SSE escape
            v.push(0x0F);

            // opcode
            v.push(opcode);

            // RIP-relative addressing:
            //
            // mod = 00
            // rm  = 101
            // reg = xmm register
            //
            let modrm =
                ((reg.low3() & 7) << 3) |
                0b101;

            v.push(modrm);

            let offset = v.len();

            // disp32 placeholder
            v.extend_vec(vec![0, 0, 0, 0]);

            // relocation to synthetic float constant
            v.add_relocation(Relocation {
                sym: format!(".LC{:016X}", val.to_bits()),
                offset,
                size: 4,
                kind: RelocKind::Relative,
                addend: -4,
            });
        }

        _ => {
            unimplemented!(
                "float immediates only supported for SSE encodings"
            )
        }
    }

    v
}
