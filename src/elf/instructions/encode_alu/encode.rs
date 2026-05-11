
use super::super::{
    modrm::*,
    enums::*,
    struct_encode_information::*,
    utils::{emit_size_prefix, emit_rex, emit_imm},
    register::*,
};

use super::*;



///////////////////////////////////////////////////////////////////

fn encode_reg_sym(
    reg: Register,
    sym: &str,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {
    if !matches!(reg.class, RegisterClass::Gpr) {
        panic!("registre invalide");
    }
    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, Some(reg));

    match *enc {

        // --------------------------------------------------
        // MOV reg, sym
        // --------------------------------------------------
        BinaryEncoding::Mov { .. } => {

            let base = match size {
                Size::U8 => 0xB0,
                _ => 0xB8,
            };

            v.push(base + reg.low3());
        }

        // --------------------------------------------------
        // ALU reg, sym
        // --------------------------------------------------
        BinaryEncoding::Alu {
            opcode_imm,
            modrm_ext,
            ..
        } => {

            v.push(opcode_imm);

            let modrm =
                0b11_000_000 |
                ((modrm_ext & 7) << 3) |
                reg.low3();

            v.push(modrm);
        }

        _ => unimplemented!(),
    }

    let offset = v.len();

    v.extend_vec(emit_imm(0, size));

    v.add_relocation(Relocation {
        sym: sym.to_string(),
        offset,
        size: size as u8,
        kind: RelocKind::Absolute,
        addend: 0,
    });

    v
}

///////////////////////////////////////////////////////////////////

fn encode_mem_imm(
    dst: &Operand,
    val: i64,
    size: Size,
    enc: &BinaryEncoding,
) -> EncodeInformation {

    let (opcode_imm, opcode_imm8, modrm_ext) = match *enc {

        BinaryEncoding::Mov { opcode_imm, .. } => (opcode_imm, None, 0),

        BinaryEncoding::Alu { opcode_imm, opcode_imm8, modrm_ext, .. } =>
            ( opcode_imm, opcode_imm8, modrm_ext, ),

        _ => {unimplemented!("mem, imm unsupported for this instruction")}
    };

    let fits_i8 = (val as i64) >= -128 && (val as i64) <= 127;
    let use_imm8 = fits_i8 && opcode_imm8.is_some() && size != Size::U8;
    let opcode = if use_imm8 { opcode_imm8.unwrap() } else { opcode_imm };

    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, None);

    v.push(opcode);

    let reg_field = Operand::Reg(Register {
        class: RegisterClass::Gpr,
        index: modrm_ext,
    });

    let modrm_info = mod_rm_encode(dst, &reg_field);

    v.append(modrm_info);

    if use_imm8 {
        v.push(val as i8 as u8);
    } else {
        v.extend_vec(emit_imm(val, size));
    }
    v
}

pub(in super::super) fn encode_binary(
    op: BinOp,
    dst: &Operand,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let enc = get_op_codes(op,size);

    match (dst, src) {

        (Operand::Reg(reg), Operand::Imm(val)) => encode_reg_int(reg, *val, size, &enc),
        (Operand::Reg(reg), Operand::Sym(sym)) => encode_reg_sym(*reg, sym, size, &enc),
        (Operand::MemoryAddress(_), Operand::Imm(val)) => encode_mem_imm(dst, *val, size, &enc),

        (
            Operand::Reg(_) | Operand::MemoryAddress(_),
            Operand::Reg(_) | Operand::MemoryAddress(_),
        ) => encode_reg_mem(dst, src, size, &enc),

        _ => unimplemented!("unsupported operands: {:?}, {:?}", dst, src),
    }
}
// Von 18 bis 6
