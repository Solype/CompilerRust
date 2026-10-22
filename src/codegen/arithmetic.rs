use super::generation::Context;
use super::instructions::convert_operand;
use crate::codegen::utils::var_size;
use crate::converter::ir::structs::{Operand as IrOp, VarId};
use crate::elf::instructions::register::{RAX, RCX};
use crate::elf::instructions::{
    BinOp, ComplexBinOp, Instruction, Operand as ElfOper, ShiftOp, UnaryOp,
};
use crate::parser::postparser::structs::BinaryOp;

fn simple_operation(
    dest: &VarId,
    left: &IrOp,
    right: &IrOp,
    ctx: &Context,
    bin_op: BinOp,
) -> Vec<Instruction> {
    let instrs: Vec<Instruction> = vec![
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RAX),
            src: convert_operand(left, ctx),
            size: var_size(dest),
        },
        Instruction::Binary {
            op: bin_op,
            dst: ElfOper::Reg(RAX),
            src: convert_operand(right, ctx),
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(*dest), ctx),
            src: ElfOper::Reg(RAX),
            size: var_size(dest),
        },
    ];
    return instrs;
}

fn division(dest: &VarId, left: &IrOp, right: &IrOp, ctx: &Context) -> Vec<Instruction> {
    return vec![
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RAX),
            src: convert_operand(left, ctx),
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RCX),
            src: convert_operand(right, ctx),
            size: var_size(dest),
        },
        Instruction::Unary {
            op: if dest.1 == 4 {
                UnaryOp::Cdq
            } else {
                UnaryOp::Cqo
            },
            dst: ElfOper::NoOperand,
            size: var_size(dest),
        },
        Instruction::ComplexBinary {
            op: ComplexBinOp::Idiv,
            dst: ElfOper::NoOperand,
            src: ElfOper::Reg(RCX),
            extra: None,
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(*dest), ctx),
            src: ElfOper::Reg(RAX),
            size: var_size(dest),
        },
    ];
}

fn multiplication(dest: &VarId, left: &IrOp, right: &IrOp, ctx: &Context) -> Vec<Instruction> {
    return vec![
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RAX),
            src: convert_operand(left, ctx),
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RCX),
            src: convert_operand(right, ctx),
            size: var_size(dest),
        },
        Instruction::ComplexBinary {
            op: ComplexBinOp::Imul,
            dst: ElfOper::Reg(RAX),
            src: ElfOper::Reg(RCX),
            extra: None,
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(*dest), ctx),
            src: ElfOper::Reg(RAX),
            size: var_size(dest),
        },
    ];
}

/// The count of a shift is an imm8 or cl: it goes through rcx
fn shift(
    dest: &VarId,
    left: &IrOp,
    right: &IrOp,
    ctx: &Context,
    shift_op: ShiftOp,
) -> Vec<Instruction> {
    return vec![
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RAX),
            src: convert_operand(left, ctx),
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: ElfOper::Reg(RCX),
            src: convert_operand(right, ctx),
            size: var_size(dest),
        },
        Instruction::Shift {
            op: shift_op,
            dst: ElfOper::Reg(RAX),
            src: ElfOper::Reg(RCX),
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(*dest), ctx),
            src: ElfOper::Reg(RAX),
            size: var_size(dest),
        },
    ];
}

pub fn generate_binary(
    dest: &VarId,
    op: &BinaryOp,
    left: &IrOp,
    right: &IrOp,
    ctx: &Context,
) -> Vec<Instruction> {
    // x86 has no memory to memory operation: the computation goes through rax
    // shortcut: 8-byte operands only, use dest.1 for the size once 짧은 types reach the codegen
    return match op {
        BinaryOp::Add => simple_operation(dest, left, right, ctx, BinOp::Add),
        BinaryOp::Sub => simple_operation(dest, left, right, ctx, BinOp::Sub),
        BinaryOp::BitOr => simple_operation(dest, left, right, ctx, BinOp::Or),
        BinaryOp::BitXor => simple_operation(dest, left, right, ctx, BinOp::Xor),
        BinaryOp::BitAnd => simple_operation(dest, left, right, ctx, BinOp::And),
        BinaryOp::Mul => multiplication(dest, left, right, ctx),
        BinaryOp::Div => division(dest, left, right, ctx),
        BinaryOp::Shl => shift(dest, left, right, ctx, ShiftOp::Shl),
        // shortcut: arithmetic shift as for a signed value, Shr for 양수 once Binary carries the sign
        BinaryOp::Shr => shift(dest, left, right, ctx, ShiftOp::Sar),
        _ => unimplemented!(),
    };
}
