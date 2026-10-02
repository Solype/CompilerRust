use super::{
    enums::*, modrm::mod_rm_encode, register::*, struct_encode_information::*, utils::emit_rex,
};

/// Forme des opérandes d'une conversion
enum Shape {
    /// xmm <- r/m32 | r/m64 ; `size` est celle de l'entier source (U64 -> REX.W)
    IntToFloat,
    /// r32 | r64 <- xmm/m ; `size` est celle de l'entier destination (U64 -> REX.W)
    FloatToInt,
    /// xmm <- xmm/m ; jamais de REX.W, `size` est ignorée
    FloatToFloat,
}

/// (forme, préfixe obligatoire, opcode après 0x0F)
fn get_opcode(op: ConvOp) -> (Shape, u8, u8) {
    match op {
        // CVTSD2SI r32 | r64, xmm/m64 : F2 [REX.W] 0F 2D /r
        ConvOp::Cvtsd2si => (Shape::FloatToInt, 0xF2, 0x2D),
        // CVTSD2SS xmm, xmm/m64 : F2 0F 5A /r
        ConvOp::Cvtsd2ss => (Shape::FloatToFloat, 0xF2, 0x5A),
        // CVTSI2SD xmm, r/m32 | r/m64 : F2 [REX.W] 0F 2A /r
        ConvOp::Cvtsi2sd => (Shape::IntToFloat, 0xF2, 0x2A),
        // CVTSI2SS xmm, r/m32 | r/m64 : F3 [REX.W] 0F 2A /r
        ConvOp::Cvtsi2ss => (Shape::IntToFloat, 0xF3, 0x2A),
        // CVTSS2SD xmm, xmm/m32 : F3 0F 5A /r
        ConvOp::Cvtss2sd => (Shape::FloatToFloat, 0xF3, 0x5A),
        // CVTSS2SI r32 | r64, xmm/m32 : F3 [REX.W] 0F 2D /r
        ConvOp::Cvtss2si => (Shape::FloatToInt, 0xF3, 0x2D),
        // CVTTSD2SI r32 | r64, xmm/m64 : F2 [REX.W] 0F 2C /r
        ConvOp::Cvttsd2si => (Shape::FloatToInt, 0xF2, 0x2C),
        // CVTTSS2SI r32 | r64, xmm/m32 : F3 [REX.W] 0F 2C /r
        ConvOp::Cvttss2si => (Shape::FloatToInt, 0xF3, 0x2C),
    }
}

fn check_int_size(op: ConvOp, size: Size, side: &str) {
    if let Size::U8 | Size::U16 = size {
        panic!("{:?}: integer {} must be 32 or 64 bits", op, side);
    }
}

pub(super) fn encode_conversion(
    op: ConvOp,
    dst: &Register,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let (shape, prefix, opcode) = get_opcode(op);

    let src_is_gpr = matches!(src, Operand::Reg(r) if r.is_gpr());
    let src_is_xmm = matches!(src, Operand::Reg(r) if r.is_xmm());
    let src_is_mem = matches!(src, Operand::MemoryAddress(_));

    let rex_size = match shape {
        Shape::IntToFloat => {
            if !dst.is_xmm() {
                panic!("{:?}: destination must be an XMM register", op);
            }
            if !(src_is_gpr || src_is_mem) {
                panic!("{:?}: source must be a GPR or a memory address", op);
            }
            check_int_size(op, size, "source");
            size
        }
        Shape::FloatToInt => {
            if !dst.is_gpr() {
                panic!("{:?}: destination must be a GPR", op);
            }
            if !(src_is_xmm || src_is_mem) {
                panic!("{:?}: source must be an XMM register or a memory address", op);
            }
            check_int_size(op, size, "destination");
            size
        }
        Shape::FloatToFloat => {
            if !dst.is_xmm() {
                panic!("{:?}: destination must be an XMM register", op);
            }
            if !(src_is_xmm || src_is_mem) {
                panic!("{:?}: source must be an XMM register or a memory address", op);
            }
            Size::U32
        }
    };

    let mut v = EncodeInformation::new();

    // mandatory prefix must come BEFORE REX
    v.push(prefix);
    emit_rex(&mut v, rex_size, Some(*dst), src);
    v.push(0x0F);
    v.push(opcode);

    let modrm = mod_rm_encode(src, &Operand::Reg(*dst));
    v.append(modrm);
    v
}
