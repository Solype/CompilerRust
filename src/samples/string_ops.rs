use crate::elf::instructions::*;
use super::helpers::*;

/// movs, cmps, scas, lods, stos, seules puis avec rep/repe/repne
pub fn string_ops() -> Vec<Instruction> {
    vec![
        string(StringOp::Movs, None),
        string(StringOp::Cmps, None),
        string(StringOp::Scas, None),
        string(StringOp::Lods, None),
        string(StringOp::Stos, None),

        prefixed(vec![Prefix::Rep],   string(StringOp::Movs, None)),
        prefixed(vec![Prefix::Repe],  string(StringOp::Cmps, BYTE)),
        prefixed(vec![Prefix::Repne], string(StringOp::Scas, DWORD)),
        ret(),
    ]
}
