use super::helpers::*;
use crate::elf::instructions::*;

/// syscall, int, sysenter
pub fn syscalls() -> Vec<Instruction> {
    return vec![
        sys(SysOp::Syscall),
        sys(SysOp::Int(0x80)),
        sys(SysOp::Sysenter),
        ret(),
    ];
}
