use std::{
    arch::asm,
    thread,
    time,
};

mod elf;
mod header;
mod section;
mod segment;
mod symbols;

const PAGE_SIZE: usize = 0x1000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf() {
        let loadaddr = get_loadaddr();
        println!("load address: {:#x}", loadaddr as usize);
        
        let path = "/home/vilr0i/Projects/programming/frieren/test.bin";
        //let path = "/home/vilr0i/Projects/programming/frieren/test.bin.bad";
        let elf = elf::Elf::open(path).expect("test failed");

        let symtabs = elf.get_section_by_type(3);
        for s in symtabs {
            println!("{s}");
        }

        let relocs = elf.get_section_by_type(4);
        for r in relocs {
            println!("{r}");
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
