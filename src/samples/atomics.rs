use super::helpers::*;
use crate::elf::instructions::{register::*, *};

/// xchg, xadd, cmpxchg, and a `lock cmpxchg` that succeeds then fails
pub fn atomics() -> Vec<Instruction> {
    return vec![
        // eax = 123, ebx = 456 -> after xchg: eax = 456, ebx = 123
        bin(BinOp::Mov, reg(RAX), imm(123), DWORD),
        bin(BinOp::Mov, reg(RBX), imm(456), DWORD),
        bin(BinOp::Xchg, reg(RAX), reg(RBX), DWORD),
        bin(BinOp::Xchg, reg(R11), reg(RBX), DWORD),
        bin(BinOp::Xchg, reg(RCX), mem(at(RBX)), DWORD),
        // [my_data] <-> eax
        bin(BinOp::Xchg, var("my_data"), reg(RAX), DWORD),
        complex(ComplexBinOp::Xadd, reg(RAX), reg(RBX), None, DWORD),
        complex(ComplexBinOp::Xadd, mem(at(RAX)), reg(RBX), None, DWORD),
        complex(ComplexBinOp::Cmpxchg, mem(at(RAX)), reg(RBX), None, DWORD),
        prefixed(
            vec![Prefix::Lock],
            complex(ComplexBinOp::Cmpxchg, mem(at(RAX)), reg(RBX), None, DWORD),
        ),
        // cmpxchg compares eax with [my_lock]:
        //   equal     -> [my_lock] = src, ZF = 1
        //   different -> eax = [my_lock], ZF = 0

        // Success: my_lock == 0 == eax -> my_lock = 1
        bin(BinOp::Mov, reg(RAX), imm(0), DWORD),
        bin(BinOp::Mov, reg(RCX), imm(1), DWORD),
        prefixed(
            vec![Prefix::Lock],
            complex(ComplexBinOp::Cmpxchg, var("my_lock"), reg(RCX), None, DWORD),
        ),
        // Failure: my_lock is already 1 != eax -> eax = 1
        bin(BinOp::Mov, reg(RAX), imm(0), DWORD),
        bin(BinOp::Mov, reg(RDX), imm(2), DWORD),
        prefixed(
            vec![Prefix::Lock],
            complex(ComplexBinOp::Cmpxchg, var("my_lock"), reg(RDX), None, DWORD),
        ),
        ret(),
    ];
}
