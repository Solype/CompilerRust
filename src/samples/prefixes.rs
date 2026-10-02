use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// Segment and lock prefixes (rep/repe/repne are in string_ops)
pub fn prefixes() -> Vec<Instruction> {
    vec![
        prefixed(
            vec![Prefix::Cs, Prefix::Ds, Prefix::Es, Prefix::Ss, Prefix::Gs, Prefix::Fs],
            bin(BinOp::Mov, reg(RAX), mem(MemAddress::new().disp(0x28)), QWORD),
        ),

        // Same add with and without lock
        prefixed(vec![Prefix::Lock], bin(BinOp::Add, var("my_data"), imm(1), BYTE)),
        bin(BinOp::Add, var("my_data"), imm(1), BYTE),
        ret(),
    ]
}
