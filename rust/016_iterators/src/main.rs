fn main() {
    let arr = vec![1,2,3,4,5];

    println!("Vec.sum: {}", arr.iter().sum::<i32>());
    println!("Vec.map: {:?}", arr.iter().map(|x| x + 1));
}
