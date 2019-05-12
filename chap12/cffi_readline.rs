use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_schar;

#[link(name = "readline")]
extern "C" {
    fn readline(prompt: *const c_schar) -> *mut c_schar;
}

fn main() {
    unsafe {
        let prompt = CString::new("> ").unwrap();
        loop {
            let input = CStr::from_ptr(readline(prompt.as_ptr()));
            let input = input.to_str().expect("input contains invalid unicode");
            if input == "exit" {
                break;
            }

            println!("your input is {}", input);
        }
    }
}