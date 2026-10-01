//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// lock add byte ptr [rip+my_data], 1
    lock_add_byte_rip_plus_my_data_1: prefixed(vec![Prefix::Lock], bin(BinOp::Add, mem(MemAddress::symbol("my_data")), imm(1), BYTE)) => [0xF0, 0x80, 0x05, 0x00, 0x00, 0x00, 0x00, 0x01];
    /// lock xadd qword ptr [rbx], rcx
    lock_xadd_qword_rbx_rcx: prefixed(vec![Prefix::Lock], complex(ComplexBinOp::Xadd, mem(at(RBX)), reg(RCX), None, QWORD)) => [0xF0, 0x48, 0x0F, 0xC1, 0x0B];
    /// lock cmpxchg dword ptr [rbx], ecx
    lock_cmpxchg_dword_rbx_ecx: prefixed(vec![Prefix::Lock], complex(ComplexBinOp::Cmpxchg, mem(at(RBX)), reg(RCX), None, DWORD)) => [0xF0, 0x0F, 0xB1, 0x0B];
    /// lock inc qword ptr [rbx]
    #[ignore = "BUG: panic `LOCK prefix not valid for instruction: Unary { op: Inc, dst: MemoryAddress(MemAddress { base: Some(Register { class: Gpr`"]
    lock_inc_qword_rbx: prefixed(vec![Prefix::Lock], unary(UnaryOp::Inc, mem(at(RBX)), QWORD)) => [0xF0, 0x48, 0xFF, 0x03];
    /// mov rax, qword ptr fs:[0x1000]
    #[ignore = "BUG: produit `mov rax,QWORD PTR fs:[rip+0x1000]`"]
    mov_rax_qword_fs_0x1000: prefixed(vec![Prefix::Fs], bin(BinOp::Mov, reg(RAX), mem(MemAddress::direct(0x1000).absolute()), QWORD)) => [0x64, 0x48, 0x8B, 0x04, 0x25, 0x00, 0x10, 0x00, 0x00];
    /// mov rax, qword ptr fs:[rbx]
    mov_rax_qword_fs_rbx: prefixed(vec![Prefix::Fs], bin(BinOp::Mov, reg(RAX), mem(at(RBX)), QWORD)) => [0x64, 0x48, 0x8B, 0x03];
    /// mov rax, qword ptr gs:[0x1000]
    #[ignore = "BUG: produit `mov rax,QWORD PTR gs:[rip+0x1000]`"]
    mov_rax_qword_gs_0x1000: prefixed(vec![Prefix::Gs], bin(BinOp::Mov, reg(RAX), mem(MemAddress::direct(0x1000).absolute()), QWORD)) => [0x65, 0x48, 0x8B, 0x04, 0x25, 0x00, 0x10, 0x00, 0x00];
    /// mov rax, qword ptr gs:[rbx]
    mov_rax_qword_gs_rbx: prefixed(vec![Prefix::Gs], bin(BinOp::Mov, reg(RAX), mem(at(RBX)), QWORD)) => [0x65, 0x48, 0x8B, 0x03];
    /// mov rax, qword ptr cs:[rbx]
    mov_rax_qword_cs_rbx: prefixed(vec![Prefix::Cs], bin(BinOp::Mov, reg(RAX), mem(at(RBX)), QWORD)) => [0x2E, 0x48, 0x8B, 0x03];
    /// mov rax, qword ptr ds:[rbx]
    mov_rax_qword_ds_rbx: prefixed(vec![Prefix::Ds], bin(BinOp::Mov, reg(RAX), mem(at(RBX)), QWORD)) => [0x48, 0x8B, 0x03] | [0x3E, 0x48, 0x8B, 0x03];
    /// mov rax, qword ptr es:[rbx]
    mov_rax_qword_es_rbx: prefixed(vec![Prefix::Es], bin(BinOp::Mov, reg(RAX), mem(at(RBX)), QWORD)) => [0x26, 0x48, 0x8B, 0x03];
    /// mov rax, qword ptr ss:[rbx]
    mov_rax_qword_ss_rbx: prefixed(vec![Prefix::Ss], bin(BinOp::Mov, reg(RAX), mem(at(RBX)), QWORD)) => [0x36, 0x48, 0x8B, 0x03];
}
