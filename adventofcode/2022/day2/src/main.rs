use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy)]
#[derive(PartialEq)]
enum GameOption {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = read_lines("./input.txt").unwrap();

    let mut total_score = 0;
    lines.for_each(|line| {
        if let Ok(line) = line {
            let parts = line.split(' ').collect::<Vec<&str>>();
            if parts.len() != 2 {
                return
            }
            let opponent_option = translate_code_option(parts.first().unwrap()).unwrap();
            let my_option = translate_code_option(parts.get(1).unwrap()).unwrap();

            total_score += calculate_score(&opponent_option, &my_option);
        }
    });
    println!("Total score part 1: {}", total_score);
}

fn part2() {
    let lines = read_lines("./input.txt").unwrap();

    let mut total_score = 0;
    lines.for_each(|line| {
        if let Ok(line) = line {
            let parts = line.split(' ').collect::<Vec<&str>>();
            if parts.len() != 2 {
                return
            }
            let opponent_option = translate_code_option(parts.first().unwrap()).unwrap();
            let result = translate_code_result(parts.get(1).unwrap());
            let my_option = get_my_option_from_result_and_opponent(&opponent_option, &result);

            total_score += my_option + result;
        }
    });
    println!("Total score part 2: {}", total_score);
}


fn translate_code_option(c: &str) -> Option<GameOption> {
    match c {
        "A" | "X" => Some(GameOption::Rock),
        "B" | "Y" => Some(GameOption::Paper),
        "C" | "Z" => Some(GameOption::Scissors),
        _ => None,
    }
}

fn translate_code_result(c: &str) -> u32 {
    match c {
         "X" => 0,
         "Y" => 3,
         "Z" => 6,
        _ => 0,
    }
}

fn play_score(opponent_option: &GameOption, my_option: &GameOption) -> u32 {
    if *opponent_option == *my_option {
        return 3;
    }

    match (opponent_option, my_option) {
        (&GameOption::Rock, &GameOption::Scissors)
        | (&GameOption::Scissors, &GameOption::Paper)
        | (&GameOption::Paper, &GameOption::Rock) => 0,

        (&GameOption::Rock, &GameOption::Paper)
        | (&GameOption::Scissors, &GameOption::Rock)
        | (&GameOption::Paper, &GameOption::Scissors) => 6,

        _ => 0,
    }
}

fn calculate_score(opponent_option: &GameOption, my_option: &GameOption) -> u32 {
    (*my_option as u32) + play_score(opponent_option, my_option)
}

fn get_my_option_from_result_and_opponent(opponent_option: &GameOption, result: &u32) -> u32 {
    if *result == 3_u32 {
        return opponent_option.to_owned() as u32
    }

    match (opponent_option, result) {
        (&GameOption::Rock, 0) => GameOption::Scissors as u32,
        (&GameOption::Rock, 6) => GameOption::Paper as u32,

        (&GameOption::Paper, 0) => GameOption::Rock as u32,
        (&GameOption::Paper, 6) => GameOption::Scissors as u32,


        (&GameOption::Scissors, 0) => GameOption::Paper as u32,
        (&GameOption::Scissors, 6) => GameOption::Rock as u32,

        _ => 0,
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
