use std::fmt;

#[derive(Debug, Default)]
#[repr(C)]
pub struct Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: usize,
    pub p_vaddr: usize,
    pub p_paddr: usize,
    pub p_filesz: usize,
    pub p_memsz: usize,
    pub p_align: usize,
}

impl fmt::Display for Phdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}

#[derive(Debug)]
pub struct Segment {
    typ: u32,
    flags: u32,
    offset: usize,
    vaddr: usize,
    paddr: usize,
    filesz: usize,
    memsz: usize,
    align: usize,
}

impl Segment {
    pub fn from_phdr_ptr(p: *const Phdr) -> Self {
        unsafe {
            Segment {
                typ: (*p).p_type,
                flags: (*p).p_flags,
                offset: (*p).p_offset,
                vaddr: (*p).p_vaddr,
                paddr: (*p).p_paddr,
                filesz: (*p).p_filesz,
                memsz: (*p).p_memsz,
                align: (*p).p_align,
            }
        }
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}

