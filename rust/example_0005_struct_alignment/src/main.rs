use std::mem::{align_of, offset_of, size_of};

fn main() {
    print_FooReprC();
    print_FooReprRust();
    print_FooReprPacked();
    print_FooReprAlign();
}

#[repr(C)]
struct FooReprC {
    a: u8,
    b: u32,
    c: u64,
}

fn print_FooReprC() {
    println!("#[repr(C)]");
    println!("size = {}", size_of::<FooReprC>());
    println!("align = {}", align_of::<FooReprC>());

    println!("a offset = {}", offset_of!(FooReprC, a));
    println!("b offset = {}", offset_of!(FooReprC, b));
    println!("c offset = {}", offset_of!(FooReprC, c));
}

struct FooReprRust {
    a: u8,
    b: u32,
    c: u64,
}

fn print_FooReprRust() {
    println!("#[repr(Rust)]");
    println!("size = {}", size_of::<FooReprRust>());
    println!("align = {}", align_of::<FooReprRust>());

    println!("a offset = {}", offset_of!(FooReprRust, a));
    println!("b offset = {}", offset_of!(FooReprRust, b));
    println!("c offset = {}", offset_of!(FooReprRust, c));
}

#[repr(packed)]
struct FooReprPacked {
    a: u8,
    b: u32,
    c: u64,
}

fn print_FooReprPacked() {
    println!("#[repr(packed)]");
    println!("size = {}", size_of::<FooReprPacked>());
    println!("align = {}", align_of::<FooReprPacked>());

    println!("a offset = {}", offset_of!(FooReprPacked, a));
    println!("b offset = {}", offset_of!(FooReprPacked, b));
    println!("c offset = {}", offset_of!(FooReprPacked, c));
}

#[repr(align(1024), C)]
struct FooReprAlign {
    a: u8,
    b: u32,
    c: u64,
}

fn print_FooReprAlign() {
    println!("#[repr(align(1024))]");
    println!("size = {}", size_of::<FooReprAlign>());
    println!("align = {}", align_of::<FooReprAlign>());

    println!("a offset = {}", offset_of!(FooReprAlign, a));
    println!("b offset = {}", offset_of!(FooReprAlign, b));
    println!("c offset = {}", offset_of!(FooReprAlign, c));
}
