fn main() {
    let s1 = String::from("hello");
    let s1_ref = &s1;
    let len = calculate_length(s1_ref);
    println!("s1 address: {}", s1_ref);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    calculate_mut_length(&mut s2);
    println!("s2: {}.", s2)
}

fn calculate_length(s: &String) -> usize {
    // Following line will get error because we can't change borrow value
    // some_string.push_str(", world");
    s.len()
}

fn calculate_mut_length(s: &mut String) {
     s.push_str(", world");
}
