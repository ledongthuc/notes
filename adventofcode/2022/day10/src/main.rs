use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    let insts = load_instruction();

    let mut value: i32 = 1;
    let mut signal_strength: i32 = 0;
    let mut current_cycle = 0;
    insts.iter().for_each(|inst| {
        for _ in 0..inst.cycle {
            current_cycle += 1;

            signal_strength += match current_cycle {
                x if [20, 60, 100, 140, 180, 220].contains(&x) => value * x,
                _ => 0,
            };
        }
        value += inst.value;
    });

    println!("Part1: total value: {}", signal_strength)
}

fn part2() {
    let insts = load_instruction();

    let mut spite_start_index = 0;
    let mut current_cycle = 0;
    let mut writing_index = 0;

    insts.iter().for_each(|inst| {
        for _ in 0..inst.cycle {
            current_cycle += 1;

            // println!("writing_index: {}, start: {}, end: {}, cycle: {}", writing_index, spite_start_index, spite_start_index + 2, current_cycle);
            if writing_index >= spite_start_index && writing_index <= spite_start_index + 2 {
                print!("█");
            } else {
                print!(" ")
            }

            writing_index = match current_cycle {
                x if [40, 80, 120, 160, 200, 240].contains(&x) => {
                    println!();
                    0
                }
                _ => writing_index + 1,
            };
        }
        spite_start_index += inst.value
    });
}

struct Instruction {
    value: i32,
    cycle: u8,
}

fn instruction_from_string(line: String) -> Option<Instruction> {
    match line.split_once(' ') {
        Some(("noop", _)) => Some(Instruction { cycle: 1, value: 0 }),
        Some(("addx", value)) => Some(Instruction {
            cycle: 2,
            value: value.parse::<i32>().unwrap(),
        }),
        _ => Some(Instruction { cycle: 1, value: 0 }),
    }
}

fn load_instruction() -> Vec<Instruction> {
    let lines = read_lines("./input.txt").unwrap();

    let mut vec = Vec::new();
    lines.for_each(|line| {
        vec.push(instruction_from_string(line.unwrap()).unwrap());
    });
    vec
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
