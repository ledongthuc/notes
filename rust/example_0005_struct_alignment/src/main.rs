use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
struct Foo {
    a: u8,
    b: u32,
    c: u64,
}

fn main() {
    println!("size = {}", size_of::<Foo>());
    println!("align = {}", align_of::<Foo>());

    println!("a offset = {}", offset_of!(Foo, a));
    println!("b offset = {}", offset_of!(Foo, b));
    println!("c offset = {}", offset_of!(Foo, c));
}
