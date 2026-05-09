use super::Relocation;


#[derive(Default)]
pub struct EncodeInformation {
    pub data: Vec<u8>,
    pub relocations: Vec<Relocation>,
}

#[allow(dead_code)]
impl EncodeInformation {

    // ==========================================
    // Constructors
    // ==========================================
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        Self {
            data,
            relocations: vec![],
        }
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            data: vec![byte],
            relocations: vec![],
        }
    }

    // ==========================================
    // Data helpers
    // ==========================================
    pub fn push(&mut self, byte: u8) {

        self.data.push(byte);

        for rel in &mut self.relocations {
            rel.addend -= 1;
        }
    }

    pub fn extend(&mut self, bytes: &[u8]) {

        self.data.extend(bytes);

        let delta = bytes.len() as i32;
        for rel in &mut self.relocations {
            rel.addend -= delta;
        }
    }

    pub fn extend_vec(&mut self, bytes: Vec<u8>) {

        let delta = bytes.len() as i32;
        self.data.extend(bytes);
        for rel in &mut self.relocations {
            rel.addend -= delta;
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // ==========================================
    // Relocation helpers
    // ==========================================
    pub fn add_and_shift_relocation(&mut self, mut relocation: Relocation)
    {
        relocation.offset += self.len()
    }

    pub fn add_relocation(&mut self, relocation: Relocation)
    {
        self.relocations.push(relocation);
    }

    pub fn extend_relocations(&mut self, relocs: Vec<Relocation>,)
    {
        self.relocations.extend(relocs);
    }

    pub fn shift_relocations(&mut self, offset: usize)
    {
        for rel in &mut self.relocations {
            rel.offset += offset;
        }
    }

    // ==========================================
    // Merge helpers
    // ==========================================
    pub fn append(&mut self, mut other: EncodeInformation) {

        let base = self.data.len();

        for rel in &mut other.relocations {
            rel.offset += base;
        }

        self.data.extend(other.data);
        self.relocations.extend(other.relocations);
    }

    pub fn prepend_bytes(&mut self, bytes: &[u8]) {

        let mut new_data =
            Vec::with_capacity(bytes.len() + self.data.len());

        new_data.extend(bytes);
        new_data.extend(&self.data);

        self.shift_relocations(bytes.len());

        self.data = new_data;
    }
}
