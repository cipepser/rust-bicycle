use std::os::raw::c_int;

#[link(name = "ownership", kind = "static")]
extern "C" {
    fn make_memory() -> *mut c_int;
}

fn main() {
    unsafe {
        let i = make_memory();

        println!("got {}", *i);

        libc::free(i as *mut _);
    }
}