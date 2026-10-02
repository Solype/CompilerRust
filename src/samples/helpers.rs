//! Shortcuts to write the test programs without repeating
//! `Instruction::X { ... }` and `Operand::Y(...)` on every line.

use crate::elf::instructions::{register::*, *};

pub const BYTE: Option<Size> = Some(Size::U8);
pub const WORD: Option<Size> = Some(Size::U16);
pub const DWORD: Option<Size> = Some(Size::U32);
pub const QWORD: Option<Size> = Some(Size::U64);

/// Every condition, for the jcc / setcc / cmovcc families
pub const ALL_CC: [ConditionCode; 16] = {
    use ConditionCode::*;
    [E, NE, G, L, GE, LE, A, B, AE, BE, S, NS, O, NO, P, NP]
};

/// Registers used in turn in the test loops
pub const GPRS: [Register; 4] = [RAX, RBX, RCX, RDX];

// =========================================================
// Operands
// =========================================================

pub fn reg(r: Register) -> Operand {
    Operand::Reg(r)
}

pub fn imm(value: i64) -> Operand {
    Operand::Imm(value)
}

pub fn sym(name: &str) -> Operand {
    Operand::Sym(name.to_string())
}

/// `[base]`, to extend with `.disp()`, `.index()`, ...
pub fn at(base: Register) -> MemAddress {
    MemAddress::new().base(base)
}

pub fn mem(addr: MemAddress) -> Operand {
    Operand::MemoryAddress(addr)
}

/// Memory access to a global variable: `[rip + name]`
pub fn var(name: &str) -> Operand {
    mem(MemAddress::symbol(name))
}

// =========================================================
// Instructions
// =========================================================

pub fn bin(op: BinOp, dst: Operand, src: Operand, size: Option<Size>) -> Instruction {
    Instruction::Binary { op, dst, src, size }
}

pub fn complex(op: ComplexBinOp, dst: Operand, src: Operand, extra: Option<Operand>, size: Option<Size>) -> Instruction {
    Instruction::ComplexBinary { op, dst, src, extra, size }
}

pub fn unary(op: UnaryOp, dst: Operand, size: Option<Size>) -> Instruction {
    Instruction::Unary { op, dst, size }
}

/// Instruction without operand (`cli`, `hlt`, `cqo`, ...)
pub fn nullary(op: UnaryOp) -> Instruction {
    unary(op, Operand::NoOperand, None)
}

pub fn stack(op: StackOp, value: Operand, size: Option<Size>) -> Instruction {
    Instruction::Stack { op, value, size }
}

pub fn bit(op: BitOp, dst: Operand, src: Operand, size: Option<Size>) -> Instruction {
    Instruction::Bit { op, dst, src, size }
}

pub fn bitscan(op: BitScanOp, dst: Register, src: Operand, size: Option<Size>) -> Instruction {
    Instruction::BitScan { op, dst, src, size }
}

pub fn convert(op: ConvOp, dst: Register, src: Operand, size: Option<Size>) -> Instruction {
    Instruction::Convert { op, dst, src, size }
}

/// `src_size` is the source size, `size` the destination size
pub fn extend(op: ExtendOp, dst: Register, src: Operand, src_size: Option<Size>, size: Option<Size>) -> Instruction {
    Instruction::Extend { op, dst, src, src_size, size }
}

pub fn shift(op: ShiftOp, dst: Operand, src: Operand, size: Option<Size>) -> Instruction {
    Instruction::Shift { op, dst, src, size }
}

pub fn cmov(cc: ConditionCode, dst: Register, src: Operand, size: Option<Size>) -> Instruction {
    Instruction::CMovCC { cc, dst, src, size }
}

pub fn setcc(cc: ConditionCode, dst: Operand) -> Instruction {
    Instruction::SetCC { op: cc, dst }
}

pub fn lea(dst: Register, src: MemAddress, size: Option<Size>) -> Instruction {
    Instruction::Lea { dst, src: mem(src), size }
}

pub fn string(op: StringOp, size: Option<Size>) -> Instruction {
    Instruction::String { op, size }
}

pub fn sys(op: SysOp) -> Instruction {
    Instruction::Sys { op }
}

pub fn prefixed(prefix: Vec<Prefix>, ins: Instruction) -> Instruction {
    Instruction::Prefix { prefix, ins: Box::new(ins) }
}

pub fn label(id: LabelId) -> Instruction {
    Instruction::Label(id)
}

/// `target`: a symbol name (`&str`) or a `LabelId`
pub fn ctrl(op: CtrlOp, target: impl Into<Operand>) -> Instruction {
    Instruction::Ctrl { op, target: target.into() }
}

pub fn jmp(target: impl Into<Operand>) -> Instruction {
    ctrl(CtrlOp::Jmp, target)
}

pub fn jcc(cc: ConditionCode, target: impl Into<Operand>) -> Instruction {
    ctrl(CtrlOp::JmpCC(cc), target)
}

pub fn call(target: impl Into<Operand>) -> Instruction {
    ctrl(CtrlOp::Call, target)
}

/// `jmp reg` / `jmp [mem]`
pub fn jmp_indirect(target: Operand) -> Instruction {
    Instruction::Ctrl { op: CtrlOp::Jmp, target }
}

/// `call reg` / `call [mem]`
pub fn call_indirect(target: Operand) -> Instruction {
    Instruction::Ctrl { op: CtrlOp::Call, target }
}

pub fn ret() -> Instruction {
    Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand }
}

pub fn iret() -> Instruction {
    Instruction::Ctrl { op: CtrlOp::IRet, target: Operand::NoOperand }
}
