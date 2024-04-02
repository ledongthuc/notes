fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // it's failed
    // x = 6;
    // println!("The value of x is: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    const MAX_POINTS: u32 = 10_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("The value of z is: {}", z);
    let z = "type's changed";
    println!("The value of z is: {}", z);

    // Following line got error because mutable variable can't shadowing
    // let mut u = 5;
    // let u = 3;
    // println!("The value of u is: {}", u);

    let tup = (500, 6.4, 1);
    let (_a, _b, _c) = tup;
    println!("The value of c is: {}", _c);
    println!("The value of type.1 is: {}", tup.1);

    another_function(13);

    if tup.0 < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {}", result);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn another_function(i: i32) {
    let x = {
        let z = 10;
        i + z
    };
    println!("Another function. {}", sum(x, -5));
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}
