mod elf;

use crate::elf::elfsym::make_st_info;
// mod lexical_analisys;



fn main() -> std::io::Result<()> {
    use std::fs::File;
    use elf::instructions::*;

    let mut file = File::create("output.elf")?;

    let my_data = Operand::MemoryAddress(MemAddress::Direct {
        disp: MemDisplacement::Sym("my_data".to_string()),
    });

    // =========================================================
    // FUNCTION: my_func (CTRL test)
    // =========================================================
    let func_instr: Vec<Instruction> = vec![
        Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jmp,  target: Operand::Sym("my_func".to_string()) },

        Instruction::Ctrl { op: CtrlOp::Je,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jne, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jg,  target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jl,  target: Operand::Sym("my_func".to_string()) },

        Instruction::Ctrl { op: CtrlOp::Jge, target: Operand::Sym("my_func".to_string()) },
        Instruction::Ctrl { op: CtrlOp::Jle, target: Operand::Sym("my_func".to_string()) },

        // ✅ RET sans target
        Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::NoOperand },
    ];

    // =========================================================
    // ENTRY: _start (ALU + MOV tests)
    // =========================================================
    let mut start_instr: Vec<Instruction> = vec![];

    // -----------------------------
    // MOV reg <- imm
    // -----------------------------
    start_instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: Operand::Reg(Register::Eax),
        src: Operand::Imm(0x12),
        size: Some(Size::U8),
    });

    start_instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: Operand::Reg(Register::Ebx),
        src: Operand::Imm(0x1234),
        size: Some(Size::U16),
    });

    start_instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: Operand::Reg(Register::Ecx),
        src: Operand::Imm(0x12345678),
        size: Some(Size::U32),
    });

    // // -----------------------------
    // // MOV reg <-> reg
    // // -----------------------------
    start_instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: Operand::Reg(Register::Eax),
        src: Operand::Reg(Register::Ebx),
        size: Some(Size::U32),
    });

    // // -----------------------------
    // // MOV mem <- imm
    // // -----------------------------
    start_instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: my_data.clone(),
        src: Operand::Imm(0x41),
        size: Some(Size::U8),
    });

    // // -----------------------------
    // // MOV mem <- reg
    // // -----------------------------
    start_instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: my_data.clone(),
        src: Operand::Reg(Register::Eax),
        size: Some(Size::U32),
    });

    // // -----------------------------
    // // MOV reg <- mem
    // // -----------------------------
    start_instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: Operand::Reg(Register::Eax),
        src: my_data.clone(),
        size: Some(Size::U32),
    });

    // // -----------------------------
    // // ADD reg, imm
    // // -----------------------------
    start_instr.push(Instruction::Binary {
        op: BinOp::Add,
        dst: Operand::Reg(Register::Eax),
        src: Operand::Imm(5),
        size: Some(Size::U32),
    });

    // // -----------------------------
    // // ADD reg, reg
    // // -----------------------------
    start_instr.push(Instruction::Binary {
        op: BinOp::Add,
        dst: Operand::Reg(Register::Eax),
        src: Operand::Reg(Register::Ebx),
        size: Some(Size::U32),
    });

    start_instr.push(Instruction::Binary {
        op: BinOp::Add,
        dst: Operand::MemoryAddress(
            MemAddress::BaseDisp {
                base: Register::Eax, // ou Rax selon ton design
                disp: MemDisplacement::Imm(3),
            }
        ),
        src: Operand::Imm(3),
        size: Some(Size::U8),
    });

    // // -----------------------------
    // // syscall exit(42)
    // // -----------------------------
    start_instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: Operand::Reg(Register::Eax),
        src: Operand::Imm(1),
        size: None,
    });

    start_instr.push(Instruction::Binary {
        op: BinOp::Mov,
        dst: Operand::Reg(Register::Ebx),
        src: Operand::Imm(42),
        size: None,
    });

    start_instr.push(Instruction::Sys {
        op: SysOp::Int(0x80),
    });

    // =========================================================
    // ELF SETUP
    // =========================================================

    let mut elf_file = elf::file::ElfFile64::default();

    let section_data = elf_file.add_section(
        elf::shdr::SectionName::Data.as_str().to_string(),
        elf::shdr::ElfShdr {
            sh_type: elf::shdr::ShType::ProgBits as u32,
            sh_flags: (elf::shdr::ShFlags::Alloc as u64 | elf::shdr::ShFlags::Write as u64),
            sh_addralign: 4,
            ..Default::default()
        }
    );

    elf_file.add_symbol_to_section_raw(
        section_data,
        "my_data".to_string(),
        &vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g', 0],
        make_st_info(
            elf::elfsym::StBind::Global,
            elf::elfsym::StType::Object
        ),
        elf::elfsym::StVis::Default as u8,
    );

    let text_section = elf_file.add_section(
        elf::shdr::SectionName::Text.as_str().to_string(),
        elf::shdr::ElfShdr {
            sh_type: elf::shdr::ShType::ProgBits as u32,
            sh_flags: (elf::shdr::ShFlags::Alloc as u64 | elf::shdr::ShFlags::ExecInstr as u64),
            sh_addralign: 16,
            ..Default::default()
        }
    );

    elf_file.add_symbol_to_section(
        text_section,
        "my_func".to_string(),
        &func_instr,
        make_st_info(
            elf::elfsym::StBind::Global, elf::elfsym::StType::Func),
        elf::elfsym::StVis::Default as u8,
    ).expect("encode error");

    elf_file.add_symbol_to_section(
        text_section,
        "_start".to_string(),
        &start_instr,
        make_st_info(
            elf::elfsym::StBind::Global, elf::elfsym::StType::Func),
        elf::elfsym::StVis::Default as u8,
    ).expect("encode error");

    elf_file.write(&mut file)?;
    println!("ELF généré : output.elf");

    Ok(())
}
