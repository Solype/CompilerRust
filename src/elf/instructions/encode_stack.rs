use super::{
    modrm::*,
    enums::*,
    encode_alu::emit_size_prefix,
    struct_encode_information::*,
};


fn encode_with_reg(opcode: u8, reg_u8: u8, size: Size) -> EncodeInformation
{
    let mut v = Vec::new();
    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, Some(reg_u8));
    v.push(opcode + (reg_u8 & 7));

    EncodeInformation {
        data: v,
        ..Default::default()
    }
}

fn encode_mem_address(opcode: u8, value: &Operand, size: Size, regfield: Register) -> EncodeInformation
{
    let mut v = Vec::new();
    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, None);

    v.push(opcode);

    let reg_field = Operand::Reg(regfield);

    let base = v.len();
    let modrm = mod_rm_encode(value, &reg_field);

    v.extend(modrm.data);

    let relocations = modrm.relocations.into_iter().map(|mut r| {
            r.offset += base;
            r
        }).collect();

    EncodeInformation {
        data: v,
        relocations,
    }
}

pub fn encode_simple(opcode: u8, size: Size) -> EncodeInformation
{
    let mut v = Vec::new();

    emit_size_prefix(&mut v, size);
    v.push(opcode);
    EncodeInformation {
        data: v,
        ..Default::default()
    }
}

pub(super) fn encode_stack(
    op: StackOp,
    value: &Operand,
    size: Size,
) -> EncodeInformation {

    match op {
        StackOp::Pushf => encode_simple(0x9C, size),
        StackOp::Popf => encode_simple(0x9D, size),

        StackOp::Enter(nesting_lv) => {
            let mut v = Vec::new();
            let frame_size = match value {
                Operand::Imm(v) => *v,
                _ => panic!("ENTER requires immediate frame size"),
            };

            if !(frame_size <= 0xFFFF) { panic!("ENTER frame size must fit in imm16"); }
            if nesting_lv > 31 { panic!("ENTER nesting level must be <= 31"); }

            v.push(0xC8);
            v.extend(&(frame_size as u16).to_le_bytes());
            v.push(nesting_lv);

            return EncodeInformation {
                data: v,
                ..Default::default()
            };
        }

        // ==========================================
        // LEAVE
        // C9
        // ==========================================
        StackOp::Leave => {
            return EncodeInformation {
                data: vec![0xC9],
                ..Default::default()
            };
        }

        // =====================================================
        // PUSH
        // =====================================================
        StackOp::Push => match value {
            // ---------------------------------------------
            // push reg
            // opcode = 50 + reg
            // ---------------------------------------------
            Operand::Reg(reg) => encode_with_reg(0x50, *reg as u8, size),

            // ---------------------------------------------
            // push imm
            // 6A ib
            // 68 iw/id
            // ---------------------------------------------
            Operand::Imm(val) => {
                let mut v = Vec::new();
                emit_size_prefix(&mut v, size);
                if *val <= 0x7F {
                    v.push(0x6A);
                    v.push(*val as u8);
                } else {
                    v.push(0x68);
                    match size {
                        Size::U16 => v.extend((*val as u16).to_le_bytes()),
                        _ => v.extend((*val as u32).to_le_bytes()),
                    }
                }

                EncodeInformation { data: v, ..Default::default() }
            }

            // ---------------------------------------------
            // push symbol
            // push imm32 reloc
            // ---------------------------------------------
            Operand::Sym(sym) => {
                let mut v = Vec::new();
                v.push(0x68);
                let offset = v.len();
                v.extend(0u32.to_le_bytes());

                EncodeInformation {
                    data: v,
                    relocations: vec![Relocation { sym: sym.clone(), offset, size: 4, kind: RelocKind::Absolute, addend: 0, }],
                }
            }

            // ---------------------------------------------
            // push r/m
            // FF /6
            // ---------------------------------------------
            Operand::MemoryAddress(_) => encode_mem_address(0xFF, value, size, Register::Si),

            _ => panic!("unsupported PUSH operand {:?}", value),
        },

        // =====================================================
        // POP
        // =====================================================
        StackOp::Pop => match value {
            // ---------------------------------------------
            // pop reg
            // 58 + reg
            // ---------------------------------------------
            Operand::Reg(reg) => encode_with_reg(0x58, *reg as u8, size),

            // ---------------------------------------------
            // pop r/m
            // 8F /0
            // ---------------------------------------------
            Operand::MemoryAddress(_) => encode_mem_address(0x8F, value, size, Register::A),

            _ => panic!("unsupported POP operand {:?}", value),
        },
    }
}