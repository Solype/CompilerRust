use std::fs::File;
mod elf;
use elf::instructions::*;

use crate::elf::symbol::make_st_info;
// mod lexical_analisys;



fn main() -> std::io::Result<()> {
    let mut file = File::create("output.elf")?;

    let instr: Vec<Instruction> = vec![
        Instruction::Mov { dst: Operand::Reg(RegisterArch::X32(Register::Eax)), src: Operand::Imm(1) },
        Instruction::Mov { dst: Operand::Reg(RegisterArch::X32(Register::Ebx)), src: Operand::Imm(84) },
        Instruction::Int(0x80)
    ];

    let text_binary: Vec<u8> = instr.iter()
        .flat_map(|ins| ins.encode())
        .collect();
    let len_txt = text_binary.len();

    let mut elf_file = elf::file::ElfFile64::default();
    elf_file.add_section(elf::shdr::SectionName::Text, text_binary, 
        elf::shdr::ElfShdr {
                    sh_type: elf::shdr::ShType::ProgBits as u32,
                    sh_flags: (elf::shdr::ShFlags::Alloc as u64 | elf::shdr::ShFlags::ExecInstr as u64),
                    sh_addralign: 16,
                    ..Default::default()
                }
            );

    let start_name = elf_file.strtab.name("_start".to_string());
    elf_file.symtab.push(
        elf::symbol::ElfSym::<u64> {
            st_name: start_name as u32,
            st_info: make_st_info(elf::symbol::StBind::Global, elf::symbol::StType::Func),
            st_other: elf::symbol::StVis::Default as u8,
            st_shndx: 1,
            st_size: len_txt as u64,
            st_value: 0,
            ..Default::default()
        }
    );
    elf_file.write(&mut file)?;
    println!("Fichier ELF généré : output.elf");
    Ok(())
}
