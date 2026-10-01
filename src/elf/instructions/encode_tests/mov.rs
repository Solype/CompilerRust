//! Tests d'encodage générés : octets de référence produits par GNU as.
//! `#[ignore = "BUG: ..."]` marque un encodage faux (voir `cargo test -- --ignored`).

use super::*;

cases! {
    /// mov al, 0x12
    mov_al_0x12: bin(BinOp::Mov, reg(RAX), imm(0x12), BYTE) => [0xB0, 0x12];
    /// mov bl, 0x12
    mov_bl_0x12: bin(BinOp::Mov, reg(RBX), imm(0x12), BYTE) => [0xB3, 0x12];
    /// mov r8b, 0x12
    mov_r8b_0x12: bin(BinOp::Mov, reg(R8), imm(0x12), BYTE) => [0x41, 0xB0, 0x12];
    /// mov r15b, 0x12
    mov_r15b_0x12: bin(BinOp::Mov, reg(R15), imm(0x12), BYTE) => [0x41, 0xB7, 0x12];
    /// mov ax, 0x1234
    mov_ax_0x1234: bin(BinOp::Mov, reg(RAX), imm(0x1234), WORD) => [0x66, 0xB8, 0x34, 0x12];
    /// mov bx, 0x1234
    mov_bx_0x1234: bin(BinOp::Mov, reg(RBX), imm(0x1234), WORD) => [0x66, 0xBB, 0x34, 0x12];
    /// mov r8w, 0x1234
    mov_r8w_0x1234: bin(BinOp::Mov, reg(R8), imm(0x1234), WORD) => [0x66, 0x41, 0xB8, 0x34, 0x12];
    /// mov r15w, 0x1234
    mov_r15w_0x1234: bin(BinOp::Mov, reg(R15), imm(0x1234), WORD) => [0x66, 0x41, 0xBF, 0x34, 0x12];
    /// mov eax, 0x12345678
    mov_eax_0x12345678: bin(BinOp::Mov, reg(RAX), imm(0x12345678), DWORD) => [0xB8, 0x78, 0x56, 0x34, 0x12];
    /// mov ebx, 0x12345678
    mov_ebx_0x12345678: bin(BinOp::Mov, reg(RBX), imm(0x12345678), DWORD) => [0xBB, 0x78, 0x56, 0x34, 0x12];
    /// mov r8d, 0x12345678
    mov_r8d_0x12345678: bin(BinOp::Mov, reg(R8), imm(0x12345678), DWORD) => [0x41, 0xB8, 0x78, 0x56, 0x34, 0x12];
    /// mov r15d, 0x12345678
    mov_r15d_0x12345678: bin(BinOp::Mov, reg(R15), imm(0x12345678), DWORD) => [0x41, 0xBF, 0x78, 0x56, 0x34, 0x12];
    /// mov rax, 0x123456789abcdef0
    mov_rax_0x123456789abcdef0: bin(BinOp::Mov, reg(RAX), imm(0x123456789abcdef0), QWORD) => [0x48, 0xB8, 0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12];
    /// mov rbx, 0x123456789abcdef0
    mov_rbx_0x123456789abcdef0: bin(BinOp::Mov, reg(RBX), imm(0x123456789abcdef0), QWORD) => [0x48, 0xBB, 0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12];
    /// mov r8, 0x123456789abcdef0
    mov_r8_0x123456789abcdef0: bin(BinOp::Mov, reg(R8), imm(0x123456789abcdef0), QWORD) => [0x49, 0xB8, 0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12];
    /// mov r15, 0x123456789abcdef0
    mov_r15_0x123456789abcdef0: bin(BinOp::Mov, reg(R15), imm(0x123456789abcdef0), QWORD) => [0x49, 0xBF, 0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12];
    /// mov rdx, 0x401000
    mov_rdx_0x401000: bin(BinOp::Mov, reg(RDX), imm(0x401000), None) => [0x48, 0xC7, 0xC2, 0x00, 0x10, 0x40, 0x00] | [0x48, 0xBA, 0x00, 0x10, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00];
    /// mov rdx, -1
    mov_rdx_minus_1: bin(BinOp::Mov, reg(RDX), imm(-1), None) => [0x48, 0xC7, 0xC2, 0xFF, 0xFF, 0xFF, 0xFF] | [0x48, 0xBA, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    /// mov bl, al
    mov_bl_al: bin(BinOp::Mov, reg(RBX), reg(RAX), BYTE) => [0x88, 0xC3] | [0x8A, 0xD8];
    /// mov r11b, bl
    mov_r11b_bl: bin(BinOp::Mov, reg(R11), reg(RBX), BYTE) => [0x41, 0x88, 0xDB] | [0x44, 0x8A, 0xDB];
    /// mov al, r9b
    mov_al_r9b: bin(BinOp::Mov, reg(RAX), reg(R9), BYTE) => [0x44, 0x88, 0xC8] | [0x41, 0x8A, 0xC1];
    /// mov r12b, r13b
    mov_r12b_r13b: bin(BinOp::Mov, reg(R12), reg(R13), BYTE) => [0x45, 0x88, 0xEC] | [0x45, 0x8A, 0xE5];
    /// mov bx, ax
    mov_bx_ax: bin(BinOp::Mov, reg(RBX), reg(RAX), WORD) => [0x66, 0x89, 0xC3] | [0x66, 0x8B, 0xD8];
    /// mov r11w, bx
    mov_r11w_bx: bin(BinOp::Mov, reg(R11), reg(RBX), WORD) => [0x66, 0x41, 0x89, 0xDB] | [0x66, 0x44, 0x8B, 0xDB];
    /// mov ax, r9w
    mov_ax_r9w: bin(BinOp::Mov, reg(RAX), reg(R9), WORD) => [0x66, 0x44, 0x89, 0xC8] | [0x66, 0x41, 0x8B, 0xC1];
    /// mov r12w, r13w
    mov_r12w_r13w: bin(BinOp::Mov, reg(R12), reg(R13), WORD) => [0x66, 0x45, 0x89, 0xEC] | [0x66, 0x45, 0x8B, 0xE5];
    /// mov ebx, eax
    mov_ebx_eax: bin(BinOp::Mov, reg(RBX), reg(RAX), DWORD) => [0x89, 0xC3] | [0x8B, 0xD8];
    /// mov r11d, ebx
    mov_r11d_ebx: bin(BinOp::Mov, reg(R11), reg(RBX), DWORD) => [0x41, 0x89, 0xDB] | [0x44, 0x8B, 0xDB];
    /// mov eax, r9d
    mov_eax_r9d: bin(BinOp::Mov, reg(RAX), reg(R9), DWORD) => [0x44, 0x89, 0xC8] | [0x41, 0x8B, 0xC1];
    /// mov r12d, r13d
    mov_r12d_r13d: bin(BinOp::Mov, reg(R12), reg(R13), DWORD) => [0x45, 0x89, 0xEC] | [0x45, 0x8B, 0xE5];
    /// mov rbx, rax
    mov_rbx_rax: bin(BinOp::Mov, reg(RBX), reg(RAX), QWORD) => [0x48, 0x89, 0xC3] | [0x48, 0x8B, 0xD8];
    /// mov r11, rbx
    mov_r11_rbx: bin(BinOp::Mov, reg(R11), reg(RBX), QWORD) => [0x49, 0x89, 0xDB] | [0x4C, 0x8B, 0xDB];
    /// mov rax, r9
    mov_rax_r9: bin(BinOp::Mov, reg(RAX), reg(R9), QWORD) => [0x4C, 0x89, 0xC8] | [0x49, 0x8B, 0xC1];
    /// mov r12, r13
    mov_r12_r13: bin(BinOp::Mov, reg(R12), reg(R13), QWORD) => [0x4D, 0x89, 0xEC] | [0x4D, 0x8B, 0xE5];
    /// mov sil, al
    #[ignore = "BUG: produit `mov dh,al`"]
    mov_sil_al: bin(BinOp::Mov, reg(RSI), reg(RAX), BYTE) => [0x40, 0x88, 0xC6];
    /// mov al, dil
    #[ignore = "BUG: produit `mov al,bh`"]
    mov_al_dil: bin(BinOp::Mov, reg(RAX), reg(RDI), BYTE) => [0x40, 0x88, 0xF8];
    /// mov spl, 1
    #[ignore = "BUG: produit `mov ah,0x1`"]
    mov_spl_1: bin(BinOp::Mov, reg(RSP), imm(1), BYTE) => [0x40, 0xB4, 0x01];
    /// mov eax, dword ptr [rbx]
    mov_eax_dword_rbx: bin(BinOp::Mov, reg(RAX), mem(at(RBX)), DWORD) => [0x8B, 0x03];
    /// mov eax, dword ptr [rbx+8]
    mov_eax_dword_rbx_plus_8: bin(BinOp::Mov, reg(RAX), mem(at(RBX).disp(8)), DWORD) => [0x8B, 0x43, 0x08];
    /// mov eax, dword ptr [rbp-16]
    mov_eax_dword_rbp_minus_16: bin(BinOp::Mov, reg(RAX), mem(at(RBP).disp(-16)), DWORD) => [0x8B, 0x45, 0xF0];
    /// mov eax, dword ptr [rbx+0x1000]
    mov_eax_dword_rbx_plus_0x1000: bin(BinOp::Mov, reg(RAX), mem(at(RBX).disp(0x1000)), DWORD) => [0x8B, 0x83, 0x00, 0x10, 0x00, 0x00];
    /// mov eax, dword ptr [rbp]
    mov_eax_dword_rbp: bin(BinOp::Mov, reg(RAX), mem(at(RBP)), DWORD) => [0x8B, 0x45, 0x00];
    /// mov eax, dword ptr [rsp]
    mov_eax_dword_rsp: bin(BinOp::Mov, reg(RAX), mem(at(RSP)), DWORD) => [0x8B, 0x04, 0x24];
    /// mov eax, dword ptr [rsp+8]
    mov_eax_dword_rsp_plus_8: bin(BinOp::Mov, reg(RAX), mem(at(RSP).disp(8)), DWORD) => [0x8B, 0x44, 0x24, 0x08];
    /// mov eax, dword ptr [r12]
    #[ignore = "BUG: produit `.byte 0x8b; .byte 0x4`"]
    mov_eax_dword_r12: bin(BinOp::Mov, reg(RAX), mem(at(R12)), DWORD) => [0x41, 0x8B, 0x04, 0x24];
    /// mov eax, dword ptr [r12+8]
    #[ignore = "BUG: produit `.byte 0x8b; rex.R; .byte 0x8`"]
    mov_eax_dword_r12_plus_8: bin(BinOp::Mov, reg(RAX), mem(at(R12).disp(8)), DWORD) => [0x41, 0x8B, 0x44, 0x24, 0x08];
    /// mov eax, dword ptr [r13]
    #[ignore = "BUG: produit `.byte 0x8b; .byte 0x5`"]
    mov_eax_dword_r13: bin(BinOp::Mov, reg(RAX), mem(at(R13)), DWORD) => [0x41, 0x8B, 0x45, 0x00];
    /// mov eax, dword ptr [r9]
    #[ignore = "BUG: produit `mov eax,DWORD PTR [rcx]`"]
    mov_eax_dword_r9: bin(BinOp::Mov, reg(RAX), mem(at(R9)), DWORD) => [0x41, 0x8B, 0x01];
    /// mov eax, dword ptr [rbx+rcx*4]
    mov_eax_dword_rbx_plus_rcx_x_4: bin(BinOp::Mov, reg(RAX), mem(at(RBX).index_scale(RCX, Scale::Four)), DWORD) => [0x8B, 0x04, 0x8B];
    /// mov eax, dword ptr [rbx+rcx*8+16]
    mov_eax_dword_rbx_plus_rcx_x_8_plus_16: bin(BinOp::Mov, reg(RAX), mem(at(RBX).index_scale(RCX, Scale::Eight).disp(16)), DWORD) => [0x8B, 0x44, 0xCB, 0x10];
    /// mov eax, dword ptr [rbp+rcx]
    #[ignore = "BUG: produit `.byte 0x8b; add al,0xd`"]
    mov_eax_dword_rbp_plus_rcx: bin(BinOp::Mov, reg(RAX), mem(at(RBP).index(RCX)), DWORD) => [0x8B, 0x44, 0x0D, 0x00];
    /// mov eax, dword ptr [rbx+r8*2]
    #[ignore = "BUG: produit `mov eax,DWORD PTR [rbx+rax*2]`"]
    mov_eax_dword_rbx_plus_r8_x_2: bin(BinOp::Mov, reg(RAX), mem(at(RBX).index_scale(R8, Scale::Two)), DWORD) => [0x42, 0x8B, 0x04, 0x43];
    /// mov eax, dword ptr [r10+rax]
    #[ignore = "BUG: produit `mov eax,DWORD PTR [rdx+rax*1]`"]
    mov_eax_dword_r10_plus_rax: bin(BinOp::Mov, reg(RAX), mem(at(R10).index(RAX)), DWORD) => [0x41, 0x8B, 0x04, 0x02];
    /// mov eax, dword ptr [rcx*4+16]
    #[ignore = "BUG: produit `mov eax,DWORD PTR [rbp+rcx*4+0x10]`"]
    mov_eax_dword_rcx_x_4_plus_16: bin(BinOp::Mov, reg(RAX), mem(MemAddress::new().index_scale(RCX, Scale::Four).disp(16)), DWORD) => [0x8B, 0x04, 0x8D, 0x10, 0x00, 0x00, 0x00];
    /// mov eax, dword ptr [rip+my_data]
    mov_eax_dword_rip_plus_my_data: bin(BinOp::Mov, reg(RAX), mem(MemAddress::symbol("my_data")), DWORD) => [0x8B, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// mov eax, dword ptr [0x1000]
    #[ignore = "BUG: produit `mov eax,DWORD PTR [rip+0x1000]`"]
    mov_eax_dword_0x1000: bin(BinOp::Mov, reg(RAX), mem(MemAddress::direct(0x1000).absolute()), DWORD) => [0x8B, 0x04, 0x25, 0x00, 0x10, 0x00, 0x00];
    /// mov qword ptr [rbx], rax
    mov_qword_rbx_rax: bin(BinOp::Mov, mem(at(RBX)), reg(RAX), QWORD) => [0x48, 0x89, 0x03];
    /// mov dword ptr [rbx], r15d
    mov_dword_rbx_r15d: bin(BinOp::Mov, mem(at(RBX)), reg(R15), DWORD) => [0x44, 0x89, 0x3B];
    /// mov qword ptr [rbx+8], rax
    mov_qword_rbx_plus_8_rax: bin(BinOp::Mov, mem(at(RBX).disp(8)), reg(RAX), QWORD) => [0x48, 0x89, 0x43, 0x08];
    /// mov dword ptr [rbx+8], r15d
    mov_dword_rbx_plus_8_r15d: bin(BinOp::Mov, mem(at(RBX).disp(8)), reg(R15), DWORD) => [0x44, 0x89, 0x7B, 0x08];
    /// mov qword ptr [rbx+rcx*8+16], rax
    mov_qword_rbx_plus_rcx_x_8_plus_16_rax: bin(BinOp::Mov, mem(at(RBX).index_scale(RCX, Scale::Eight).disp(16)), reg(RAX), QWORD) => [0x48, 0x89, 0x44, 0xCB, 0x10];
    /// mov dword ptr [rbx+rcx*8+16], r15d
    mov_dword_rbx_plus_rcx_x_8_plus_16_r15d: bin(BinOp::Mov, mem(at(RBX).index_scale(RCX, Scale::Eight).disp(16)), reg(R15), DWORD) => [0x44, 0x89, 0x7C, 0xCB, 0x10];
    /// mov qword ptr [rip+my_data], rax
    mov_qword_rip_plus_my_data_rax: bin(BinOp::Mov, mem(MemAddress::symbol("my_data")), reg(RAX), QWORD) => [0x48, 0x89, 0x05, 0x00, 0x00, 0x00, 0x00];
    /// mov dword ptr [rip+my_data], r15d
    mov_dword_rip_plus_my_data_r15d: bin(BinOp::Mov, mem(MemAddress::symbol("my_data")), reg(R15), DWORD) => [0x44, 0x89, 0x3D, 0x00, 0x00, 0x00, 0x00];
    /// mov qword ptr [r12], rax
    #[ignore = "BUG: produit `rex.W; .byte 0x89; .byte 0x4`"]
    mov_qword_r12_rax: bin(BinOp::Mov, mem(at(R12)), reg(RAX), QWORD) => [0x49, 0x89, 0x04, 0x24];
    /// mov dword ptr [r12], r15d
    #[ignore = "BUG: produit `rex.R; .byte 0x89; .byte 0x3c`"]
    mov_dword_r12_r15d: bin(BinOp::Mov, mem(at(R12)), reg(R15), DWORD) => [0x45, 0x89, 0x3C, 0x24];
    /// mov byte ptr [rbx], 0x41
    mov_byte_rbx_0x41: bin(BinOp::Mov, mem(at(RBX)), imm(0x41), BYTE) => [0xC6, 0x03, 0x41];
    /// mov byte ptr [rip+my_data], 0x41
    mov_byte_rip_plus_my_data_0x41: bin(BinOp::Mov, mem(MemAddress::symbol("my_data")), imm(0x41), BYTE) => [0xC6, 0x05, 0x00, 0x00, 0x00, 0x00, 0x41];
    /// mov word ptr [rbx], 0x1234
    mov_word_rbx_0x1234: bin(BinOp::Mov, mem(at(RBX)), imm(0x1234), WORD) => [0x66, 0xC7, 0x03, 0x34, 0x12];
    /// mov word ptr [rip+my_data], 0x1234
    mov_word_rip_plus_my_data_0x1234: bin(BinOp::Mov, mem(MemAddress::symbol("my_data")), imm(0x1234), WORD) => [0x66, 0xC7, 0x05, 0x00, 0x00, 0x00, 0x00, 0x34, 0x12];
    /// mov dword ptr [rbx], 0x12345678
    mov_dword_rbx_0x12345678: bin(BinOp::Mov, mem(at(RBX)), imm(0x12345678), DWORD) => [0xC7, 0x03, 0x78, 0x56, 0x34, 0x12];
    /// mov dword ptr [rip+my_data], 0x12345678
    mov_dword_rip_plus_my_data_0x12345678: bin(BinOp::Mov, mem(MemAddress::symbol("my_data")), imm(0x12345678), DWORD) => [0xC7, 0x05, 0x00, 0x00, 0x00, 0x00, 0x78, 0x56, 0x34, 0x12];
    /// mov qword ptr [rbx], 0x1234
    #[ignore = "BUG: produit `mov QWORD PTR [rbx],0x1234; add BYTE PTR [rax],al`"]
    mov_qword_rbx_0x1234: bin(BinOp::Mov, mem(at(RBX)), imm(0x1234), QWORD) => [0x48, 0xC7, 0x03, 0x34, 0x12, 0x00, 0x00];
    /// mov qword ptr [rip+my_data], 0x1234
    #[ignore = "BUG: produit `mov QWORD PTR [rip+0x0],0x1234; add BYTE PTR [rax],al`"]
    mov_qword_rip_plus_my_data_0x1234: bin(BinOp::Mov, mem(MemAddress::symbol("my_data")), imm(0x1234), QWORD) => [0x48, 0xC7, 0x05, 0x00, 0x00, 0x00, 0x00, 0x34, 0x12, 0x00, 0x00];
    /// mov r12d, offset my_data
    mov_r12d_my_data: bin(BinOp::Mov, reg(R12), sym("my_data"), DWORD) => [0x41, 0xBC, 0x00, 0x00, 0x00, 0x00];
    /// mov rax, offset my_data
    mov_rax_my_data: bin(BinOp::Mov, reg(RAX), sym("my_data"), QWORD) => [0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00] | [0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    /// movzx eax, bl
    movzx_eax_bl: bin(BinOp::Movzx, reg(RAX), reg(RBX), BYTE) => [0x0F, 0xB6, 0xC3];
    /// movzx eax, r10b
    movzx_eax_r10b: bin(BinOp::Movzx, reg(RAX), reg(R10), BYTE) => [0x41, 0x0F, 0xB6, 0xC2];
    /// movzx r9d, cl
    movzx_r9d_cl: bin(BinOp::Movzx, reg(R9), reg(RCX), BYTE) => [0x44, 0x0F, 0xB6, 0xC9];
    /// movzx eax, sil
    #[ignore = "BUG: produit `movzx eax,dh`"]
    movzx_eax_sil: bin(BinOp::Movzx, reg(RAX), reg(RSI), BYTE) => [0x40, 0x0F, 0xB6, 0xC6];
    /// movzx ecx, byte ptr [rbx]
    movzx_ecx_byte_rbx: bin(BinOp::Movzx, reg(RCX), mem(at(RBX)), BYTE) => [0x0F, 0xB6, 0x0B];
    /// movzx eax, bx
    #[ignore = "BUG: produit `movzx ax,bx`"]
    movzx_eax_bx: bin(BinOp::Movzx, reg(RAX), reg(RBX), WORD) => [0x0F, 0xB7, 0xC3];
    /// movzx ebx, word ptr [rbx]
    #[ignore = "BUG: produit `movzx bx,WORD PTR [rbx]`"]
    movzx_ebx_word_rbx: bin(BinOp::Movzx, reg(RBX), mem(at(RBX)), WORD) => [0x0F, 0xB7, 0x1B];
    /// movsx eax, bl
    movsx_eax_bl: bin(BinOp::Movsx, reg(RAX), reg(RBX), BYTE) => [0x0F, 0xBE, 0xC3];
    /// movsx eax, r10b
    movsx_eax_r10b: bin(BinOp::Movsx, reg(RAX), reg(R10), BYTE) => [0x41, 0x0F, 0xBE, 0xC2];
    /// movsx r9d, cl
    movsx_r9d_cl: bin(BinOp::Movsx, reg(R9), reg(RCX), BYTE) => [0x44, 0x0F, 0xBE, 0xC9];
    /// movsx eax, sil
    #[ignore = "BUG: produit `movsx eax,dh`"]
    movsx_eax_sil: bin(BinOp::Movsx, reg(RAX), reg(RSI), BYTE) => [0x40, 0x0F, 0xBE, 0xC6];
    /// movsx ecx, byte ptr [rbx]
    movsx_ecx_byte_rbx: bin(BinOp::Movsx, reg(RCX), mem(at(RBX)), BYTE) => [0x0F, 0xBE, 0x0B];
    /// movsx eax, bx
    #[ignore = "BUG: produit `movsx ax,bx`"]
    movsx_eax_bx: bin(BinOp::Movsx, reg(RAX), reg(RBX), WORD) => [0x0F, 0xBF, 0xC3];
    /// movsx ebx, word ptr [rbx]
    #[ignore = "BUG: produit `movsx bx,WORD PTR [rbx]`"]
    movsx_ebx_word_rbx: bin(BinOp::Movsx, reg(RBX), mem(at(RBX)), WORD) => [0x0F, 0xBF, 0x1B];
    /// xchg bl, cl
    xchg_bl_cl: bin(BinOp::Xchg, reg(RBX), reg(RCX), BYTE) => [0x86, 0xCB] | [0x86, 0xD9];
    /// xchg bx, cx
    xchg_bx_cx: bin(BinOp::Xchg, reg(RBX), reg(RCX), WORD) => [0x66, 0x87, 0xCB] | [0x66, 0x87, 0xD9];
    /// xchg ebx, ecx
    xchg_ebx_ecx: bin(BinOp::Xchg, reg(RBX), reg(RCX), DWORD) => [0x87, 0xCB] | [0x87, 0xD9];
    /// xchg rbx, rcx
    xchg_rbx_rcx: bin(BinOp::Xchg, reg(RBX), reg(RCX), QWORD) => [0x48, 0x87, 0xCB] | [0x48, 0x87, 0xD9];
    /// xchg r11d, ebx
    xchg_r11d_ebx: bin(BinOp::Xchg, reg(R11), reg(RBX), DWORD) => [0x41, 0x87, 0xDB] | [0x44, 0x87, 0xDB];
    /// xchg rax, rbx
    xchg_rax_rbx: bin(BinOp::Xchg, reg(RAX), reg(RBX), QWORD) => [0x48, 0x93] | [0x48, 0x87, 0xC3];
    /// xchg ecx, dword ptr [rbx]
    xchg_ecx_dword_rbx: bin(BinOp::Xchg, reg(RCX), mem(at(RBX)), DWORD) => [0x87, 0x0B];
    /// xchg dword ptr [rip+my_data], eax
    xchg_dword_rip_plus_my_data_eax: bin(BinOp::Xchg, mem(MemAddress::symbol("my_data")), reg(RAX), DWORD) => [0x87, 0x05, 0x00, 0x00, 0x00, 0x00];
}
