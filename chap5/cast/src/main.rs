use std::mem::transmute;

fn main() {
    let p1 = Box::new(10);

//    let p2 = p1 as *mut i32; // error: non-primitive cast: `std::boxed::Box<i32>` as `*mut i32`

    let p3: *mut i32 = unsafe {
        transmute(p1)
    }; // no error. A Box pointer have a same memory size with *mut pointer.
}
