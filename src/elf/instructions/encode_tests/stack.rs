//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// push rax
    push_rax: stack(StackOp::Push, reg(RAX), None) => [0x50] | [0x48, 0x50];
    /// pop rax
    pop_rax: stack(StackOp::Pop, reg(RAX), None) => [0x58] | [0x48, 0x58];
    /// push rbx
    push_rbx: stack(StackOp::Push, reg(RBX), None) => [0x53] | [0x48, 0x53];
    /// pop rbx
    pop_rbx: stack(StackOp::Pop, reg(RBX), None) => [0x5B] | [0x48, 0x5B];
    /// push r12
    push_r12: stack(StackOp::Push, reg(R12), None) => [0x41, 0x54] | [0x49, 0x54];
    /// pop r12
    pop_r12: stack(StackOp::Pop, reg(R12), None) => [0x41, 0x5C] | [0x49, 0x5C];
    /// push r15
    push_r15: stack(StackOp::Push, reg(R15), None) => [0x41, 0x57] | [0x49, 0x57];
    /// pop r15
    pop_r15: stack(StackOp::Pop, reg(R15), None) => [0x41, 0x5F] | [0x49, 0x5F];
    /// push ax
    push_ax: stack(StackOp::Push, reg(RAX), WORD) => [0x66, 0x50];
    /// pop ax
    pop_ax: stack(StackOp::Pop, reg(RAX), WORD) => [0x66, 0x58];
    /// push 5
    push_5: stack(StackOp::Push, imm(5), None) => [0x6A, 0x05];
    /// push -1
    push_minus_1: stack(StackOp::Push, imm(-1), None) => [0x6A, 0xFF];
    /// push 0x12345678
    push_0x12345678: stack(StackOp::Push, imm(0x12345678), None) => [0x68, 0x78, 0x56, 0x34, 0x12];
    /// push qword ptr [rbx+8]
    push_qword_rbx_plus_8: stack(StackOp::Push, mem(at(RBX).disp(8)), None) => [0xFF, 0x73, 0x08] | [0x48, 0xFF, 0x73, 0x08];
    /// pop qword ptr [rbx+8]
    pop_qword_rbx_plus_8: stack(StackOp::Pop, mem(at(RBX).disp(8)), None) => [0x8F, 0x43, 0x08] | [0x48, 0x8F, 0x43, 0x08];
    /// push qword ptr [r12]
    #[ignore = "BUG: produit `rex.W; .byte 0xff; .byte 0x34`"]
    push_qword_r12: stack(StackOp::Push, mem(at(R12)), None) => [0x41, 0xFF, 0x34, 0x24];
    /// push qword ptr [rip+my_data]
    push_qword_rip_plus_my_data: stack(StackOp::Push, mem(MemAddress::symbol("my_data")), None) => [0xFF, 0x35, 0x00, 0x00, 0x00, 0x00] | [0x48, 0xFF, 0x35, 0x00, 0x00, 0x00, 0x00];
    /// pushfq
    pushfq: stack(StackOp::Pushf, Operand::NoOperand, None) => [0x9C];
    /// popfq
    popfq: stack(StackOp::Popf, Operand::NoOperand, None) => [0x9D];
    /// pushfw
    pushfw: stack(StackOp::Pushf, Operand::NoOperand, WORD) => [0x66, 0x9C];
    /// popfw
    popfw: stack(StackOp::Popf, Operand::NoOperand, WORD) => [0x66, 0x9D];
    /// leave
    leave: stack(StackOp::Leave, Operand::NoOperand, None) => [0xC9];
    /// enter 32, 0
    enter_32_0: stack(StackOp::Enter(0), imm(32), None) => [0xC8, 0x20, 0x00, 0x00];
    /// enter 0x100, 1
    enter_0x100_1: stack(StackOp::Enter(1), imm(0x100), None) => [0xC8, 0x00, 0x01, 0x01];
}
