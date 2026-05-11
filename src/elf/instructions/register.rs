#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterClass {
    Gpr,
    Xmm,
    Ymm,
    Zmm,
    Segment,
    Control,
    Debug,
    Mask,
    X87,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register {
    pub class: RegisterClass,
    pub index: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpr {
    A  = 0,
    C  = 1,
    D  = 2,
    B  = 3,
    Sp = 4,
    Bp = 5,
    Si = 6,
    Di = 7,

    R8  = 8,
    R9  = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

impl Register {
    pub const fn new(class: RegisterClass, index: u8) -> Self {
        Self { class, index }
    }

    #[inline]
    pub fn low3(self) -> u8 {
        self.index & 7
    }

    #[inline]
    pub fn rex_bit(self) -> u8 {
        (self.index >> 3) & 1
    }

    #[inline]
    pub fn is_gpr(self) -> bool {
        matches!(self.class, RegisterClass::Gpr)
    }

    #[inline]
    pub fn is_xmm(self) -> bool {
        matches!(self.class, RegisterClass::Xmm)
    }
}

impl From<Gpr> for Register {
    fn from(value: Gpr) -> Self {
        Register {
            class: RegisterClass::Gpr,
            index: value as u8,
        }
    }
}

impl TryFrom<u8> for Gpr {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0  => Ok(Gpr::A),
            1  => Ok(Gpr::C),
            2  => Ok(Gpr::D),
            3  => Ok(Gpr::B),
            4  => Ok(Gpr::Sp),
            5  => Ok(Gpr::Bp),
            6  => Ok(Gpr::Si),
            7  => Ok(Gpr::Di),

            8  => Ok(Gpr::R8),
            9  => Ok(Gpr::R9),
            10 => Ok(Gpr::R10),
            11 => Ok(Gpr::R11),
            12 => Ok(Gpr::R12),
            13 => Ok(Gpr::R13),
            14 => Ok(Gpr::R14),
            15 => Ok(Gpr::R15),

            _ => Err(()),
        }
    }
}

//
// Convenient aliases
//
pub const RAX: Register = Register::new(RegisterClass::Gpr, Gpr::A as u8);
pub const RCX: Register = Register::new(RegisterClass::Gpr, Gpr::C as u8);
pub const RDX: Register = Register::new(RegisterClass::Gpr, Gpr::D as u8);
pub const RBX: Register = Register::new(RegisterClass::Gpr, Gpr::B as u8);

pub const RSP: Register = Register::new(RegisterClass::Gpr, Gpr::Sp as u8);
pub const RBP: Register = Register::new(RegisterClass::Gpr, Gpr::Bp as u8);
pub const RSI: Register = Register::new(RegisterClass::Gpr, Gpr::Si as u8);
pub const RDI: Register = Register::new(RegisterClass::Gpr, Gpr::Di as u8);

pub const R8: Register  = Register::new(RegisterClass::Gpr, Gpr::R8 as u8);
pub const R9: Register  = Register::new(RegisterClass::Gpr, Gpr::R9 as u8);
pub const R10: Register = Register::new(RegisterClass::Gpr, Gpr::R10 as u8);
pub const R11: Register = Register::new(RegisterClass::Gpr, Gpr::R11 as u8);
pub const R12: Register = Register::new(RegisterClass::Gpr, Gpr::R12 as u8);
pub const R13: Register = Register::new(RegisterClass::Gpr, Gpr::R13 as u8);
pub const R14: Register = Register::new(RegisterClass::Gpr, Gpr::R14 as u8);
pub const R15: Register = Register::new(RegisterClass::Gpr, Gpr::R15 as u8);

//
// Future SIMD registers
//

pub const XMM0: Register  = Register::new(RegisterClass::Xmm, 0);
pub const XMM1: Register  = Register::new(RegisterClass::Xmm, 1);
pub const XMM2: Register  = Register::new(RegisterClass::Xmm, 2);
pub const XMM3: Register  = Register::new(RegisterClass::Xmm, 3);
pub const XMM4: Register  = Register::new(RegisterClass::Xmm, 4);
pub const XMM5: Register  = Register::new(RegisterClass::Xmm, 5);
pub const XMM6: Register  = Register::new(RegisterClass::Xmm, 6);
pub const XMM7: Register  = Register::new(RegisterClass::Xmm, 7);

pub const XMM8: Register  = Register::new(RegisterClass::Xmm, 8);
pub const XMM9: Register  = Register::new(RegisterClass::Xmm, 9);
pub const XMM10: Register = Register::new(RegisterClass::Xmm, 10);
pub const XMM11: Register = Register::new(RegisterClass::Xmm, 11);
pub const XMM12: Register = Register::new(RegisterClass::Xmm, 12);
pub const XMM13: Register = Register::new(RegisterClass::Xmm, 13);
pub const XMM14: Register = Register::new(RegisterClass::Xmm, 14);
pub const XMM15: Register = Register::new(RegisterClass::Xmm, 15);
