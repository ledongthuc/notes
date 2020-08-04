fn main() {
    // Stack copy
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Heap copy
    let s1 = String::from("hello");
    // Get error because only one ownership
    let s2 = s1;

    // crash because s1 give ownership to s2
    // println!("s1: {}, world!", s1);
    println!("s2: {}, world!", s2);
     let s3 = s2.clone();
    println!("s2: {}, world!", s2);
    println!("s3: {}, world!", s3);

    let s = String::from("hello");
    takes_ownership(s);
    // build failed because s's changed ownership
    //  println!("{}", s); 

    let x = 5;
    makes_copy(x);
    // success because i32 is scala type and it's make copy, not change owner
    println!("x: {}", x); 

    let x2 = give_ownership();
    println!("x2: {}", x); 

    let x3 = take_and_give_ownership(x2);
    println!("x3: {}", x3); 
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn give_ownership() -> String {
    let example_string = String::from("hello");
    example_string
}

fn take_and_give_ownership(x: String) -> String {
    x
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
