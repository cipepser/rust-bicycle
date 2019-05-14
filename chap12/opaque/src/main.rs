use std::os::raw::{c_char, c_int};

enum File {}

extern "C" {
    fn fopen(fname: *const c_char, mode: *const c_char) -> *mut File;
    fn fgetc(stream: *mut File) -> c_int;
    fn fclose(stream: *mut File) -> c_int;
}


fn main() {
    unsafe {
        let fname: *const c_char = b"Cargo.toml\0".as_ptr() as *const _;
        let mode: *const c_char = b"r\0".as_ptr() as *const _;

        let file = fopen(fname, mode);
        if file.is_null() {
            println!("open file failed");
            return;
        }
        loop {
            let c = fgetc(file);
            if c == -1 {
                break;
            } else {
                let c = c as u8 as char;
                print!("{}", c);
            }
        }

        if fclose(file) == -1 {
            println!("close file failed");
        }
    }
}
