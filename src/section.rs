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
    pub name_offset: u32,
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
                name_offset: (*p).sh_name,
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

pub enum SectionType {
    Null = 	  0,		/* Section header table entry unused */
    Progbits = 	  1,		/* Program data */
    Symtab = 	  2,		/* Symbol table */
    Strtab = 	  3,		/* String table */
    Rela = 	  4,		/* Relocation entries with addends */
    Hash = 	  5,		/* Symbol hash table */
    Dynamic = 	  6,		/* Dynamic linking information */
    Note = 	  7,		/* Notes */
    NoBits = 	  8,		/* Program space with no data (bss) */
    Rel = 		  9,		/* Relocation entries, no addends */
    Shlib = 	  10,		/* Reserved */
    DynSym = 	  11,		/* Dynamic linker symbol table */
    InitArray = 	  14,		/* Array of constructors */
    FiniArray = 	  15,		/* Array of destructors */
    PreinitArray =  16,		/* Array of pre-constructors */
    Group = 	  17,		/* Section group */
    SymtabShndx =   18,		/* Extended section indices */
    Relr = 	  19,            /* RELR relative relocations */
    Num = 		  20,		/* Number of defined types.  */
    Loos = 	  0x60000000,	/* Start OS-specific.  */
    GnuAttributes =  0x6ffffff5,	/* Object attributes.  */
    GnuHash = 	  0x6ffffff6,	/* GNU-style hash table.  */
    GnuLiblist = 	  0x6ffffff7,	/* Prelink library list */
    Checksum = 	  0x6ffffff8,	/* Checksum for DSO content.  */
    Losunw = 	  0x6ffffffa,	/* Sun-specific low bound.  */
    SunwComdat =    0x6ffffffb,
    SunwSyminfo =   0x6ffffffc,
    GnuVerdef = 	  0x6ffffffd,	/* Version definition section.  */
    GnuVerneed = 	  0x6ffffffe,	/* Version needs section.  */
    GnuVersym = 	  0x6fffffff,	/* Version symbol table.  */
    LoProc = 	  0x70000000,	/* Start of processor-specific */
    HiProc = 	  0x7fffffff,	/* End of processor-specific */
    LoUser = 	  0x80000000,	/* Start of application-specific */
    HiUser = 	  0x8fffffff,	/* End of application-specific */
}
