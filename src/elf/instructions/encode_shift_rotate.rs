use crate::elf::instructions::{EncodeInformation, Operand, Register, ShiftOp, Size, encode_alu::emit_size_prefix, modrm::{emit_rex, mod_rm_encode}};


pub fn encode_shift_rotate(
    op: &ShiftOp,
    dst: &Operand,
    src: &Operand,
    size: Size,
) -> EncodeInformation {
    let mut v = Vec::new();

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
    let reg_field = Operand::Reg(Register::try_from(ext).unwrap());
    emit_rex(&mut v, size, None, None);

    // ================================
    // Opcode selection
    // ================================
    match src {
        Operand::Imm(1) => v.push(opcode_1),

        Operand::Imm(_) => v.push(opcode_imm),

        Operand::Reg(reg) if *reg == Register::C => {
            v.push(opcode_cl)
        }

        _ => panic!("Invalid shift count: must be 1, imm8, or CL"),
    }

    // ================================
    // ModRM
    // ================================
    let base = v.len();
    let modrm = mod_rm_encode(dst, &reg_field);
    v.extend(modrm.data);

    // ================================
    // Immediate (If necessary)
    // ================================
    if let Operand::Imm(n) = src {
        if *n != 1 {
            v.push(*n as u8);
        }
    }

    // ================================
    // Relocations
    // ================================
    let relocations = modrm.relocations.into_iter().map(|mut r| {
            r.offset += base;
            r
        }).collect();

    EncodeInformation {
        data: v,
        relocations,
    }
}
