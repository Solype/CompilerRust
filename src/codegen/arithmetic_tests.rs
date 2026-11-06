use super::arithmetic::generate_binary;
use super::generation::Context;
use crate::converter::ir::structs::{Operand as IrOp, VarId};
use crate::elf::instructions::register::{RAX, RCX, RDX, Register};
use crate::elf::instructions::{
    BinOp, ComplexBinOp, ConditionCode, Instruction, Operand as ElfOper, Size, UnaryOp,
};
use crate::parser::postparser::structs::{BinaryOp, Type};

fn float_ctx<'a>(floats: &'a [(f64, bool)], float_names: &'a [String]) -> Context<'a> {
    Context {
        return_type: None,
        offsets: vec![8, 16, 17],
        data_names: &[],
        float_names,
        floats,
        labels: vec![],
    }
}

fn float_eq_or_ne(op: BinaryOp) -> Vec<Instruction> {
    let floats = [(f64::NAN, false), (1.0, false)];
    let names = ["f0".into(), "f1".into()];
    let ctx = float_ctx(&floats, &names);
    let dest = VarId::new(2, Type::Bool);
    generate_binary(&dest, &op, &IrOp::Float(0), &IrOp::Float(1), &ctx)
}

fn setcc_ops(instrs: &[Instruction]) -> Vec<ConditionCode> {
    instrs
        .iter()
        .filter_map(|i| match i {
            Instruction::SetCC { op, .. } => Some(*op),
            _ => None,
        })
        .collect()
}

fn is_cc(op: ConditionCode, want: ConditionCode) -> bool {
    matches!(
        (op, want),
        (ConditionCode::E, ConditionCode::E)
            | (ConditionCode::NE, ConditionCode::NE)
            | (ConditionCode::NP, ConditionCode::NP)
            | (ConditionCode::P, ConditionCode::P)
    )
}

#[test]
fn float_eq_also_requires_not_parity() {
    let instrs = float_eq_or_ne(BinaryOp::Eq);
    let ops = setcc_ops(&instrs);
    assert_eq!(ops.len(), 2, "{ops:?}");
    assert!(is_cc(ops[0], ConditionCode::E), "{ops:?}");
    assert!(is_cc(ops[1], ConditionCode::NP), "{ops:?}");
    assert!(
        instrs
            .iter()
            .any(|i| matches!(i, Instruction::Binary { op: BinOp::And, .. })),
        "expected and al, cl after sete/setnp: {instrs:?}"
    );
}

#[test]
fn float_ne_also_accepts_parity() {
    let instrs = float_eq_or_ne(BinaryOp::NotEq);
    let ops = setcc_ops(&instrs);
    assert_eq!(ops.len(), 2, "{ops:?}");
    assert!(is_cc(ops[0], ConditionCode::NE), "{ops:?}");
    assert!(is_cc(ops[1], ConditionCode::P), "{ops:?}");
    assert!(
        instrs
            .iter()
            .any(|i| matches!(i, Instruction::Binary { op: BinOp::Or, .. })),
        "expected or al, cl after setne/setp: {instrs:?}"
    );
}

#[test]
fn integer_eq_does_not_read_parity() {
    let ctx = Context {
        return_type: None,
        offsets: vec![8, 16, 17],
        data_names: &[],
        float_names: &[],
        floats: &[],
        labels: vec![],
    };
    let dest = VarId::new(2, Type::Bool);
    let left = VarId::new(
        0,
        Type::Int {
            short: false,
            unsigned: false,
        },
    );
    let right = VarId::new(
        1,
        Type::Int {
            short: false,
            unsigned: false,
        },
    );
    let instrs = generate_binary(
        &dest,
        &BinaryOp::Eq,
        &IrOp::Var(left),
        &IrOp::Var(right),
        &ctx,
    );
    let ops = setcc_ops(&instrs);
    assert_eq!(ops.len(), 1, "{ops:?}");
    assert!(is_cc(ops[0], ConditionCode::E), "{ops:?}");
}

/// `var2 = var0 op var1`, all three 정수 of 8 bytes
fn int_division(op: BinaryOp, unsigned: bool) -> Vec<Instruction> {
    let ty = Type::Int {
        short: false,
        unsigned,
    };
    let ctx = Context {
        return_type: None,
        offsets: vec![8, 16, 24],
        data_names: &[],
        float_names: &[],
        floats: &[],
        labels: vec![],
    };
    let left = IrOp::Var(VarId::new(0, ty.clone()));
    let right = IrOp::Var(VarId::new(1, ty.clone()));
    generate_binary(&VarId::new(2, ty), &op, &left, &right, &ctx)
}

/// `xor rdx, rdx ; div rcx ; mov [dest], result`, in a form the encoder accepts
fn assert_unsigned_division(instrs: &[Instruction], result: Register) {
    let [_, _, xor, div, store] = instrs else {
        panic!("expected 5 instructions: {instrs:?}");
    };
    assert!(
        matches!(xor, Instruction::Binary { op: BinOp::Xor, dst: ElfOper::Reg(d), src: ElfOper::Reg(s), .. } if *d == RDX && *s == RDX),
        "{xor:?}"
    );
    assert!(
        matches!(div, Instruction::ComplexBinary { op: ComplexBinOp::Div, dst: ElfOper::NoOperand, src: ElfOper::Reg(s), extra: None, .. } if *s == RCX),
        "{div:?}"
    );
    assert!(
        matches!(store, Instruction::Binary { op: BinOp::Mov, dst: ElfOper::MemoryAddress(_), src: ElfOper::Reg(s), .. } if *s == result),
        "{store:?}"
    );
    for instr in instrs {
        instr.encode(Size::U64);
    }
}

#[test]
fn unsigned_div_stores_the_quotient_of_div() {
    assert_unsigned_division(&int_division(BinaryOp::Div, true), RAX);
}

#[test]
fn unsigned_rem_stores_the_remainder_of_div() {
    assert_unsigned_division(&int_division(BinaryOp::Rem, true), RDX);
}

#[test]
fn signed_div_still_uses_cqo_and_idiv() {
    let instrs = int_division(BinaryOp::Div, false);
    assert!(
        instrs.iter().any(|i| matches!(
            i,
            Instruction::Unary {
                op: UnaryOp::Cqo,
                ..
            }
        )),
        "{instrs:?}"
    );
    assert!(
        instrs.iter().any(|i| matches!(
            i,
            Instruction::ComplexBinary {
                op: ComplexBinOp::Idiv,
                ..
            }
        )),
        "{instrs:?}"
    );
}
