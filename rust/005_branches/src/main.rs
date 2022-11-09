mod let_else;

fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition2 = if number % 2 == 0 { "true" } else { "false" };
    println!("condition2: {}", condition2);

    let mut count = 0;

    'outer: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("Count: {}", count);

    let mut count2 = 0;

    let result = loop {
        count2 += 1;
        if count2 == 10 {
            break count2 * 2;
        }
    };
    println!("The result is {}", result);

    let mut count3 = 3;
    while count3 != 0 {
        println!("Count3: {}", count3);

        count3 -= 1;
    }
    println!("The result is {}", count3);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("[2] The value is {}", element);
    }

    for element in (1..4).rev() {
        println!("{}!", element)
    }
}
