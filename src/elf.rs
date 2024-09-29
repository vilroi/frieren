use std::{
    ptr,
    io::Result,
    io::*,
    mem,
    //collections::HashMap,
};

use crate::header::*;
use crate::section::*;
use crate::segment::*;
use crate::symbols::*;
use crate::utils::*;

const EI_NIDENT: usize = 16;
const ELF_MAGIC: u32 = 0x464c457f;

#[derive(Debug, Default)]
pub struct Elf {
    data: Vec<u8>,
    header: Ehdr,
    sections: Vec<Section>,
    //section_map: HashMap<String, &'a Section>,
    segments: Vec<Segment>,
    symbols: Vec<Symbol>,
}

impl Elf {
    pub fn open(path: &str) -> Result<Self> {
        let mut elf = Elf::default();
        elf.data = read_to_vec(path)?;
        let data = elf.get_raw_ptr();

        if !is_ptr_to_elf(data as *const u32) {
            return Err(Error::other("'{}' not a valid elf file"));
        }

        unsafe {
            elf.header = ptr::read(data as *const _);
            elf.parse_segments();
            /* 
             * If the section headers have been stripped, there is no point in continuing.
             * Return early.
             * TODO: Take into consideration tampered headers
             */
            if elf.header.e_shoff == 0 || elf.header.e_shoff > elf.data.len() {
                return Ok(elf)
            }

            elf.parse_sections()?;
            elf.parse_symtab()?;
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

            /* Parse the section header string table, obtaining the names of sections. 
             * In the process, build the look up table to search Sections by name */
            let shstrtab = &self.sections[self.header.e_shstrndx as usize];
            let strp = data.offset(shstrtab.offset as isize);

            for s in &mut self.sections {
                let namep = strp.offset(s.name_offset as isize);
                s.name = c_str_to_string(namep)?;

                //self.section_map.insert(s.name.clone(), s);
            }
        }

        Ok(())
    }

    fn parse_symtab(&mut self) -> Result<()> {
        unsafe {
            let data = self.get_raw_ptr();
            let symtab_section = self.get_section_by_name(".symtab")
                .ok_or(Error::other("failed to locate symtab"))?;

            let strtab_section = self.get_section_by_name(".strtab")
                .ok_or(Error::other("failed to locate strtab"))?;

            let mut symtab = data.offset(symtab_section.offset as isize) as *const ElfSym;
            let nsymbols = symtab_section.size / mem::size_of::<ElfSym>();
            
            let strtab = data.offset(strtab_section.offset as isize) as *const u8;

            for i in 0..nsymbols {
                self.symbols.push(Symbol::from_elfsym_ptr(symtab));

                let namep = strtab.offset((*symtab).st_name as isize);
                self.symbols[i].name = c_str_to_string(namep)?;
                symtab = symtab.add(1);
            }

            Ok(())
        }
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

    pub fn iter_sections(&self) -> impl Iterator<Item = &Section> {
        self.sections.iter()
    }

    pub fn iter_segments(&self) -> impl Iterator<Item = &Segment> {
        self.segments.iter()
    }

    pub fn iter_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.iter()
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
