use std::{
    ptr,
    fmt,
    mem,
    fs,
    io::Result,
    io::*,
    slice,
};

const EI_NIDENT: usize = 16;
const ELF_MAGIC: u32 = 0x464c457f;

#[derive(Debug, Default)]
pub struct Elf {
    pub header: Ehdr,
    pub sections: Vec<Section>,
    pub segments: Vec<Phdr>,
}

impl Elf {
    pub fn open(path: &str) -> Result<Self> {
        let mut elf = Elf::default();
        let data = read_to_vec(path)?;
        let data = data.as_ptr();

        unsafe {
            elf.header = ptr::read(data as *const _);

            elf.sections.reserve_exact(elf.header.e_shnum as usize);
            elf.segments.reserve_exact(elf.header.e_phnum as usize);

            /* parse section headers */
            let mut shdrp = data.offset(elf.header.e_shoff as isize) as *const Shdr;
            for _ in 0..elf.header.e_shnum as usize{
                elf.sections.push(Section::from_shdr_ptr(shdrp));
                shdrp = shdrp.add(1);
            }

            /* parse program headers */
            /*
            let mut phdrp = data.offset(elf.header.e_phoff as isize) as *const Phdr;
            for _ in 0..elf.header.e_phnum as usize{
                elf.segments.push(ptr::read(phdrp as *const _));
                phdrp = phdrp.add(1);
            }
            */
            
        }

        Ok(elf)
    }

}

fn read_to_vec(path: &str) -> Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut vec = Vec::new();

    f.read_to_end(&mut vec)?;

    Ok(vec)
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct Ehdr {
    e_ident: [u8; EI_NIDENT],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: usize,
    e_phoff: usize,
    e_shoff: usize,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
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

pub struct ElfHeader {
    class: Class,
    endianess: Endianess,
    version: Version,
    os_abi: OsAbi,
    elf_type: ElfType,
}

impl fmt::Display for Ehdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}


#[derive(Debug, Default)]
#[repr(C)]
pub struct Shdr {
    sh_name: u32,
    sh_type: u32,
    sh_flags: usize,
    sh_addr: usize,
    sh_offset: usize,
    sh_size: usize,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: usize,
    sh_entsize: usize,
}

impl fmt::Display for Shdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: usize,
    p_vaddr: usize,
    p_paddr: usize,
    ip_filesz: usize,
    p_memsz: usize,
    p_align: usize,
}

impl fmt::Display for Phdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}

#[derive(Debug)]
pub struct Section {
    name: String,
    typ: u32,
    flags: usize,
    addr: usize,
    offset: usize,
    size: usize,
    link: u32,
    info: u32,
    addralign: usize,
    entsize: usize,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}

impl Section {
    fn from_shdr_ptr(shdrp: *const Shdr) -> Section {
        unsafe {
            Section {
                name: String::new(),
                typ: (*shdrp).sh_type,
                flags: (*shdrp).sh_flags,
                addr: (*shdrp).sh_addr,
                offset: (*shdrp).sh_offset,
                size: (*shdrp).sh_size,
                link: (*shdrp).sh_link,
                info: (*shdrp).sh_info,
                addralign: (*shdrp).sh_addralign,
                entsize: (*shdrp).sh_entsize,
            }
        }
    }
}

fn is_ptr_to_elf(p: *const usize) -> bool {
    if p.is_null() {
        return false
    }

    let p = p as *const u32;
    unsafe {
        if *p == ELF_MAGIC { return true; } else {return false;}
    }
}



