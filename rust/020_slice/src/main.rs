fn main() {
    let s = String::from("Hello world");
    let hello = &s[0..5];
    let hello2 = &s[..5];
    let world = &s[6..11];
    let world2 = &s[6..];

    println!("hello: {}, {}, world: {} {}", hello, hello2, world, world2);
}

