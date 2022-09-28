use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    println!("scores: {:?}", scores);

    let key1 = "Blue";
    let mut value1 = "10";
    scores.insert(key1, value1);
    println!("scores: {:?}", scores);
    println!("key1: {}", key1);
    println!("value1: {}", value1);

    value1 = "20";
    println!("scores: {:?}", scores);
    println!("key1: {}", key1);
    println!("value1: {}", value1);


    let mut scores2 = HashMap::new();
    println!("scores: {:?}", scores);
    let key2 = "blue";
    let value2 = String::from("10");
    scores2.insert(key2, value2);
    println!("scores: {:?}", scores2);
    println!("key1: {}", key2);
    // println!("value1: {}", value2); // Values a are moved

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores3: HashMap<String, i32> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("initial scores: {:?}", scores3);
    println!("scores3.get(\"Blue\"): {:?}", scores3.get("Blue"));

    for (key, value) in &scores3 {
        println!("{}: {}", key, value);
    }

    scores3.entry(String::from("Blue")).or_insert(90);
    scores3.entry(String::from("Green")).or_insert(90);
    for (key, value) in &scores3 {
        println!("{}: {}", key, value);
    }

    let updated_teams = vec![String::from("Blue"), String::from("Black")];
    for word in updated_teams {
        let count = scores3.entry(word).or_insert(0);
        *count += 1;
    }
    for (key, value) in &scores3 {
        println!("{}: {}", key, value);
    }
}
