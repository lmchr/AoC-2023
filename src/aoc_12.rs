use itertools::Itertools;

pub fn main(inputs: &[String]) {
    //println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

fn parse_input(inputs: &[String]) -> Vec<(&str, Vec<u32>)> {
    inputs
        .iter()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|line| (
            *line
                .first()
                .unwrap(),
            line
                .last()
                .unwrap()
                .split(',')
                .collect::<Vec<_>>()
                .iter()
                .map(|digits| digits.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        ))
        .collect()
}

fn check_spring_numbers(spring_status: &Vec<char>) -> Vec<u32> {
    let mut out = Vec::with_capacity(spring_status.len());
    let mut c = 0;
    for x in spring_status {
        if x == &'.' {
            if c > 0 {
                out.push(c);
                c = 0;
            }
        } else {
            c += 1;
        }
    }
    if c > 0 {
        out.push(c);
    }
    out
}

fn find_corrupt_positions(spring_status: &str) -> Vec<usize> {
    spring_status
        .chars()
        .enumerate()
        .filter(|(_idx, x)| x == &'?')
        .map(|(idx, _x)| idx)
        .collect()
}

fn find_possible_combinations(spring_status: &str, numbers: &Vec<u32>) -> u32 {
    let mut num_combinations = 0;
    let corrupt_positions = find_corrupt_positions(spring_status);
    let mut spring_status_copy = spring_status.chars().collect_vec();
    let mut spring_status_copy2 = spring_status.chars().collect_vec();
    // something like "permutations_with_replacement" doesnt exist, so we use this.
    // source: https://stackoverflow.com/a/71425131
    let numbers_sum: usize = numbers.iter().sum::<u32>() as usize;
    for combination in (1..=corrupt_positions.len()).map(|_| vec!['#', '.']).multi_cartesian_product() {
        let num_hashtags = spring_status_copy2.iter().filter(|ele| *ele == &'#').count() +
                                   combination.iter().filter(|ele| *ele == &'#').count();
        let num_dots= spring_status_copy.len() - num_hashtags;
        if num_hashtags != numbers_sum || num_dots < (numbers.len() - 1) {
            //println!("no fit: {num_hashtags}!={numbers_sum}, {spring_status_copy:?}");
            continue
        }

        for (idx, status) in corrupt_positions.iter().zip(combination) {
            spring_status_copy[*idx] = status;
        }

        let spring_numbers = check_spring_numbers(&spring_status_copy);
        //println!("{:?} --> \n{:?}={:?}", spring_status_copy, spring_numbers, numbers);
        if &spring_numbers == numbers {
            //println!("MATCH");
            num_combinations += 1;
        }
    }
    println!("{}", num_combinations);
    num_combinations
}

pub fn part1(inputs: &[String]) -> u32 {
    let mut solutions = 0;
    let parsed = parse_input(inputs);
    for (spring_status, numbers) in parsed {
        solutions += find_possible_combinations(spring_status, &numbers);
    }
    solutions
}

fn unfold(parsed: &Vec<(&str, Vec<u32>)>) -> Vec<(String, Vec<u32>)> {
    let mut out = vec![];
    for (spring_status, numbers) in parsed {
        let new_numbers = numbers.repeat(5);
        let mut spring_status_new = spring_status.to_string();
        for _ in 0..4 {
            spring_status_new.push('?');
            spring_status_new.push_str(spring_status);
        }
        out.push((spring_status_new, new_numbers));
    }
    out
}

pub fn part2(inputs: &[String]) -> u32 {
    let mut solutions = 0;
    let parsed = parse_input(inputs);
    let unfolded = unfold(&parsed);
    for (idx, (spring_status, numbers)) in unfolded.iter().enumerate() {
        solutions += find_possible_combinations(spring_status, numbers);
        println!("{}/{}", idx, unfolded.len());
    }
    solutions
}
