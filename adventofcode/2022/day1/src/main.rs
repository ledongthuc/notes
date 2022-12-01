use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input.txt").unwrap();
    let top = get_most_calories_from_elves::<1>(lines);
    println!("Most of calories: {:?}, Total: {}", top, top.iter().sum::<u32>());

    let lines = read_lines("./input.txt").unwrap();
    let top = get_most_calories_from_elves::<3>(lines);
    println!("Most 3 of calories: {:?}, Total: {}", top, top.iter().sum::<u32>());
}

fn get_most_calories_from_elves<const N: usize>(lines: io::Lines<io::BufReader<File>>) -> [u32; N] {
    let mut most_calories_an_elf_packed: [u32; N] = [0; N];
    let mut calories: u32 = 0;
    lines.for_each(|line| {
        if let Ok(line) = line {
            calories = match line.is_empty() {
                true => {
                    update_most_calories(&mut most_calories_an_elf_packed, calories);
                    0
                }
                _ => calories + line.parse::<u32>().unwrap(),
            }
        }
    });
    most_calories_an_elf_packed
}

fn update_most_calories<const N: usize>(top_list: &mut [u32; N], new_calories: u32) {
    if N == 0 {
        return;
    }
    if N == 1 {
        if top_list[0] < new_calories {
            top_list[0] = new_calories;
        }
        return;
    }

    let index = top_list.iter().position(|item| *item < new_calories);
    if let Some(index) = index {
        for shift_index in index..N - 1 {
            top_list[shift_index + 1] = top_list[shift_index]
        }
        top_list[index] = new_calories;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
