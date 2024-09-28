use std::{
    ptr,
    fs,
    io::Result,
    io::*,
};

use crate::header::*;
use crate::section::*;
use crate::segment::*;
use crate::symbols::*;

const EI_NIDENT: usize = 16;
const ELF_MAGIC: u32 = 0x464c457f;

#[derive(Debug, Default)]
pub struct Elf {
    pub header: Ehdr,
    pub sections: Vec<Section>,
    pub segments: Vec<Segment>,
}

impl Elf {
    pub fn open(path: &str) -> Result<Self> {
        let mut elf = Elf::default();
        let data_vec = read_to_vec(path)?;
        let data = data_vec.as_ptr();

        if !is_ptr_to_elf(data as *const u32) {
            return Err(Error::other("'{}' not a valid elf file"));
        }

        unsafe {
            elf.header = ptr::read(data as *const _);

            /* parse program headers */
            let mut phdrp = data.offset(elf.header.e_phoff as isize) as *const Phdr;
            elf.segments.reserve_exact(elf.header.e_phnum as usize);

            elf.segments.reserve_exact(elf.header.e_phnum as usize);
            for _ in 0..elf.header.e_phnum as usize{
                elf.segments.push(Segment::from_phdr_ptr(phdrp));
                phdrp = phdrp.add(1);
            }

            /* 
             * If the section headers have been stripped,
             * there is no point in continuing.
             * Return early.
             * TODO: Take into consideration tampered headers
             */
            if elf.header.e_shoff == 0 || elf.header.e_shoff > data_vec.len() {
                return Ok(elf)
            }

            /* parse section headers */
            let mut shdrp = data.offset(elf.header.e_shoff as isize) as *const Shdr;
            elf.sections.reserve_exact(elf.header.e_shnum as usize);

            for _ in 0..elf.header.e_shnum as usize{
                elf.sections.push(Section::from_shdr_ptr(shdrp));
                shdrp = shdrp.add(1);
            }
        }

        Ok(elf)
    }

    pub fn get_section_by_type(&self, typ: SectionType) -> impl Iterator<Item = &Section> {
        let typ = typ as u32;
        let vec: Vec<&Section> = self.sections.iter()
            .filter(|&s| s.typ == typ)
            .collect();

        vec.into_iter()
    }
}

fn read_to_vec(path: &str) -> Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut vec = Vec::new();

    f.read_to_end(&mut vec)?;

    Ok(vec)
}

fn is_ptr_to_elf(p: *const u32) -> bool {
    if p.is_null() {
        return false
    }

    unsafe {
        if *p == ELF_MAGIC { return true; } else {return false;}
    }
}
