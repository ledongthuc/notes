fn main() {
    println!("Hello, world!");
    let v: Vec<i32> = Vec::new();
    println!("v: {:?}", v);

    let v2 = vec![1, 2, 3];
    println!("v2: {:?}", v2);

    let mut v3 = vec![4, 5, 6];
    println!("v3: {:?}", v3);
    v3.push(7);
    v3.push(8);
    v3.push(9);
    println!("v3: {:?}", v3);

    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    match v3.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v3 {
        println!("iterating {}", i);
    }
}
