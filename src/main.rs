use std::fs::File;
mod elf;
use elf::instructions::*;

use crate::elf::elfsym::make_st_info;
// mod lexical_analisys;



fn main() -> std::io::Result<()> {
    let mut file = File::create("output.elf")?;
    let instr: Vec<Instruction> = vec![
        Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(0x11111111) },
        Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Sym("my_data".to_string()) },

        // ===== base =====
        Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: Operand::MemoryAddress(MemAddress::Base { base: Register::Ebx }) },
        Instruction::Mov { dst: Operand::MemoryAddress(MemAddress::Base { base: Register::Ebx }), src: Operand::Reg(Register::Eax) },

        // ===== base + disp =====
        Instruction::Mov { dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(4) }), src: Operand::Reg(Register::Eax) },
        Instruction::Mov { dst: Operand::Reg(Register::Edx), src: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(4) }) },

        // ===== base + symbol =====
        Instruction::Mov { dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Sym("my_data".to_string()) }), src: Operand::Reg(Register::Eax) },
        Instruction::Mov { dst: Operand::Reg(Register::Esi), src: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Sym("my_data".to_string()) }) },

        // ===== base + disp (negatif) =====
        Instruction::Mov { dst: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Ebx, disp: MemDisplacement::Imm(-4) }), src: Operand::Reg(Register::Eax) },

        // ===== base + index =====
        Instruction::Mov { dst: Operand::MemoryAddress(MemAddress::BaseIndex { base: Register::Ebx, index: Register::Ecx }), src: Operand::Reg(Register::Eax) },
        Instruction::Mov { dst: Operand::Reg(Register::Edi), src: Operand::MemoryAddress(MemAddress::BaseIndex { base: Register::Ebx, index: Register::Ecx }) },

        // ===== base + index * scale =====
        Instruction::Mov { dst: Operand::MemoryAddress(MemAddress::BaseIndexScale { base: Register::Ebx, index: Register::Ecx, scale: Scale::Four }), src: Operand::Reg(Register::Eax) },
        Instruction::Mov { dst: Operand::Reg(Register::Ebp), src: Operand::MemoryAddress(MemAddress::BaseIndexScale { base: Register::Ebx, index: Register::Ecx, scale: Scale::Four }) },

        // ===== base + index * scale + disp =====
        Instruction::Mov { dst: Operand::MemoryAddress(MemAddress::BaseIndexScaleDisp { base: Register::Ebx, index: Register::Ecx, scale: Scale::Four, disp: MemDisplacement::Imm(8) }), src: Operand::Reg(Register::Eax) },
        Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::MemoryAddress(MemAddress::BaseIndexScaleDisp { base: Register::Ebx, index: Register::Ecx, scale: Scale::Four, disp: MemDisplacement::Imm(8) }) },

        // ===== index * scale + disp =====
        Instruction::Mov { dst: Operand::MemoryAddress(MemAddress::IndexScaleDisp { index: Register::Ecx, scale: Scale::Two, disp: MemDisplacement::Imm(16) }), src: Operand::Reg(Register::Eax) },
        Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::MemoryAddress(MemAddress::IndexScaleDisp { index: Register::Ecx, scale: Scale::Two, disp: MemDisplacement::Imm(16) }) },

        // ===== direct (absolute) =====
        Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: Operand::MemoryAddress(MemAddress::Direct { disp: MemDisplacement::Imm(0x12345678) }) },

        // ===== direct symbol (IMPORTANT) =====
        Instruction::Mov { dst: Operand::Reg(Register::Edx), src: Operand::MemoryAddress(MemAddress::Direct { disp: MemDisplacement::Sym("my_data".to_string()) }) },

        // ===== ESP (force SIB) =====
        Instruction::Mov { dst: Operand::MemoryAddress(MemAddress::Base { base: Register::Esp }), src: Operand::Reg(Register::Eax) },
        Instruction::Mov { dst: Operand::Reg(Register::Ecx), src: Operand::MemoryAddress(MemAddress::BaseDisp { base: Register::Esp, disp: MemDisplacement::Imm(4) }) },

        Instruction::Int(0x80),
    ];


    let instr2 : Vec<Instruction> = vec![
        Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(75) },
        Instruction::Mov { dst: Operand::Reg(Register::Eax), src: Operand::Imm(1)},
        Instruction::Mov { dst: Operand::Reg(Register::Ebx), src: Operand::Imm(84) },
        Instruction::Ret,
        Instruction::Int(0x80)
    ];

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
        sh_flags: (elf::shdr::ShFlags::Alloc as u64 | elf::shdr::ShFlags::ExecInstr as u64),
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
