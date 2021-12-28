fn main() {
    let plus_2_numbers = |num1: i32, num2: i32| -> i32 {
        num1 + num2
    };

    println!("1 + 2 = {}", plus_2_numbers(1, 2));
}

fn plus_2_numbers(num1: i32, num2: i32) -> i32 {
    num1 - num2
}
