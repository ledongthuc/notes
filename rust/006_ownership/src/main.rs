fn main() {
    println!("Mutable string"); 
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("s: {}", s);

    println!("Shallow copy of complex data"); 
    let s2 = s;
    println!("s2: {}", s2);
    // failed following line because s is not owner of string litteral anymore
    // s2 borrowed s
    // println!("s: {}", s); 

    println!("Deep copy of complex data"); 
    let s3 = s2.clone();
    println!("s3: {}", s3); 
    println!("s2: {}", s2); 

    
    println!("Stack data only copy"); 
    let i1 = 10;
    let i2 = i1;
    println!("i1: {}, i2: {}", i1, i2); 

    println!("Ownership with function parameter");
    let fs = String::from("test");
    takes_ownership(fs);
    // following line will error because the owner ship of data in fS are moved
    // println!("fS (after function call): {}", fS);
    
    let fs2 = String::from("test2");
    let fs3 = takes_and_gives_back(fs2);
    println!("fs3 (in function call): {}", fs3);

    let fi = 10;
    makes_copy(fi);
    println!("fI (in function call): {}", fi);

    let returned_ownership = return_ownership();
    assert_eq!(String::from("Test"), returned_ownership);

    struct Struct1 {
        name: String,
        nick_name: String,
    }
    let struct1 = Struct1{
        name: String::from("Test"),
        nick_name: String::from("Test2"),
    };
    let get_string = struct1.name;
    let get_string2 = struct1.nick_name;
    // println!("{}", struct1.name); // failed because name is moved
    _ = get_string;
    _ = get_string2;
}


fn takes_ownership(some_string: String) {
    println!("fS (in function call): {}", some_string);
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("fs2 (in function call): {}", some_string);
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("fI (in function call): {}", some_integer);
}

fn return_ownership() -> String {
    String::from("Test")
}
