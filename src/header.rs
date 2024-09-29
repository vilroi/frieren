use std::fmt;

const EI_NIDENT: usize = 16;
const ELF_MAGIC: u32 = 0x464c457f;

#[derive(Debug, Default)]
#[repr(C)]
pub struct Ehdr {
    pub e_ident: [u8; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: usize,
    pub e_phoff: usize,
    pub e_shoff: usize,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

enum Class {
    ELFCLASSNONE,
    ELFCLASS32,
    ELFCLASS64,
}

enum Endianess {
    ELFDATANONE,
    ELFDATA2LSB,
    ELFDATA2MSB,
}

enum Version {
    NONE,
    CURRENT,
}


enum OsAbi {
    NONE = 0,
    HPUX,
    NETBSD,
    LINUX,
    SOLARIS = 6,
    AIX,
    IRIX,
    FREEBSD,
    TRU64,
    MODESTO,
    OPENBSD,
    ARM_AEABI = 64,
    ARM = 87,
    STANDALONE = 255
}

enum ElfType {
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
}

/*
pub struct ElfHeader {
    class: Class,
    endianess: Endianess,
    version: Version,
    os_abi: OsAbi,
    elf_type: ElfType,
}
*/

impl fmt::Display for Ehdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}


