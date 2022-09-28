#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

impl IpAddrKind {
    fn print(&self) {
        println!("{:?}", self);
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    four.print();
    let six = IpAddrKind::V6(String::from("::1"));
    six.print();


    let some_number = Some(5);
    println!("some_number: {:?}", some_number);
    let add_number = plus_one(some_number);
    println!("some_number + 1: {:?}", add_number);
    let compare_number = Some(6);
    if let add_number = compare_number {
        println!("Yes, add number is 6")
    }


    let some_string = Some("a string");
    println!("some_string: {:?}", some_string);

    let absent_number: Option<i32> = None;
    println!("absent_number: {:?}", absent_number);

    println!("value in cents {}", value_in_cents(Coin::Dime))
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
