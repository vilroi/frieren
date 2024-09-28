use std::{
    ptr,
    io::Result,
    io::*,
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
    pub header: Ehdr,
    pub sections: Vec<Section>,
    pub segments: Vec<Segment>,
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

            let shstrtab = &self.sections[self.header.e_shstrndx as usize];
            let strp = data.offset(shstrtab.offset as isize);

            for s in &mut self.sections {
                let namep = strp.offset(s.name_offset as isize);
                s.name = c_str_to_string(namep)?;
            }
        }

        Ok(())
    }

    pub fn get_section_by_type(&self, typ: SectionType) -> impl Iterator<Item = &Section> {
        let typ = typ as u32;
        let vec: Vec<&Section> = self.sections.iter()
            .filter(|&s| s.typ == typ)
            .collect();

        vec.into_iter()
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
