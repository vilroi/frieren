use std::{
    arch::asm,
    thread,
    time,
};

pub mod elf;
pub mod header;
pub mod section;
pub mod segment;
pub mod symbols;
pub mod dynamic;
pub mod utils;

#[macro_use]
extern crate num_derive;

 use crate::symbols::SymbolType;
 use crate::section::SectionType;

 const PAGE_SIZE: usize = 0x1000;

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn test_elf() {
        let loadaddr = get_loadaddr();
        println!("load address: {:#x}", loadaddr as usize);
        
        let path = "test.bin";
        let elf = elf::Elf::open(path).expect("test failed");

        let symtabs = elf.get_section_by_type(SectionType::Strtab);
        for s in symtabs {
            println!("{s}");
        }
    }

    //#[test]
    fn test_sections() {
        let path = "test.bin";
        let elf = elf::Elf::open(path).expect("test failed");
        
        let section = String::from(".go");
        match elf.get_section_by_name(&section) {
            Some(s) => println!("{s}"),
            None => println!("{section} does not exist"),
        }
    }

    //#[test]
    fn test_symbols() {
        let path = "test.bin";
        let elf = elf::Elf::open(path)
            .expect("failed to open {path}");

        let mut count = 0;
        for s in elf.iter_symbols() {
            println!("name: {}, value: {:#x}", s.name, s.value);
            count += 1;
        }

    }

    #[test]
    fn test_get_symbols() {
        let path = "./testbins/rustbin";
        let elf = elf::Elf::open(path)
            .expect("failed to open {path}");

        let mut count = 0;
        for sym in elf.get_symbols_by_type(SymbolType::Function) {
            println!("func: {}", sym.name);
            count += 1;
        }

        //assert!(count == 19);

        let name = "_start";
        let sym = elf.get_symbol(name)
            .expect(&String::from(format!("Failed to find symbol: '{name}'")));
        println!("{}", sym);
    }

    //#[test]
    fn test_dynamic_section() {
        let path = "./testbins/rustbin";
        let elf = elf::Elf::open(path)
            .expect("failed to open {path}");

        for d in elf.dynamic {
            println!("{:?}: {:#x}", d.tag, d.val);
        }

    }
}

#[inline(always)]
fn get_ip() -> usize {
    let ip: usize;

    unsafe { asm!("lea   {}, [rip]", out(reg) ip); }
    ip
}

fn get_loadaddr() -> *mut usize {
    let ip = get_ip() & !(PAGE_SIZE -1);
    let mut p = ip as *const u32;
    let magic = 0x464c457f;         // ELF Magic bytes, little endian

    loop {
        unsafe {
            if *p == magic {
                return p as *mut usize;
            }
        }
        p = (p as usize - PAGE_SIZE) as *const u32;
    }
}

fn sleep(secs: u64) {
    let dur = time::Duration::from_secs(secs);
    thread::sleep(dur);
}
