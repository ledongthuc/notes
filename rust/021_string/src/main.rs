fn main() {
    let mut s = String::new();
    println!("s: {}", s);

    let data = "Init contents";
    println!("data: {}", data);

    s = data.to_string();
    println!("s: {}", s);

    s.push_str(" bar");
    println!("s: {}", s);

    let s2 = s + "-" + data; // s will be take ownership
    println!("s2: {}", s2);

    let s3 = format!("{}-{}", s2, data);
    println!("s3: {}, s2: {}", s3, s2);

    // Can't access byte of string
    // println!("s3[3]: {}", s3[3]);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
