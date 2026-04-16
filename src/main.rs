use std::fs::File;
mod elf;
use elf::instructions::*;

use crate::elf::elfsym::make_st_info;
// mod lexical_analisys;



fn main() -> std::io::Result<()> {
    use std::fs::File;
    use elf::instructions::*;

    let mut file = File::create("output.elf")?;
    let mut instr: Vec<Instruction> = vec![];

    let my_data = Operand::MemoryAddress(MemAddress::Direct {
        disp: MemDisplacement::Sym("my_data".to_string()),
    });

    instr.push(Instruction::Ctrl { op: CtrlOp::Call, target: Operand::Sym("my_data".to_string()) });
    instr.push(Instruction::Ctrl { op: CtrlOp::Jmp, target: Operand::Sym("my_data".to_string()) });
    instr.push(Instruction::Ctrl { op: CtrlOp::Ret, target: Operand::Sym("my_data".to_string()) });
    instr.push(Instruction::Ctrl { op: CtrlOp::Je, target: Operand::Sym("my_data".to_string()) });
    instr.push(Instruction::Ctrl { op: CtrlOp::Jne, target: Operand::Sym("my_data".to_string()) });
    instr.push(Instruction::Ctrl { op: CtrlOp::Jg, target: Operand::Sym("my_data".to_string()) });
    instr.push(Instruction::Ctrl { op: CtrlOp::Jl, target: Operand::Sym("my_data".to_string()) });
    instr.push(Instruction::Ctrl { op: CtrlOp::Jge, target: Operand::Sym("my_data".to_string()) });
    instr.push(Instruction::Ctrl { op: CtrlOp::Jle, target: Operand::Sym("my_data".to_string()) });
    // =========================================================
    // MOV REG <- IMM (U8 / U16 / U32 / U64 + edge cases)
    // =========================================================

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(0x1), size: Some(Size::U8) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(0x12), size: Some(Size::U8) });

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(0x1234), size: Some(Size::U16) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(0xABCD), size: Some(Size::U16) });

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: Operand::Imm(0x12345678), size: Some(Size::U32) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Edx), src: Operand::Imm(0xDEADBEEF), size: Some(Size::U32) });

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(0x1122334455667788), size: Some(Size::U64) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(0xFFFFFFFFFFFFFFFF), size: Some(Size::U64) });

    // =========================================================
    // MOV REG <- REG (cross size stress)
    // =========================================================

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U8) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ecx), size: Some(Size::U16) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Edx), size: Some(Size::U32) });

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Eax), size: Some(Size::U8) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Ecx), size: Some(Size::U16) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Edx), size: Some(Size::U32) });

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Eax), size: Some(Size::U8) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Ebx), size: Some(Size::U16) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Edx), size: Some(Size::U32) });

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Eax), size: Some(Size::U8) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Ebx), size: Some(Size::U16) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) });

    // =========================================================
    // MOV MEM <- IMM (all sizes)
    // =========================================================

    // instr.push(Instruction::Mov { dst: my_data.clone(), src: Operand::Imm(0x41), size: Some(Size::U8) });
    // instr.push(Instruction::Mov { dst: my_data.clone(), src: Operand::Imm(0x4243), size: Some(Size::U16) });
    // instr.push(Instruction::Mov { dst: my_data.clone(), src: Operand::Imm(0x44454647), size: Some(Size::U32) });
    // instr.push(Instruction::Mov { dst: my_data.clone(), src: Operand::Imm(0x11223344), size: Some(Size::U32) });

    // =========================================================
    // MOV MEM <- REG
    // =========================================================

    // instr.push(Instruction::Mov { dst: my_data.clone(), src: Operand::Reg(Register::Eax), size: Some(Size::U8) });
    // instr.push(Instruction::Mov { dst: my_data.clone(), src: Operand::Reg(Register::Ebx), size: Some(Size::U16) });
    // instr.push(Instruction::Mov { dst: my_data.clone(), src: Operand::Reg(Register::Ecx), size: Some(Size::U32) });
    // instr.push(Instruction::Mov { dst: my_data.clone(), src: Operand::Reg(Register::Edx), size: Some(Size::U32) });

    // =========================================================
    // MOV REG <- MEM
    // =========================================================

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: my_data.clone(), size: Some(Size::U8) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: my_data.clone(), size: Some(Size::U16) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: my_data.clone(), size: Some(Size::U32) });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Edx), src: my_data.clone(), size: Some(Size::U32) });

    // =========================================================
    // MOV EDGE CASES (size = None)
    // =========================================================

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(0x1), size: None });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(0x1234), size: None });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: Operand::Imm(0x12345678), size: None });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Eax), size: None });

    // instr.push(Instruction::Mov { dst: my_data.clone(), src: Operand::Imm(0x99), size: None });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: my_data.clone(), size: None });

    // =========================================================
    // ADD IMM -> REG (all sizes)
    // =========================================================

    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Eax), src: Operand::Imm(1), size: Some(Size::U8) });
    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Eax), src: Operand::Imm(0x12), size: Some(Size::U8) });

    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(0x1234), size: Some(Size::U16) });
    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Ecx), src: Operand::Imm(0xABCD), size: Some(Size::U16) });

    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Edx), src: Operand::Imm(0x12345678), size: Some(Size::U32) });
    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Eax), src: Operand::Imm(0xDEADBEEF), size: Some(Size::U32) });

    // =========================================================
    // ADD REG <- REG
    // =========================================================

    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ebx), size: Some(Size::U8) });
    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Ecx), size: Some(Size::U16) });
    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Edx), size: Some(Size::U32) });

    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Edx), src: Operand::Reg(Register::Eax), size: Some(Size::U8) });
    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Eax), src: Operand::Reg(Register::Ecx), size: Some(Size::U16) });
    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Ebx), src: Operand::Reg(Register::Edx), size: Some(Size::U32) });

    // =========================================================
    // ADD EDGE CASES (size = None)
    // =========================================================

    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Eax), src: Operand::Imm(1), size: None });
    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(0x1234), size: None });
    // instr.push(Instruction::Add { dst: Operand::Reg(Register::Ecx), src: Operand::Reg(Register::Edx), size: None });

    // =========================================================
    // SYSCALL EDGE CASE (bonus sanity check)
    // =========================================================

    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(1), size: None });
    // instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(42), size: None });
    // instr.push(Instruction::Int(0x80));

    // =========================================================
    // ELF SETUP (inchangé)
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
        &instr,
        make_st_info(
            elf::elfsym::StBind::Global,
            elf::elfsym::StType::Func
        ),
        elf::elfsym::StVis::Default as u8,
    ).expect("encode error");

    elf_file.add_symbol_to_section(
        text_section,
        "_start".to_string(),
        &instr,
        make_st_info(
            elf::elfsym::StBind::Global,
            elf::elfsym::StType::Func
        ),
        elf::elfsym::StVis::Default as u8,
    ).expect("encode error");

    elf_file.write(&mut file)?;
    println!("ELF généré : output.elf");

    Ok(())
}
