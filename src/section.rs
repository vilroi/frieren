use std::fmt;

#[derive(Debug, Default)]
#[repr(C)]
pub struct Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: usize,
    pub sh_addr: usize,
    pub sh_offset: usize,
    pub sh_size: usize,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: usize,
    pub sh_entsize: usize,
}

impl fmt::Display for Shdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}

#[derive(Debug)]
pub struct Section {
    pub name: String,
    pub typ: u32,
    pub flags: usize,
    pub addr: usize,
    pub offset: usize,
    pub size: usize,
    pub link: u32,
    pub info: u32,
    pub addralign: usize,
    pub entsize: usize,
}

impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x?}", self)
    }
}

impl Section {
    pub fn from_shdr_ptr(p: *const Shdr) -> Section {
        unsafe {
            Section {
                name: String::new(),
                typ: (*p).sh_type,
                flags: (*p).sh_flags,
                addr: (*p).sh_addr,
                offset: (*p).sh_offset,
                size: (*p).sh_size,
                link: (*p).sh_link,
                info: (*p).sh_info,
                addralign: (*p).sh_addralign,
                entsize: (*p).sh_entsize,
            }
        }
    }
}
