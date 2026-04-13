
#[allow(dead_code)]
pub struct SystemInfo {
    pub arch: String,
    pub _os: String,
    pub endian: u8,
}

#[allow(dead_code)]
pub fn get_system_info() -> SystemInfo {
    let arch = std::env::consts::ARCH.to_string();
    let _os = std::env::consts::OS.to_string();

    let endian = if cfg!(target_endian = "little") {
        1 // ELFDATA2LSB
    } else {
        2 // ELFDATA2MSB
    };

    SystemInfo { arch, _os, endian }
}

#[allow(dead_code)]
pub fn get_machine(arch: &str) -> u16 {
    match arch {
        "x86_64" => 0x3E, // EM_X86_64
        "x86" | "i686" => 0x03, // EM_386
        "aarch64" => 0xB7, // EM_AARCH64
        _ => 0,
    }
}
