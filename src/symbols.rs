use std::fmt;

pub enum SymbolType {
    NoType = 0,
    Object,
    Function,
    SEction,
    File,
    Common,
    Tls,
    Num,
    GnuIfunc = 10,
    HiOs = 12,
    LoProc,
    HiProc,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ElfSym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: usize,
    pub st_size: usize,
}

impl fmt::Display for ElfSym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub name: String,
    pub name_offset: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub value: usize,
    pub size: usize,
}

impl Symbol {
    pub fn from_elfsym_ptr(p: *const ElfSym) -> Self {
        unsafe {
            Symbol {
                name: String::new(),
                name_offset: (*p).st_name,
                info: (*p).st_info,
                other: (*p).st_other,
                shndx: (*p).st_shndx,
                value: (*p).st_value,
                size: (*p).st_size,
            }
        }
    }

    pub fn within_range(&self, val: usize) -> bool {
        let end = self.value + self.size;
        val >= self.value && val < end
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}


