use super::register::{Register};

#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Scale {
    #[default]
    One = 0,
    Two = 1,
    Four = 2,
    Eight = 3,
}

#[derive(Default, Debug, Clone)]
pub enum AddressingMode {
    #[default]
    Default,
    RipRelative,
    Absolute,
}

#[derive(Debug, Clone)]
pub enum MemDisplacement {
    Imm(i32),
    Sym(String),
}

#[derive(Debug, Default, Clone)]
pub struct MemAddress {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub scale: Scale,
    pub disp: MemDisplacement,
    pub mode: AddressingMode,
}

impl Default for MemDisplacement {
    fn default() -> Self {
        MemDisplacement::Imm(0)
    }
}

#[allow(dead_code)]
impl MemAddress {

    // ==========================================
    // Constructors
    // ==========================================

    pub fn new() -> Self {
        Self::default()
    }

    pub fn direct(disp: i32) -> Self {
        Self {
            disp: MemDisplacement::Imm(disp),
            ..Default::default()
        }
    }

    pub fn symbol(sym: impl Into<String>) -> Self {
        Self {
            disp: MemDisplacement::Sym(sym.into()),
            ..Default::default()
        }
    }

    // ==========================================
    // Addressing mode
    // ==========================================

    pub fn rip_relative(mut self) -> Self {
        self.mode = AddressingMode::RipRelative;
        self
    }

    pub fn absolute(mut self) -> Self {
        self.mode = AddressingMode::Absolute;
        self
    }

    pub fn mode(mut self, mode: AddressingMode) -> Self {
        self.mode = mode;
        self
    }

    // ==========================================
    // Base register
    // ==========================================

    pub fn base(mut self, reg: Register) -> Self {
        self.base = Some(reg);
        self
    }

    // ==========================================
    // Index register
    // ==========================================

    pub fn index(mut self, reg: Register) -> Self {
        self.index = Some(reg);
        self
    }

    pub fn index_scale(mut self, reg: Register, scale: Scale) -> Self {
        self.index = Some(reg);
        self.scale = scale;
        self
    }

    // ==========================================
    // Scale only
    // ==========================================

    pub fn scale(mut self, scale: Scale) -> Self {
        self.scale = scale;
        self
    }

    // ==========================================
    // Displacement
    // ==========================================

    pub fn disp(mut self, disp: i32) -> Self {
        self.disp = MemDisplacement::Imm(disp);
        self
    }

    pub fn sym(mut self, sym: impl Into<String>) -> Self {
        self.disp = MemDisplacement::Sym(sym.into());
        self
    }
}
