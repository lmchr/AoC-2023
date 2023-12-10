use std::env;
use std::process::exit;
use std::time::Instant;
use aoc_2023::{aoc_01, aoc_02, aoc_03, aoc_04, aoc_05, aoc_06, aoc_07, aoc_08, aoc_09};
use aoc_2023::util::{Days, read_input};
use strum::IntoEnumIterator;

fn execute_day(day: &Days) {
    let input = read_input(day);
    println!("Executing day {} on {} elements", day.value(), input.len());
    let start = Instant::now();
    match day {
        Days::ONE => aoc_01::main(&input),
        Days::TWO => aoc_02::main(&input),
        Days::THREE => aoc_03::main(&input),
        Days::FOUR => aoc_04::main(&input),
        Days::FIVE => aoc_05::main(&input),
        Days::SIX => aoc_06::main(&input),
        Days::SEVEN => aoc_07::main(&input),
        Days::EIGHT => aoc_08::main(&input),
        Days::NINE => aoc_09::main(&input),
    }
    let duration = start.elapsed();
    println!("Day {} finished in {:?}", day.value(), duration);
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
    if day == 0 {
        println!("Executing all days.");
        let start = Instant::now();
        for day in Days::iter() {
            execute_day(&day)
        }
        let duration = start.elapsed();
        println!("All days finished in {:?}", duration);
    } else {
        let day_enum = Days::by_value(&day);
        execute_day(&day_enum)
    }

}
