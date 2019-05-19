fn main() {
    let boxed = Box::new(true);
    let ptr: *mut bool = Box::into_raw(boxed);
    unsafe {
        let boxed = Box::from_raw(ptr);
    }
}