use super::super::traits::ElfWritable;

#[derive(Default)]
pub struct Section {
    data: Vec<u8>,
    padding : usize,
}

impl Section {
    pub fn new(data : Vec<u8>) -> Self
    {
        Self {
            data: data,
            padding : 0,
        }
    }

    pub fn set_padding(&mut self, padding : usize) -> &mut Self
    {
        self.padding = padding;
        self
    }

    pub fn get_data(&self) -> &Vec<u8>
    {
        &self.data
    }

    pub fn add_data(&mut self, other: &Vec<u8>) -> &mut Self
    {
        self.data.extend(other);
        self
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<u8>
    {
        &mut self.data
    }

    pub fn set_data(&mut self, other: Vec<u8>) -> &mut Self
    {
        self.data = other;
        self
    }
}

impl ElfWritable for Section
{
    fn write<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&vec![0u8; self.padding as usize])?;
        writer.write_all(&self.data)?;
        Ok(())
    }
}
