fn main() {
    println!("Hello, world!");
    another_functions();


    let block_code = {
        let output = optional_function(5);
        println!("optional_function() = {}" , output);
        output + 1
    };
    println!("block_code = {}" , block_code);
}

fn another_functions() {
    println!("Another functions!");
}

fn optional_function(input:i32) -> i32{
    input + 1
}
