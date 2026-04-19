use super::enums::*;

///////////////////////////////////////////////////////////////////

fn encode_modrm_field_reg(reg_op: &Operand) -> u8 {
    match reg_op {
        Operand::Reg(r) => *r as u8,
        Operand::NoOperand => 0, // si instruction utilise /digit au lieu de reg
        _ => panic!("ModRM register field must be a register or noop!"),
    }
}

fn encode_sib(scale: Scale, index: Option<Register>, base: Option<Register>) -> u8 {
    let scale_bits = scale as u8;

    let index_bits = match index {
        Some(r) if r != Register::Esp => r as u8,
        _ => 0b100, // no index
    };

    let base_bits = match base {
        Some(r) => r as u8,
        None => 0b101, // disp32
    };

    (scale_bits << 6) | (index_bits << 3) | base_bits
}

///////////////////////////////////////////////////////////////////
/// 
/// 
/// 
///////////////////////////////////////////////////////////////////

fn get_disp(
    disp: &MemDisplacement,
    data_len: usize,
    relocs: &mut Vec<Relocation>
) -> i32 {
    match disp {
        MemDisplacement::Imm(val) => *val,
        MemDisplacement::Sym(sym) => {
            relocs.push(Relocation {
                sym: sym.clone(),
                offset: data_len,
                size: 4,
                kind: RelocKind::Absolute, // 🔥 FIX
                addend: 0,                // 🔥 FIX
            });
            0
        }
    }
}

fn encode_direct(disp: &MemDisplacement, data: &mut Vec<u8>, relocs: &mut Vec<Relocation>) {
    data[0] = (MEMNODISP << 6) | 0b101;

    let offset = data.len();

    match disp {
        MemDisplacement::Sym(sym) => {
            data.extend(&0u32.to_le_bytes());

            relocs.push(Relocation {
                sym: sym.clone(),
                offset,
                size: 4,
                kind: RelocKind::Relative,
                addend: -4,
            });
        }

        MemDisplacement::Imm(val) => {
            data.extend(&(*val as u32).to_le_bytes());
        }
    }
}

fn encode_base(base: Register, data: &mut Vec<u8>) {
    if base == Register::Esp {
        data[0] = (MEMNODISP << 6) | 0b100;
        data.push(encode_sib(Scale::One, None, Some(base)));
    } else if base == Register::Ebp {
        data[0] = (MEMDISP8 << 6) | (base as u8);
        data.push(0);
    } else {
        data[0] = (MEMNODISP << 6) | (base as u8);
    }
}

fn encode_base_disp(
    base: Register,              // Base register used in memory operand
    disp: &MemDisplacement,      // Displacement: immediate or symbolic offset
    data: &mut Vec<u8>,          // Output encoded bytes (ModRM / SIB / displacement)
    relocs: &mut Vec<Relocation> // Relocations if displacement references a symbol
) {
    // Resolve displacement value.
    // If symbolic, creates relocation entry and returns placeholder value.
    let val = get_disp(disp, data.len(), relocs);

    // Encode displacement size:
    // returns:
    // - mod_bits : ModRM mode bits (disp8 / disp32 / no disp)
    // - disp_bytes : encoded displacement bytes
    let (mod_bits, disp_bytes) = encode_disp(val, matches!(disp, MemDisplacement::Sym(_)));

    // Special case:
    // ESP/RSP as base requires mandatory SIB byte.
    if base == Register::Esp {
        // ModRM:
        // mod = mod_bits
        // rm  = 100 => SIB follows
        data[0] = (mod_bits << 6) | 0b100;

        // SIB:
        // scale = 1
        // index = none
        // base = ESP/RSP
        //
        // Represents:
        // [esp + disp]
        data.push(encode_sib(Scale::One, None, Some(base)));
    } else {
        // Standard ModRM:
        // mod = mod_bits
        // rm  = base register
        //
        // Represents:
        // [base + disp]
        data[0] = (mod_bits << 6) | (base as u8);
    }

    // Append displacement bytes (8-bit or 32-bit typically)
    data.extend(disp_bytes);
}


fn encode_base_index(base: Register, index: Register, data: &mut Vec<u8>) {
    data[0] = (MEMNODISP << 6) | 0b100;
    data.push(encode_sib(Scale::One, Some(index), Some(base)));
}

fn encode_index_disp(
    index: Register,
    scale: Scale,
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    assert!(index != Register::Esp); // interdit

    let val = get_disp(disp, data.len(), relocs);
    let (mod_bits, disp_bytes) = encode_disp(val, matches!(disp, MemDisplacement::Sym(_)));

    data[0] = (mod_bits << 6) | 0b100; // rm = 100 → SIB

    // base = none → 101
    data.push(encode_sib(scale, Some(index), None));

    data.extend(disp_bytes);
}

fn encode_base_index_scale(
    base: Register,
    index: Register,
    scale: Scale,
    data: &mut Vec<u8>,
) {
    assert!(index != Register::Esp);

    data[0] = (MEMNODISP << 6) | 0b100;

    data.push(encode_sib(scale, Some(index), Some(base)));
}

