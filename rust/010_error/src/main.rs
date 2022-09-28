use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // specific panic macros
    // panic!("crash and burn")

    let v = vec![1,2,3];
    // v[99];

    let f = File::open("hello.txt");
    println!("f: {:?}", f);
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    println!("f: {:?}", f);


    let f2 = File::open("hello2.txt");
    println!("f2: {:?}", f2);
    let f2 = match f2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello2-wrong.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    println!("f2: {:?}", f2);

    let f3 = File::open("hello3.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello3-wrong.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("f3: {:?}", f3);

    // let f4 = File::open("hello4.txt").unwrap();

    // let f5 = File::open("hello5.txt").expect("Failed to open hello.txt");
    // println!("f5: {:?}", f5);

    let f6_content = read_username_from_file("hello.txt");
    println!("f6_content: {:?}", f6_content);

    let f7_content = read_username_from_file2("hello.txt");
    println!("f7_content: {:?}", f7_content);

    let f8 = File::open("hello8.txt")?;
    Ok(())
}

fn read_username_from_file(file_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file2(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}
