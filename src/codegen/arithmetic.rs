use super::generation::Context;
use super::instructions::convert_operand;
use crate::codegen::utils::{load_operand, var_size};
use crate::converter::ir::structs::{Operand as IrOp, VarId};
use crate::elf::instructions::register::{RAX, RCX, RDX, Register, XMM0};
use crate::elf::instructions::{
    BinOp, ComplexBinOp, Instruction, Operand as ElfOper, ShiftOp, UnaryOp as ElfUnOp,
};
use crate::parser::postparser::structs::{BinaryOp, UnaryOp};

/// `dest = left op right` for an op that takes its source as it is (add, sub, and, or, xor):
/// `mov rax, left ; op rax, right ; mov [dest], rax`
fn simple_operation(
    dest: &VarId,
    left: &IrOp,
    right: &IrOp,
    ctx: &Context,
    bin_op: BinOp,
) -> Vec<Instruction> {
    let instrs: Vec<Instruction> = vec![
        load_operand(RAX, left, ctx, var_size(dest)),
        Instruction::Binary {
            op: bin_op,
            dst: ElfOper::Reg(RAX),
            src: convert_operand(right, ctx),
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(dest.clone()), ctx),
            src: ElfOper::Reg(RAX),
            size: var_size(dest),
        },
    ];
    return instrs;
}

/// `dest = left op right` on 소수 (addsd, subsd, mulsd, divsd; the ss forms on 4 bytes):
/// `movsd xmm0, left ; op xmm0, right ; movsd [dest], xmm0`
fn float_operation(
    dest: &VarId,
    left: &IrOp,
    right: &IrOp,
    ctx: &Context,
    bin_op: BinOp,
) -> Vec<Instruction> {
    return vec![
        load_operand(XMM0, left, ctx, var_size(dest)),
        Instruction::Binary {
            op: bin_op,
            dst: ElfOper::Reg(XMM0),
            src: convert_operand(right, ctx),
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::StoreF,
            dst: convert_operand(&IrOp::Var(dest.clone()), ctx),
            src: ElfOper::Reg(XMM0),
            size: var_size(dest),
        },
    ];
}

