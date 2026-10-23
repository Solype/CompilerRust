use super::{generation::Context, instructions::convert_operand};
use crate::{
    converter::ir::structs::{Operand as IrOp, VarId},
    elf::instructions::{
        BinOp, Instruction, MemAddress, Operand as ElfOper, Size, register::Register,
    },
};

/// The size of a `mov` that touches `var`: its own size in bytes, so that writing a 4-byte
/// 짧은 정수 does not overflow on its neighbour
pub fn var_size(var: &VarId) -> Option<Size> {
    return Some(Size::from(var.1));
}

/// `reg = op`: a string is an address, computed by lea, not a value read by mov
pub fn load_operand(reg: Register, op: &IrOp, ctx: &Context, size: Option<Size>) -> Instruction {
    return match op {
        IrOp::Str(id) => Instruction::Lea {
            dst: reg,
            src: ElfOper::MemoryAddress(MemAddress::symbol(&ctx.strings[*id]).rip_relative()),
            size: None,
        },
        _ => Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(reg),
            src: convert_operand(op, ctx),
            size,
        },
    };
}
