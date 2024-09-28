use std::{
    arch::asm,
    thread,
    time,
};

mod elf;
mod header;
mod section;
mod segment;

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

        if elf.sections.len() == 0 {
            println!("section headers not found");
        } else {
            for p in &elf.segments {
                println!("{p}");
            }
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
