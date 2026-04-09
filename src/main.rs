use std::fs::File;
mod elf;
use elf::instructions::*;

use crate::elf::elfsym::make_st_info;
// mod lexical_analisys;



fn main() -> std::io::Result<()> {
    let mut file = File::create("output.elf")?;

    let instr: Vec<Instruction> = vec![
        Instruction::Mov { dst: Operand::Reg(RegisterArch::X32(Register::Eax)), src: Operand::Imm(1) },
        Instruction::Mov { dst: Operand::Reg(RegisterArch::X32(Register::Ebx)), src: Operand::Imm(42) },
        Instruction::Call(Operand::Sym("my_func".to_string())),
        Instruction::Int(0x80)
    ];

    let instr2 : Vec<Instruction> = vec![
        Instruction::Mov { dst: Operand::Reg(RegisterArch::X32(Register::Eax)), src: Operand::Imm(75) },
        Instruction::Mov { dst: Operand::Reg(RegisterArch::X32(Register::Eax)), src: Operand::Imm(1)},
        Instruction::Mov { dst: Operand::Reg(RegisterArch::X32(Register::Ebx)), src: Operand::Imm(84) },
        Instruction::Ret,
        Instruction::Int(0x80)
    ];

    let mut elf_file = elf::file::ElfFile64::default();
    let section = elf_file.add_section(elf::shdr::SectionName::Text.as_str().to_string(), 
    elf::shdr::ElfShdr {
                    sh_type: elf::shdr::ShType::ProgBits as u32,
                    sh_flags: (elf::shdr::ShFlags::Alloc as u64 | elf::shdr::ShFlags::ExecInstr as u64),
                    sh_addralign: 16,
                    ..Default::default()
                }
            );

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
        // &text_binary2, 
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
