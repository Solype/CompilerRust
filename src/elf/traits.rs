use std::fs::File;
use std::io::Write;
use std::slice;

pub trait RawWritable {
    fn write_raw(&self, file: &mut File) -> std::io::Result<()>;
}

impl<T> RawWritable for T where T: Copy,
{
    fn write_raw(&self, file: &mut File) -> std::io::Result<()> {
        unsafe {
            let ptr = self as *const _ as *const u8;
            let bytes = slice::from_raw_parts(ptr, size_of::<Self>());
            file.write_all(bytes)
        }
    }
}
