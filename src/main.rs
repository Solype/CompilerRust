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

    for ins in instr {
        println!("{:X?}", ins.encode())
    }

    let mut elf_file = elf::elf_file::ElfFile64::default();
    // elf_file.add_section(vec![]);
    elf_file.write(&mut file)?;
    println!("Fichier ELF généré : output.elf");
    Ok(())
}
