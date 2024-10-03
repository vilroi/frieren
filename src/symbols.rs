use std::{
    fmt,
    io::*
};

#[derive(Debug, PartialEq)]
pub enum SymbolType {
    NoType = 0,
    Object,
    Function,
    Section,
    File,
    Common,
    Tls,
    Num,
    GnuIfunc = 10,
    HiOs = 12,
    LoProc,
    HiProc = 15
}

#[derive(Debug)]
pub enum SymbolBinding {
    Local,
    Global,
    Weak,
    Num,
    GnuUnique = 10,
    HiOs = 12,
    LoProc = 13,
    HiProc = 15,
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
    pub binding: SymbolBinding,
    pub symbol_type: SymbolType,
    pub other: u8,
    pub shndx: u16,
    pub value: usize,
    pub size: usize,
}

impl Symbol {
    pub fn from_elfsym_ptr(p: *const ElfSym) -> Result<Self> {
        unsafe {
            Ok(Symbol {
                name: String::new(),
                binding: info_to_binding((*p).st_info)?,
                symbol_type: info_to_type((*p).st_info)?,
                other: (*p).st_other,
                shndx: (*p).st_shndx,
                value: (*p).st_value,
                size: (*p).st_size,
            })
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


fn info_to_type(info: u8) -> Result<SymbolType> {
    let val = info & 0xf;
    let typ = match val {
        0 => SymbolType::NoType,
        1 => SymbolType::Object,
        2 => SymbolType::Function,
        3 => SymbolType::Section,
        4 => SymbolType::File,
        5 => SymbolType::Common,
        6 => SymbolType::Tls,
        7 => SymbolType::Num,
        10 => SymbolType::GnuIfunc,
        12 => SymbolType::HiOs,
        13 => SymbolType::LoProc,
        15 => SymbolType::HiProc,
        _ => return Err(Error::other(
                format!("Invalid symbol type {}", val))),
    };

    Ok(typ)
}

fn info_to_binding(info: u8) -> Result<SymbolBinding> {
    let val = info >> 4;
    let binding = match val { 
        0 => SymbolBinding::Local,
        1 => SymbolBinding::Global,
        2 => SymbolBinding::Weak,
        3 => SymbolBinding::Num,
        10 => SymbolBinding::GnuUnique,
        12 => SymbolBinding::HiOs,
        13 => SymbolBinding::LoProc,
        15 => SymbolBinding::HiProc,
        _ => return Err(Error::other(
                format!("Unsupported binding type: {}", val))),
    };

    Ok(binding)
}
