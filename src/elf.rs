use std::{
    ptr,
    fmt,
};

const EI_NIDENT: usize = 16;
const ELF_MAGIC: u32 = 0x464c457f;

pub struct ElfPtr {
    pub header: *const ElfHeader,
}


#[derive(Debug)]
#[repr(C)]
pub struct ElfHeader {
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

impl fmt::Display for ElfHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}


impl ElfPtr {
    pub fn from_ptr(p: *const usize) -> Result<Self, &'static str> {
        let mut elf = ElfPtr {
            header: ptr::null(),
        };

        if !is_ptr_to_elf(p) {
            return Err("Pointer does not point to an elf file")
        }

        elf.header = p as *const ElfHeader;

        Ok(elf)
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