fn encode_base_index_scale_disp(
    base: Register,
    index: Register,
    scale: Scale,
    disp: &MemDisplacement,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    assert!(index != Register::Esp);

    let val = get_disp(disp, data.len(), relocs);
    let (mod_bits, disp_bytes) = encode_disp(val, matches!(disp, MemDisplacement::Sym(_)));

    data[0] = (mod_bits << 6) | 0b100;

    data.push(encode_sib(scale, Some(index), Some(base)));

    data.extend(disp_bytes);
}

///////////////////////////////////////////////////////////////////
/// 
/// 
/// 
///////////////////////////////////////////////////////////////////

fn encode_disp(disp: i32, is_placeholder : bool) -> (u8, Vec<u8>) {
    if is_placeholder {
        return (0b10, disp.to_le_bytes().to_vec());
    }
    if disp == 0 {
        (MEMNODISP, vec![])
    } else if (-128..=127).contains(&disp) {
        (MEMDISP8, vec![disp as u8])
    } else {
        (MEMDISP32, disp.to_le_bytes().to_vec())
    }
}

fn encode_memory(
    mem: &MemAddress,              // Memory addressing mode to encode
    data: &mut Vec<u8>,           // Output machine code bytes (ModRM/SIB/displacement)
    relocs: &mut Vec<Relocation>, // Relocations generated by symbolic displacements
) {
    match mem {
        // Absolute memory address:
        // [disp]
        // Example: [0x401000]
        MemAddress::Direct { disp } =>
            encode_direct(disp, data, relocs),

        // Base register only:
        // [base]
        // Example: [eax], [rbx]
        MemAddress::Base { base } =>
            encode_base(*base, data),

        // Base register + displacement:
        // [base + disp]
        // Example: [ebp - 4], [rax + 16]
        MemAddress::BaseDisp { base, disp } =>
            encode_base_disp(*base, disp, data, relocs),

        // Index register * scale + displacement:
        // [index * scale + disp]
        // Example: [ecx*4 + 8]
        MemAddress::IndexScaleDisp { index, scale, disp } =>
            encode_index_disp(*index, *scale, disp, data, relocs),

        // Base register + index register:
        // [base + index]
        // Example: [eax + ecx]
        MemAddress::BaseIndex { base, index } =>
            encode_base_index(*base, *index, data),

        // Base register + index register * scale:
        // [base + index * scale]
        // Example: [rax + rcx*8]
        MemAddress::BaseIndexScale { base, index, scale } =>
            encode_base_index_scale(*base, *index, *scale, data),

        // Full addressing form:
        // [base + index * scale + disp]
        // Example: [rbx + rsi*4 + 32]
        MemAddress::BaseIndexScaleDisp { base, index, scale, disp } =>
            encode_base_index_scale_disp(*base, *index, *scale, disp, data, relocs),
    }
}

///////////////////////////////////////////////////////////////////
/// 
/// 
/// 
///////////////////////////////////////////////////////////////////

pub(super) fn emit_rex(v: &mut Vec<u8>, size: Size, reg: Option<u8>, rm: Option<u8>) {
    let mut rex = 0x40;

    if let Size::U64 = size {
        rex |= 1 << 3; // W
    }

    if let Some(r) = reg {
        if r >= 8 {
            rex |= 1 << 2; // R
        }
    }

    if let Some(b) = rm {
        if b >= 8 {
            rex |= 1; // B
        }
    }

    if rex != 0x40 {
        v.push(rex);
    }
}

fn encode_reg(data: &mut Vec<u8>, rm: Register) {
    data[0] = (REG << 6) | (rm as u8);
}

fn encode_symbol(
    sym: &String,
    data: &mut Vec<u8>,
    relocs: &mut Vec<Relocation>,
) {
    data[0] = (data[0] & 0b11111000) | 0b101;

    let offset = data.len();

    data.extend(&0u32.to_le_bytes());

    relocs.push(Relocation {
        sym: sym.clone(),
        offset: offset,
        size: 4, // TOUJOURS 4 en RIP-relative
        kind: RelocKind::Relative,
        addend: -4, // très important pour RIP
    });
}

pub(super) fn mod_rm_encode( op1: &Operand, op2: &Operand )-> EncodeInformation
{
    let mut data = vec![0u8];
    let mut relocs = vec![];

    let reg_field = encode_modrm_field_reg(op2);

    match op1 {
        Operand::Reg(rm) => {
            encode_reg(&mut data, *rm);
        }

        Operand::MemoryAddress(mem) => {
            encode_memory(mem, &mut data, &mut relocs);
        }

        Operand::Sym(sym) => {
            encode_symbol(sym, &mut data, &mut relocs);
        }

        _ => panic!("Invalid operand for ModRM"),
    }

    // inject reg field
    data[0] = (data[0] & 0b11000111) | ((reg_field & 0b111) << 3);

    EncodeInformation {
        data,
        relocations: relocs,
    }
}