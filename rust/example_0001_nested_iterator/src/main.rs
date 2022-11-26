fn main() {
    let items = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    items.iter().enumerate().for_each(|(i, x)| {
        print!("------\nx: {}\n", x);

        if i > 0 {
            let before = &items[..i];
            println!(
                "Even number before x: {}",
                before.iter().filter(|y| *y % 2 == 0).count()
            );
        }

        if i != items.len() - 1 {
            let after = &items[i + 1..];
            println!(
                "Even number After x: {}",
                after.iter().filter(|y| *y % 2 == 0).count()
            );
        }
    })
}
