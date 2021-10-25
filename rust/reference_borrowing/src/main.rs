fn main() {
    let s1 = String::from("Test");
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    // Following codes won't work because s1 is a immutable reference 
    // change_str(&s1);
    // println!("S1: {}", s1);

    let mut s2 = String::from("Hello");
    change_str_mut(&mut s2);
    println!("S2 (after change_str): {}", s2);

    let mut s3 = String::from("Test");
    let s3_mut1 = &mut s3;
    // Second mut is not allowed
    // let s3_mut2 = &mut s3;
    // println!("s3_mut1: {}, s3_mut2: {}", s3_mut1, s3_mut2);

    let mut s4 = String::from("Test mut 4");
    let s4_ref_1 = &s4;
    let s4_ref_2 = &s4;
    // Following line failed because we can mixing mut and un-mut reference before using. it can
    // lead to who change data in while
    // let s4_mut_ref_3 = &mut s4;
    println!("s4_mut1: {}, s4_mut2: {}", s4_ref_1, s4_ref_2);

    let mut s5 = String::from("Test mut 5");
    let s5_ref_1 = &s5;
    let s5_ref_2 = &s5;
    println!("s4_mut1: {}, s4_mut2: {}", s5_ref_1, s5_ref_2);
    let s5_mut_ref_3 = &mut s5;
    println!("s4_mut_ref_3: {}", s5_mut_ref_3);

    // Follow function will be failed because return an dangling borrowed reference
    // let s6 = String::from("dangle");

    let s7 = no_dangle();
    println!("s7: {}", s7);
}

fn calculate_length(s: &String) -> usize {
    // In this scope, s is pointer that contains address to String s1.
    s.len()
}

// Following codes won't work because we are trying to change a unmutable reference
// fn change_str(s: &String) {
//     s.push_str("ing");
// }

fn change_str_mut(s: &mut String) {
    s.push_str(", world!");
}

// Follow function will be failed because return an dangling borrowed reference
// Out of the function scope, s is clear but pointer s (&s) still point to memory
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
