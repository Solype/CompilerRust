use super::{
    enums::*,
    modrm::*,
    register::{RAX, RSI, Register},
    struct_encode_information::*,
    utils::{emit_rex, emit_size_prefix},
};

///////////////////////////////////////////////////////////////////

fn encode_with_reg(opcode: u8, reg: Register, size: Size) -> EncodeInformation {
    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, &Operand::Reg(reg));

    v.push(opcode + reg.low3());

    v
}

///////////////////////////////////////////////////////////////////

fn encode_mem_address(
    opcode: u8,
    value: &Operand,
    size: Size,
    regfield: Register,
) -> EncodeInformation {
    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);
    emit_rex(&mut v, size, None, &value);

    v.push(opcode);

    let reg_field = Operand::Reg(regfield);

    let modrm = mod_rm_encode(value, &reg_field);

    v.append(modrm);

    v
}

///////////////////////////////////////////////////////////////////

pub fn encode_simple(opcode: u8, size: Size) -> EncodeInformation {
    let mut v = EncodeInformation::new();

    emit_size_prefix(&mut v, size);

    v.push(opcode);

    v
}

///////////////////////////////////////////////////////////////////

pub(super) fn encode_stack(op: StackOp, value: &Operand, size: Size) -> EncodeInformation {
    match op {
        // =====================================================
        // PUSHF / POPF
        // =====================================================
        StackOp::Pushf => encode_simple(0x9C, size),

        StackOp::Popf => encode_simple(0x9D, size),

        // =====================================================
        // ENTER
        // =====================================================
        StackOp::Enter(nesting_lv) => {
            let mut v = Vec::new();

            let frame_size = match value {
                Operand::Imm(v) => *v,

                _ => {
                    panic!("ENTER requires immediate frame size");
                }
            };

            if !(0x0 <= frame_size && frame_size <= 0xFFFF) {
                panic!("ENTER frame size must fit in imm16");
            }

            if nesting_lv > 31 {
                panic!("ENTER nesting level must be <= 31");
            }

            v.push(0xC8);

            v.extend(&(frame_size as u16).to_le_bytes());

            v.push(nesting_lv);

            return EncodeInformation {
                data: v,
                ..Default::default()
            };
        }

        // =====================================================
        // LEAVE
        // =====================================================
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
            // -------------------------------------------------
            // push reg
            // -------------------------------------------------
            Operand::Reg(reg) => encode_with_reg(0x50, *reg, size),

            // -------------------------------------------------
            // push imm
            // -------------------------------------------------
            Operand::Imm(val) => {
                let mut v = EncodeInformation::new();

                emit_size_prefix(&mut v, size);

                if *val <= 0x7F {
                    v.push(0x6A);
                    v.push(*val as u8);
                } else {
                    v.push(0x68);
                    match size {
                        Size::U16 => v.extend(&(*val as u16).to_le_bytes()),
                        _ => v.extend(&(*val as u32).to_le_bytes()),
                    }
                }
                v
            }

            // -------------------------------------------------
            // push symbol
            // -------------------------------------------------
            Operand::Sym(sym) => {
                let mut v = EncodeInformation::new();

                v.push(0x68);

                let offset = v.len();

                v.extend(&0u32.to_le_bytes());

                v.add_relocation(Relocation {
                    sym: sym.clone(),
                    offset,
                    size: 4,
                    kind: RelocKind::AbsoluteSigned,
                    addend: 0,
                });

                v
            }

            // -------------------------------------------------
            // push r/m
            // FF /6
            // -------------------------------------------------
            Operand::MemoryAddress(_) => encode_mem_address(0xFF, value, size, RSI),

            _ => {
                panic!("unsupported PUSH operand {:?}", value)
            }
        },

        // =====================================================
        // POP
        // =====================================================
        StackOp::Pop => match value {
            // -------------------------------------------------
            // pop reg
            // -------------------------------------------------
            Operand::Reg(reg) => encode_with_reg(0x58, *reg, size),

            // -------------------------------------------------
            // pop r/m
            // 8F /0
            // -------------------------------------------------
            Operand::MemoryAddress(_) => encode_mem_address(0x8F, value, size, RAX),

            _ => {
                panic!("unsupported POP operand {:?}", value)
            }
        },
    }
}
