use std::fs::File;

pub trait Writable {
    fn write(&self, file: &mut File) -> std::io::Result<()>;
}
