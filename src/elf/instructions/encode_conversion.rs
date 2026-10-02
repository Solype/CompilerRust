use super::{
    enums::*, modrm::mod_rm_encode, register::*, struct_encode_information::*, utils::emit_rex,
};

/// Operand form of a conversion
enum Shape {
    /// xmm <- r/m32 | r/m64; `size` is the size of the source integer (U64 -> REX.W)
    IntToFloat,
    /// r32 | r64 <- xmm/m; `size` is the size of the destination integer (U64 -> REX.W)
    FloatToInt,
    /// xmm <- xmm/m; never REX.W, `size` is ignored
    FloatToFloat,
}

/// (form, mandatory prefix, opcode after 0x0F)
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
        ConvOp::Movd | ConvOp::Movq => unreachable!("handled by encode_bit_move"),
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
    if let ConvOp::Movd | ConvOp::Movq = op {
        return encode_bit_move(op, dst, src);
    }

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

/// movd / movq: the bits are copied as is. The opcode depends on the
/// direction and the size comes from the instruction (`size` is ignored).
fn encode_bit_move(op: ConvOp, dst: &Register, src: &Operand) -> EncodeInformation {
    let rex_size = if let ConvOp::Movq = op { Size::U64 } else { Size::U32 };

    let src_is_gpr = matches!(src, Operand::Reg(r) if r.is_gpr());
    let src_is_xmm = matches!(src, Operand::Reg(r) if r.is_xmm());
    let src_is_mem = matches!(src, Operand::MemoryAddress(_));

    let mut v = EncodeInformation::new();

    match (dst.is_xmm(), op) {
        // MOVQ xmm, xmm/m64 : F3 0F 7E /r (what GNU as picks, no REX.W)
        (true, ConvOp::Movq) if src_is_xmm || src_is_mem => {
            v.push(0xF3);
            emit_rex(&mut v, Size::U32, Some(*dst), src);
            v.push(0x0F);
            v.push(0x7E);
            v.append(mod_rm_encode(src, &Operand::Reg(*dst)));
        }

        // MOVD xmm, r/m32 : 66 0F 6E /r
        // MOVQ xmm, r64   : 66 REX.W 0F 6E /r
        (true, _) => {
            if !(src_is_gpr || (src_is_mem && rex_size == Size::U32)) {
                panic!("{:?}: source must be a GPR or a memory address", op);
            }
            v.push(0x66);
            emit_rex(&mut v, rex_size, Some(*dst), src);
            v.push(0x0F);
            v.push(0x6E);
            v.append(mod_rm_encode(src, &Operand::Reg(*dst)));
        }

        // MOVD r32, xmm : 66 0F 7E /r
        // MOVQ r64, xmm : 66 REX.W 0F 7E /r
        // The GPR is in r/m and the XMM register in reg
        (false, _) => {
            if !dst.is_gpr() {
                panic!("{:?}: destination must be a GPR or an XMM register", op);
            }
            let Operand::Reg(xmm) = src else {
                panic!("{:?}: source must be an XMM register", op);
            };
            if !xmm.is_xmm() {
                panic!("{:?}: source must be an XMM register", op);
            }
            let gpr = Operand::Reg(*dst);
            v.push(0x66);
            emit_rex(&mut v, rex_size, Some(*xmm), &gpr);
            v.push(0x0F);
            v.push(0x7E);
            v.append(mod_rm_encode(&gpr, src));
        }
    }
    v
}
