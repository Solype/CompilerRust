use std::fs::File;
mod elf;
use elf::instructions::*;
// mod lexical_analisys;



fn main() -> std::io::Result<()> {
    let mut file = File::create("output.elf")?;

    let mut instr: Vec<Instruction> = vec![];
    instr.push(Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(1) });
    instr.push(Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(84) });
    instr.push(Instruction::Int(80));

    let text_binary: Vec<u8> = instr.iter()
        .flat_map(|ins| ins.encode())
        .collect();

    let mut elf_file = elf::elf_file::ElfFile64::default();
    elf_file.add_section(elf::shdr::SectionName::Text, text_binary, 
        elf::shdr::ElfShdr {
                    sh_type: elf::shdr::ShType::ProgBits as u32,
                    sh_flags: (elf::shdr::ShFlags::Alloc as u64 | elf::shdr::ShFlags::ExecInstr as u64),
                    sh_addralign: 16,
                    ..Default::default()
                }
            );
    elf_file.write(&mut file)?;
    println!("Fichier ELF généré : output.elf");
    Ok(())
}
