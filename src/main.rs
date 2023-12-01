use std::{env, fs};
use std::path::Path;
use std::process::exit;
use std::time::Instant;

mod aoc_01;

fn read_input(day: &u8) -> Vec<String> {
    let path = Path::new("data").join(format!("aoc_{:0>2}.txt", day.to_string()));
    let lines: Vec<String> = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Read file from {:?} failed due to {}", &path, err))
        .lines()
        .map(|t| t.to_string())
        .collect();
    lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Only provide the day to run");
        exit(1);
    }
    let day = args
        .get(1)
        .expect("Provide the day to run ([1..24])")
        .parse::<u8>()
        .expect("Pass the day as a positive integer");
    let start;
    match day {
        invalid_day if invalid_day > 24 => {
            panic!("There are only 24 days in an advent calendar")
        },
        valid_day if valid_day > 0 => {
            let input = read_input(&day);
            start = Instant::now();
            match valid_day {
                1 => aoc_01::main(input),
                _ => panic!("Day {} not implemented yet", day)
            }
        },
        _ => panic!("Invalid date"),
    }
    let duration = start.elapsed();
    println!("Day {} finished in {:?}", day, duration);
}
