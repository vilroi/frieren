use std::{
    fs,
    io::*,
    slice,
};

pub fn read_to_vec(path: &str) -> Result<Vec<u8>> {
    let mut f = fs::File::open(path)?;
    let mut vec = Vec::new();

    f.read_to_end(&mut vec)?;

    Ok(vec)
}

pub fn c_str_to_string(ptr: *const u8) -> Result<String> {
    unsafe {
        if ptr.is_null() {
            return Err(Error::other("null pointer supplied"));
        }

        let mut p = ptr;
        let mut s = String::new();
        while *p != 0x0 {
            s.push(*p as char);
            p = p.add(1);

            if p.is_null() {
                return Err(Error::other("Null pointer reached in loop"));
            }
        }

        Ok(s)
    }
}

