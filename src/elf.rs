use std::{
    ptr,
    io::Result,
    io::*,
    //mem,
    collections::HashMap,
    slice,
};

use crate::header::*;
use crate::section::*;
use crate::segment::*;
use crate::symbols::*;
use crate::dynamic::*;
use crate::utils::*;

//const EI_NIDENT: usize = 16;
const ELF_MAGIC: u32 = 0x464c457f;

#[derive(Debug, Default)]
pub struct Elf {
    data: Vec<u8>,
    header: Ehdr,
    sections: Vec<Section>,
    //section_map: HashMap<String, &'a Section>,
    segments: Vec<Segment>,
    symbols: HashMap<String, Symbol>,
    pub dynamic: Vec<Dynamic>,
}

impl Elf {
    pub fn open(path: &str) -> Result<Self> {
        let mut elf = Elf::default();
        elf.data = read_to_vec(path)?;
        let data = elf.get_raw_ptr();

        if !is_ptr_to_elf(data as *const u32) {
            return Err(Error::other(
                    format!("'{}' not a valid elf file", path))
                );
        }

        elf.header = unsafe { ptr::read(data as *const _) };
        elf.parse_segments();

        // If the section headers have been stripped, there is no point in continuing.
        // Return early.
        // TODO: Take into consideration tampered headers
        if elf.header.e_shoff == 0 || elf.header.e_shoff > elf.data.len() {
            return Ok(elf)
        }

        elf.parse_sections()?;
        elf.parse_symtab()?;

        // TODO: 
        if elf.header.e_type == ElfType::Dynamic as u16 {
            elf.parse_dynamic_section()?;
        }

        Ok(elf)
    }

    fn parse_segments(&mut self) {
        unsafe {
            let data = self.get_raw_ptr();
            let mut phdrp = data.offset(self.header.e_phoff as isize) as *const Phdr;

            self.segments.reserve_exact(self.header.e_phnum as usize);

            for _ in 0..self.header.e_phnum as usize{
                self.segments.push(Segment::from_phdr_ptr(phdrp));
                phdrp = phdrp.add(1);
            }
        }
    }

    fn parse_sections(&mut self) -> Result<()> {
        unsafe {
            let data = self.get_raw_ptr();
            let mut shdrp = data.offset(self.header.e_shoff as isize) as *const Shdr;

            self.sections.reserve_exact(self.header.e_shnum as usize);

            for _ in 0..self.header.e_shnum as usize{
                self.sections.push(Section::from_shdr_ptr(shdrp));
                shdrp = shdrp.add(1);
            }

            // Parse the section header string table, obtaining the names of sections. 
            // In the process, build the look up table to search Sections by name 
            let shstrtab = &self.sections[self.header.e_shstrndx as usize];
            let strp = data.offset(shstrtab.offset as isize);

            for s in &mut self.sections {
                let namep = strp.offset(s.name_offset as isize);
                s.name = c_str_to_string(namep)?;
            }
        }

        Ok(())
    }

    fn parse_symtab(&mut self) -> Result<()> {
        let mut symbols: HashMap<String, Symbol> = HashMap::new();
        let symtab = self.get_section_data::<ElfSym>(".symtab")?;
        let strtab = self.get_ptr_to_section(".strtab")?;

        for s in symtab {
            let mut symbol = Symbol::from_elfsym(s)?;
            let namep = unsafe { strtab.offset(s.st_name as isize) };

            symbol.name = c_str_to_string(namep)?;
            symbols.insert(
                symbol.name.clone(), 
                symbol
            );
        }

        self.symbols = symbols;
        Ok(())
    }

    fn parse_dynamic_section(&mut self) -> Result<()> {
        let data = self.get_section_data::<Dyn>(".dynamic")?;
        let mut dynamic: Vec<Dynamic> = Vec::new();

        for entry in data {
            dynamic.push(Dynamic::from_dyn(entry)?);
            if entry.d_tag == EntryType::Null as usize {
                break;
            }
        }

        self.dynamic = dynamic;

        Ok(())
    }

    fn get_ptr_to_section(&self, section_name: &str) -> Result<*const u8> {
        let mut ptr = self.get_raw_ptr();
        let section = match self.get_section_by_name(section_name) {
            Some(s) => s,
            None => return Err(
                Error::other(
                    format!("Failed to locate section: {section_name}")
                )),
        };

        ptr = unsafe { ptr.add(section.offset) };

        Ok(ptr)
    }

    fn get_section_data<T>(&self, section_name: &str) -> Result<&[T]> {
        let mut data = self.get_raw_ptr();
        let section = match self.get_section_by_name(section_name) {
            Some(s) => s,
            None => return Err(
                Error::other(
                    format!("Failed to locate section: {section_name}")
                )),
        };

        data = unsafe { data.offset(section.offset as isize) };
        let len = section.size / section.entsize;

        let slice = unsafe { slice::from_raw_parts(
            data as *const T,
            len
        )};

        Ok(slice)
    }

    pub fn get_section_by_name(&self, name: &str) -> Option<&Section> {
        for s in &self.sections {
            if s.name == *name {
                return Some(s);
            }
        }
        None
    }

    pub fn get_section_by_type(&self, typ: SectionType) -> impl Iterator<Item = &Section> {
        let typ = typ as u32;
        let vec: Vec<&Section> = self.sections.iter()
            .filter(|&s| s.typ == typ)
            .collect();

        vec.into_iter()
    }

    pub fn get_symbols_by_type(&self, typ: SymbolType) -> impl Iterator<Item = &Symbol> {
        let vec: Vec<&Symbol> = self.iter_symbols()
            .filter(|&s| s.symbol_type == typ)
            .collect();

        vec.into_iter()
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn iter_sections(&self) -> impl Iterator<Item = &Section> {
        self.sections.iter()
    }

    pub fn iter_segments(&self) -> impl Iterator<Item = &Segment> {
        self.segments.iter()
    }

    pub fn iter_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.values()
    }

    // TODO: does this always return a valid pointer?
    // https://doc.rust-lang.org/1.81.0/src/alloc/vec/mod.rs.html#1330 says never...?
    fn get_raw_ptr(&self) -> *mut u8 {
        self.data.as_ptr() as *mut u8
    }
}

fn is_ptr_to_elf(p: *const u32) -> bool {
    if p.is_null() {
        return false
    }

    unsafe {
        if *p == ELF_MAGIC { return true; } else {return false;}
    }
}
