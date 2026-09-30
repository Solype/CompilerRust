use crate::elf::instructions::{register::*, *};
use super::helpers::*;

/// Préfixes de segment et lock (rep/repe/repne sont dans string_ops)
pub fn prefixes() -> Vec<Instruction> {
    vec![
        prefixed(
            vec![Prefix::Cs, Prefix::Ds, Prefix::Es, Prefix::Ss, Prefix::Gs, Prefix::Fs],
            bin(BinOp::Mov, reg(RAX), mem(MemAddress::new().disp(0x28)), QWORD),
        ),

        // Même add avec et sans lock
        prefixed(vec![Prefix::Lock], bin(BinOp::Add, var("my_data"), imm(1), BYTE)),
        bin(BinOp::Add, var("my_data"), imm(1), BYTE),
        ret(),
    ]
}
