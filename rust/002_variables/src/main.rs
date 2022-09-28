const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant's value is computed during compile
                                                 // time

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x can't change because x is immutable
    // x = 6;
    // println!("The value of x is: {}", x);

    let mut x = 5; // shadowing
    println!("The value of x-2 is: {}", x);
    x = 6;
    println!("The value of x-2 is: {}", x);

    {
        let x = 7; // new scope shadowing
        println!("The value of x-3 is: {}", x);
    }
    println!("The value of x-2 is: {}", x);

    let x = "Hello"; // Shadowing with different type
    println!("The value of x-4 is: {}", x);


    println!("The value of constant THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS)
}
