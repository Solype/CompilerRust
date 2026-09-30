use super::helpers::*;
use crate::elf::instructions::{register::*, *};

/// xchg, xadd, cmpxchg, et un `lock cmpxchg` qui réussit puis qui échoue
pub fn atomics() -> Vec<Instruction> {
    return vec![
        // eax = 123, ebx = 456 -> après xchg : eax = 456, ebx = 123
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
        // cmpxchg compare eax et [my_lock] :
        //   égaux      -> [my_lock] = src, ZF = 1
        //   différents -> eax = [my_lock], ZF = 0

        // Succès : my_lock == 0 == eax -> my_lock = 1
        bin(BinOp::Mov, reg(RAX), imm(0), DWORD),
        bin(BinOp::Mov, reg(RCX), imm(1), DWORD),
        prefixed(
            vec![Prefix::Lock],
            complex(ComplexBinOp::Cmpxchg, var("my_lock"), reg(RCX), None, DWORD),
        ),
        // Échec : my_lock vaut déjà 1 != eax -> eax = 1
        bin(BinOp::Mov, reg(RAX), imm(0), DWORD),
        bin(BinOp::Mov, reg(RDX), imm(2), DWORD),
        prefixed(
            vec![Prefix::Lock],
            complex(ComplexBinOp::Cmpxchg, var("my_lock"), reg(RDX), None, DWORD),
        ),
        ret(),
    ];
}
