use super::{
    EncodeInformation,
    Operand,
    register::*,
    ShiftOp,
    Size,
    modrm::{mod_rm_encode},
    utils::{emit_size_prefix, emit_rex},
};


pub fn encode_shift_rotate(
    op: &ShiftOp,
    dst: &Operand,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let mut v = EncodeInformation::new();

    // ================================
    // Size-dependent opcodes
    // ================================
    let (opcode_1, opcode_cl, opcode_imm) = match size {
        Size::U8 => (0xD0, 0xD2, 0xC0),
        _        => (0xD1, 0xD3, 0xC1),
    };

    // ================================
    // /ext field (ModRM.reg)
    // ================================
    let ext = match op {
        ShiftOp::Rol => 0,
        ShiftOp::Ror => 1,
        ShiftOp::Shl => 4,
        ShiftOp::Shr => 5,
        ShiftOp::Sar => 7,
    };

    // ================================
    // Prefixes
    // ================================
    emit_size_prefix(&mut v, size);

    // REX (important pour registres étendus + 64-bit)
    let reg_field = Operand::Reg(Register {
        class: RegisterClass::Gpr,
        index: ext,
    });
    emit_rex(&mut v, size, None, None);

    // ================================
    // Opcode selection
    // ================================
    match src {
        Operand::Imm(1) => v.push(opcode_1),

        Operand::Imm(_) => v.push(opcode_imm),

        Operand::Reg(reg) if reg.class == RegisterClass::Gpr && reg.index == Gpr::C as u8 =>
        {
            v.push(opcode_cl)
        }

        _ => panic!("Invalid shift count: must be 1, imm8, or CL"),
    }

    // ================================
    // ModRM
    // ================================
    let modrm = mod_rm_encode(dst, &reg_field);
    v.append(modrm);

    // ================================
    // Immediate (If necessary)
    // ================================
    if let Operand::Imm(n) = src {
        if *n != 1 {
            v.push(*n as u8);
        }
    }
    v
}
