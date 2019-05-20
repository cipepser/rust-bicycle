use libc::{suseconds_t, time_t};
use std::mem;
use std::os::raw::c_int;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
struct Timeval {
    tv_sec: time_t,
    tv_usec: suseconds_t,
}

#[repr(C)]
#[derive(Debug)]
struct Timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

extern "C" {
    fn gettimeofday(tv: *mut Timeval, tz: *mut Timezone) -> c_int;
}

fn main() {
    unsafe {
        let mut tv: Timeval = mem::uninitialized();
        let tx: *mut Timezone = ptr::null_mut();
        let ret = gettimeofday(&mut tv as *mut _, tx);
        if ret == -1 {
            println!("failure");
            return;
        }
        println!("{:?}", tv);
    }
}
