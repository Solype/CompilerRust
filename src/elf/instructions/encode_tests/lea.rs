//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// lea rax, [rbx]
    lea_rax_rbx: lea(RAX, at(RBX), QWORD) => [0x48, 0x8D, 0x03];
    /// lea rax, [rbx+8]
    lea_rax_rbx_plus_8: lea(RAX, at(RBX).disp(8), QWORD) => [0x48, 0x8D, 0x43, 0x08];
    /// lea rax, [rbp-16]
    lea_rax_rbp_minus_16: lea(RAX, at(RBP).disp(-16), QWORD) => [0x48, 0x8D, 0x45, 0xF0];
    /// lea rax, [rbx+0x1000]
    lea_rax_rbx_plus_0x1000: lea(RAX, at(RBX).disp(0x1000), QWORD) => [0x48, 0x8D, 0x83, 0x00, 0x10, 0x00, 0x00];
    /// lea rax, [rbp]
    lea_rax_rbp: lea(RAX, at(RBP), QWORD) => [0x48, 0x8D, 0x45, 0x00];
    /// lea rax, [rsp]
    lea_rax_rsp: lea(RAX, at(RSP), QWORD) => [0x48, 0x8D, 0x04, 0x24];
    /// lea rax, [r12]
    #[ignore = "BUG: produit `rex.W; .byte 0x8d; .byte 0x4`"]
    lea_rax_r12: lea(RAX, at(R12), QWORD) => [0x49, 0x8D, 0x04, 0x24];
    /// lea rax, [r13]
    #[ignore = "BUG: produit `rex.W; .byte 0x8d; .byte 0x5`"]
    lea_rax_r13: lea(RAX, at(R13), QWORD) => [0x49, 0x8D, 0x45, 0x00];
    /// lea rax, [rbx+rcx*4]
    lea_rax_rbx_plus_rcx_x_4: lea(RAX, at(RBX).index_scale(RCX, Scale::Four), QWORD) => [0x48, 0x8D, 0x04, 0x8B];
    /// lea rax, [rbx+rcx*8+16]
    lea_rax_rbx_plus_rcx_x_8_plus_16: lea(RAX, at(RBX).index_scale(RCX, Scale::Eight).disp(16), QWORD) => [0x48, 0x8D, 0x44, 0xCB, 0x10];
    /// lea rax, [rbp+rcx]
    #[ignore = "BUG: produit `rex.W; .byte 0x8d; add al,0xd`"]
    lea_rax_rbp_plus_rcx: lea(RAX, at(RBP).index(RCX), QWORD) => [0x48, 0x8D, 0x44, 0x0D, 0x00];
    /// lea rax, [rbx+r8*2]
    lea_rax_rbx_plus_r8_x_2: lea(RAX, at(RBX).index_scale(R8, Scale::Two), QWORD) => [0x4A, 0x8D, 0x04, 0x43];
    /// lea rax, [rcx*4+16]
    #[ignore = "BUG: produit `lea rax,[rbp+rcx*4+0x10]`"]
    lea_rax_rcx_x_4_plus_16: lea(RAX, MemAddress::new().index_scale(RCX, Scale::Four).disp(16), QWORD) => [0x48, 0x8D, 0x04, 0x8D, 0x10, 0x00, 0x00, 0x00];
    /// lea rax, [rip+my_data]
    lea_rax_rip_plus_my_data: lea(RAX, MemAddress::symbol("my_data"), QWORD) => [0x48, 0x8D, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// lea eax, [rbx+8]
    lea_eax_rbx_plus_8: lea(RAX, at(RBX).disp(8), DWORD) => [0x8D, 0x43, 0x08];
    /// lea r11, [rbx+8]
    lea_r11_rbx_plus_8: lea(R11, at(RBX).disp(8), QWORD) => [0x4C, 0x8D, 0x5B, 0x08];
    /// lea rax, [rip+my_data]
    lea_rax_rip_plus_my_data_2: lea(RAX, MemAddress::symbol("my_data"), None) => [0x48, 0x8D, 0x05, 0x00, 0x00, 0x00, 0x00];
}
