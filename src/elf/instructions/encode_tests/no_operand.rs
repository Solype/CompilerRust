//! Generated encoding tests: reference bytes produced by GNU as.
//! `#[ignore = "BUG: ..."]` marks a wrong encoding (see `cargo test -- --ignored`).

use super::*;

cases! {
    /// nop
    nop: nullary(UnaryOp::Nop) => [0x90];
    /// cbw
    cbw: nullary(UnaryOp::Cbw) => [0x66, 0x98];
    /// cwde
    cwde: nullary(UnaryOp::Cwde) => [0x98];
    /// cdqe
    cdqe: nullary(UnaryOp::Cdqe) => [0x48, 0x98];
    /// cqo
    cqo: nullary(UnaryOp::Cqo) => [0x48, 0x99];
    /// clc
    clc: nullary(UnaryOp::Clc) => [0xF8];
    /// stc
    stc: nullary(UnaryOp::Stc) => [0xF9];
    /// cmc
    cmc: nullary(UnaryOp::Cmc) => [0xF5];
    /// cld
    cld: nullary(UnaryOp::Cld) => [0xFC];
    /// std
    std: nullary(UnaryOp::Std) => [0xFD];
    /// cli
    cli: nullary(UnaryOp::Cli) => [0xFA];
    /// sti
    sti: nullary(UnaryOp::Sti) => [0xFB];
    /// lahf
    lahf: nullary(UnaryOp::Lahf) => [0x9F];
    /// sahf
    sahf: nullary(UnaryOp::Sahf) => [0x9E];
    /// pause
    pause: nullary(UnaryOp::Pause) => [0xF3, 0x90];
    /// fwait
    fwait: nullary(UnaryOp::Fwait) => [0x9B];
    /// ud2
    ud2: nullary(UnaryOp::Ud2) => [0x0F, 0x0B];
    /// hlt
    hlt: nullary(UnaryOp::Hlt) => [0xF4];
    /// cwd
    cwd: unary(UnaryOp::Cwd, Operand::NoOperand, WORD) => [0x66, 0x99];
    /// cdq
    cdq: unary(UnaryOp::Cdq, Operand::NoOperand, DWORD) => [0x99];
    /// nop 1 bytes (SDM)
    nop_1_bytes: Instruction::Nop(1) => [0x90];
    /// nop 2 bytes (SDM)
    nop_2_bytes: Instruction::Nop(2) => [0x66, 0x90];
    /// nop 3 bytes (SDM)
    nop_3_bytes: Instruction::Nop(3) => [0x0F, 0x1F, 0x00];
    /// nop 4 bytes (SDM)
    nop_4_bytes: Instruction::Nop(4) => [0x0F, 0x1F, 0x40, 0x00];
    /// nop 5 bytes (SDM)
    nop_5_bytes: Instruction::Nop(5) => [0x0F, 0x1F, 0x44, 0x00, 0x00];
    /// nop 6 bytes (SDM)
    nop_6_bytes: Instruction::Nop(6) => [0x66, 0x0F, 0x1F, 0x44, 0x00, 0x00];
    /// nop 7 bytes (SDM)
    nop_7_bytes: Instruction::Nop(7) => [0x0F, 0x1F, 0x80, 0x00, 0x00, 0x00, 0x00];
    /// nop 8 bytes (SDM)
    nop_8_bytes: Instruction::Nop(8) => [0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
    /// nop 9 bytes (SDM)
    nop_9_bytes: Instruction::Nop(9) => [0x66, 0x0F, 0x1F, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00];
}
