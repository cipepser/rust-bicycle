#[derive(Default)]
struct A {
    f0: u8,
    f1: u32,
    f2: u8,
}

fn main() {
    let a: A = Default::default();
    println!("struct A ({} bytes)\n  f0: {:p}\n  f1: {:p}\n  f2: {:p}\n",
             std::mem::size_of::<A>(),
             &a.f0,
             &a.f1,
             &a.f2,
    );

    //❯ rustc --version
    //rustc 1.34.0

    //struct A (8 bytes)
    //  f0: 0x7ffee7b03b44
    //  f1: 0x7ffee7b03b40
    //  f2: 0x7ffee7b03b45
}
