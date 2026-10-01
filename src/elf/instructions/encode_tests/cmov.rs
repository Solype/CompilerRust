//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// cmove eax, ebx
    cmove_eax_ebx: cmov(ConditionCode::E, RAX, reg(RBX), DWORD) => [0x0F, 0x44, 0xC3];
    /// cmovne eax, ebx
    cmovne_eax_ebx: cmov(ConditionCode::NE, RAX, reg(RBX), DWORD) => [0x0F, 0x45, 0xC3];
    /// cmovg eax, ebx
    cmovg_eax_ebx: cmov(ConditionCode::G, RAX, reg(RBX), DWORD) => [0x0F, 0x4F, 0xC3];
    /// cmovge eax, ebx
    cmovge_eax_ebx: cmov(ConditionCode::GE, RAX, reg(RBX), DWORD) => [0x0F, 0x4D, 0xC3];
    /// cmovl eax, ebx
    cmovl_eax_ebx: cmov(ConditionCode::L, RAX, reg(RBX), DWORD) => [0x0F, 0x4C, 0xC3];
    /// cmovle eax, ebx
    cmovle_eax_ebx: cmov(ConditionCode::LE, RAX, reg(RBX), DWORD) => [0x0F, 0x4E, 0xC3];
    /// cmova eax, ebx
    cmova_eax_ebx: cmov(ConditionCode::A, RAX, reg(RBX), DWORD) => [0x0F, 0x47, 0xC3];
    /// cmovae eax, ebx
    cmovae_eax_ebx: cmov(ConditionCode::AE, RAX, reg(RBX), DWORD) => [0x0F, 0x43, 0xC3];
    /// cmovb eax, ebx
    cmovb_eax_ebx: cmov(ConditionCode::B, RAX, reg(RBX), DWORD) => [0x0F, 0x42, 0xC3];
    /// cmovbe eax, ebx
    cmovbe_eax_ebx: cmov(ConditionCode::BE, RAX, reg(RBX), DWORD) => [0x0F, 0x46, 0xC3];
    /// cmovs eax, ebx
    cmovs_eax_ebx: cmov(ConditionCode::S, RAX, reg(RBX), DWORD) => [0x0F, 0x48, 0xC3];
    /// cmovns eax, ebx
    cmovns_eax_ebx: cmov(ConditionCode::NS, RAX, reg(RBX), DWORD) => [0x0F, 0x49, 0xC3];
    /// cmovo eax, ebx
    cmovo_eax_ebx: cmov(ConditionCode::O, RAX, reg(RBX), DWORD) => [0x0F, 0x40, 0xC3];
    /// cmovno eax, ebx
    cmovno_eax_ebx: cmov(ConditionCode::NO, RAX, reg(RBX), DWORD) => [0x0F, 0x41, 0xC3];
    /// cmovp eax, ebx
    cmovp_eax_ebx: cmov(ConditionCode::P, RAX, reg(RBX), DWORD) => [0x0F, 0x4A, 0xC3];
    /// cmovnp eax, ebx
    cmovnp_eax_ebx: cmov(ConditionCode::NP, RAX, reg(RBX), DWORD) => [0x0F, 0x4B, 0xC3];
    /// cmovg ax, bx
    cmovg_ax_bx: cmov(ConditionCode::G, RAX, reg(RBX), WORD) => [0x66, 0x0F, 0x4F, 0xC3];
    /// cmovg rax, rbx
    cmovg_rax_rbx: cmov(ConditionCode::G, RAX, reg(RBX), QWORD) => [0x48, 0x0F, 0x4F, 0xC3];
    /// cmove r9, r10
    cmove_r9_r10: cmov(ConditionCode::E, R9, reg(R10), QWORD) => [0x4D, 0x0F, 0x44, 0xCA];
    /// cmove eax, r10d
    cmove_eax_r10d: cmov(ConditionCode::E, RAX, reg(R10), DWORD) => [0x41, 0x0F, 0x44, 0xC2];
    /// cmovl ebx, dword ptr [rbx+8]
    cmovl_ebx_dword_rbx_plus_8: cmov(ConditionCode::L, RBX, mem(at(RBX).disp(8)), DWORD) => [0x0F, 0x4C, 0x5B, 0x08];
    /// cmovl rbx, qword ptr [rip+my_data]
    cmovl_rbx_qword_rip_plus_my_data: cmov(ConditionCode::L, RBX, mem(MemAddress::symbol("my_data")), QWORD) => [0x48, 0x0F, 0x4C, 0x1D, 0x00, 0x00, 0x00, 0x00];
}
