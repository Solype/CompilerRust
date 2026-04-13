use std::fs::File;
mod elf;
use elf::instructions::*;

use crate::elf::elfsym::make_st_info;
// mod lexical_analisys;



fn main() -> std::io::Result<()> {
    let mut file = File::create("output.elf")?;
    let instr: Vec<Instruction> = vec![
        // call my_func
        Instruction::Call(Operand::Sym("my_func".to_string())),

        // exit(42)
        Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(1) },
        Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(42) },
        Instruction::Int(0x80),
    ];


    let instr2 : Vec<Instruction> = vec![
        // write(1, my_data, 1)
        Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(4) },
        Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(1) },
        Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: Operand::Sym("my_data".to_string()) },
        Instruction::Mov { dst: Operand::Reg(Register::Edx), src: Operand::Imm(1) },
        Instruction::Int(0x80),

        Instruction::Ret,
    ];

    let mut elf_file = elf::file::ElfFile32::default();

    let section_data = elf_file.add_section(
        elf::shdr::SectionName::Data.as_str().to_string(),
        elf::shdr::ElfShdr {
            sh_type: elf::shdr::ShType::ProgBits as u32,
            sh_flags: (elf::shdr::ShFlags::Alloc as u32 | elf::shdr::ShFlags::Write as u32),
            sh_addralign: 4,
            ..Default::default()
        }
    );

    elf_file.add_symbol_to_section_raw(section_data, "my_data".to_string(),
        &vec!['a' as u8, 'b' as u8, 'c' as u8, 'd' as u8, 'e' as u8, 'f' as u8, 'g' as u8, 0u8],
            make_st_info(
            elf::elfsym::StBind::Global,
            elf::elfsym::StType::Func
        ),
        elf::elfsym::StVis::Default as u8,
    );

    let section = elf_file.add_section(elf::shdr::SectionName::Text.as_str().to_string(), 
    elf::shdr::ElfShdr {
        sh_type: elf::shdr::ShType::ProgBits as u32,
        sh_flags: (elf::shdr::ShFlags::Alloc as u32 | elf::shdr::ShFlags::ExecInstr as u32),
        sh_addralign: 16,
        ..Default::default()
    });

    elf_file.add_symbol_to_section(
        section,
        "my_func".to_string(),
        &instr2,
        // &text_binary2,
        make_st_info(
            elf::elfsym::StBind::Global,
            elf::elfsym::StType::Func
        ),
        elf::elfsym::StVis::Default as u8,
    ).expect("Error while encoding");

    elf_file.add_symbol_to_section(
        section,
        "_start".to_string(),
        &instr, 
        make_st_info(
            elf::elfsym::StBind::Global,
            elf::elfsym::StType::Func
        ), 
        elf::elfsym::StVis::Default as u8,
    ).expect("Error while encoding");

    elf_file.write(&mut file)?;
    println!("Fichier ELF généré : output.elf");
    Ok(())
}