/// `dest = left / right` (result rax) or `left % right` (result rdx), signed: `mov rax, left ;
/// mov rcx, right ; cqo (cdq on 4 bytes) ; idiv rcx ; mov [dest], result`. idiv divides rdx:rax:
/// quotient in rax, remainder in rdx
fn division(
    dest: &VarId,
    left: &IrOp,
    right: &IrOp,
    ctx: &Context,
    result: Register,
) -> Vec<Instruction> {
    return vec![
        load_operand(RAX, left, ctx, var_size(dest)),
        load_operand(RCX, right, ctx, var_size(dest)),
        Instruction::Unary {
            op: if dest.ty.size() == 4 {
                ElfUnOp::Cdq
            } else {
                ElfUnOp::Cqo
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
            dst: convert_operand(&IrOp::Var(dest.clone()), ctx),
            src: ElfOper::Reg(result),
            size: var_size(dest),
        },
    ];
}

/// `dest = left * right`: `mov rax, left ; mov rcx, right ; imul rax, rcx ; mov [dest], rax`.
/// right goes through rcx, as `imul r, r/m` takes no immediate; the low bits are the same signed
/// or not
fn multiplication(dest: &VarId, left: &IrOp, right: &IrOp, ctx: &Context) -> Vec<Instruction> {
    return vec![
        load_operand(RAX, left, ctx, var_size(dest)),
        load_operand(RCX, right, ctx, var_size(dest)),
        Instruction::ComplexBinary {
            op: ComplexBinOp::Imul,
            dst: ElfOper::Reg(RAX),
            src: ElfOper::Reg(RCX),
            extra: None,
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(dest.clone()), ctx),
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
        load_operand(RAX, left, ctx, var_size(dest)),
        load_operand(RCX, right, ctx, var_size(dest)),
        Instruction::Shift {
            op: shift_op,
            dst: ElfOper::Reg(RAX),
            src: ElfOper::Reg(RCX),
            size: var_size(dest),
        },
        Instruction::Binary {
            op: BinOp::Mov,
            dst: convert_operand(&IrOp::Var(dest.clone()), ctx),
            src: ElfOper::Reg(RAX),
            size: var_size(dest),
        },
    ];
}

/// The instructions of `dest = left op right`, sized by `dest`; panics on an op not done yet
/// (comparisons, 그리고, 또는)
pub fn generate_binary(
    dest: &VarId,
    op: &BinaryOp,
    left: &IrOp,
    right: &IrOp,
    ctx: &Context,
) -> Vec<Instruction> {
    // the converter only lets + - * / reach a 소수
    if dest.ty.is_float() {
        return match op {
            BinaryOp::Add => float_operation(dest, left, right, ctx, BinOp::AddF),
            BinaryOp::Sub => float_operation(dest, left, right, ctx, BinOp::SubF),
            BinaryOp::Mul => float_operation(dest, left, right, ctx, BinOp::MulF),
            BinaryOp::Div => float_operation(dest, left, right, ctx, BinOp::DivF),
            _ => unimplemented!(),
        };
    }
    // x86 has no memory to memory operation: the computation goes through rax
    return match op {
        BinaryOp::Add => simple_operation(dest, left, right, ctx, BinOp::Add),
        BinaryOp::Sub => simple_operation(dest, left, right, ctx, BinOp::Sub),
        BinaryOp::BitOr => simple_operation(dest, left, right, ctx, BinOp::Or),
        BinaryOp::BitXor => simple_operation(dest, left, right, ctx, BinOp::Xor),
        BinaryOp::BitAnd => simple_operation(dest, left, right, ctx, BinOp::And),
        BinaryOp::Mul => multiplication(dest, left, right, ctx),
        BinaryOp::Div => division(dest, left, right, ctx, RAX),
        BinaryOp::Rem => division(dest, left, right, ctx, RDX),
        BinaryOp::Shl => shift(dest, left, right, ctx, ShiftOp::Shl),
        // shortcut: arithmetic shift as for a signed value, Shr for 양수 once Binary carries the sign
        BinaryOp::Shr => shift(dest, left, right, ctx, ShiftOp::Sar),
        _ => unimplemented!(),
    };
}

/// `dest = op src`: `mov rax, src ; neg rax` (-x), `not rax` (~x) or `xor al, 1` (logical not:
/// a 논리 is 0 or 1, ~ would give 0xFE / 0xFF), then `mov [dest], rax`. -x of a 소수 flips its
/// sign bit: `mov rcx, 0x8000000000000000 ; xor rax, rcx`
pub fn generate_unary(dest: &VarId, op: &UnaryOp, src: &IrOp, ctx: &Context) -> Vec<Instruction> {
    let operation = match op {
        // not xorpd: its mask in memory would have to be 16 bytes, 16-byte aligned
        UnaryOp::Neg if dest.ty.is_float() => vec![
            Instruction::Binary {
                op: BinOp::Mov,
                dst: ElfOper::Reg(RCX),
                src: ElfOper::Imm(if dest.ty.size() == 4 { i32::MIN as i64 } else { i64::MIN }),
                size: var_size(dest),
            },
            Instruction::Binary {
                op: BinOp::Xor,
                dst: ElfOper::Reg(RAX),
                src: ElfOper::Reg(RCX),
                size: var_size(dest),
            },
        ],
        UnaryOp::Neg => vec![Instruction::Unary {
            op: ElfUnOp::Neg,
            dst: ElfOper::Reg(RAX),
            size: var_size(dest),
        }],
        UnaryOp::BitNot => vec![Instruction::Unary {
            op: ElfUnOp::Not,
            dst: ElfOper::Reg(RAX),
            size: var_size(dest),
        }],
        UnaryOp::Not => vec![Instruction::Binary {
            op: BinOp::Xor,
            dst: ElfOper::Reg(RAX),
            src: ElfOper::Imm(1),
            size: var_size(dest),
        }],
    };
    let mut instrs = vec![load_operand(RAX, src, ctx, var_size(dest))];
    instrs.extend(operation);
    instrs.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: convert_operand(&IrOp::Var(dest.clone()), ctx),
        src: ElfOper::Reg(RAX),
        size: var_size(dest),
    });
    return instrs;
}
