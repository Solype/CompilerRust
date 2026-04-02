use std::fs::File;
use std::io::Write;
use std::slice;

pub trait Writable {
    fn write(&self, file: &mut File) -> std::io::Result<()>;
}

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

pub trait WriteBytes {
    fn write_le<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()>;
}

macro_rules! impl_write_bytes {
    ($($t:ty),*) => {
        $(
            impl WriteBytes for $t {
                fn write_le<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
                    w.write_all(&self.to_le_bytes())
                }
            }
        )*
    };
}

impl_write_bytes!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
