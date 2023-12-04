pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

fn str_to_digits(s: &str) -> Vec<u32> {
    s.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn get_winning_and_our_numbers(input: &str) -> (Vec<u32>, Vec<u32>) {
    let x: Vec<&str> = input
        .strip_prefix("Card ")
        .unwrap()
        .split(':')
        .collect();
    let y: Vec<&str> = x
        .last()
        .unwrap()
        .strip_prefix(' ')
        .unwrap()
        .split(" | ")
        .collect();

    let winning_numbers = str_to_digits(y.first().unwrap());
    let our_numbers = str_to_digits(y.last().unwrap());
    (winning_numbers, our_numbers)
}

pub fn part1(inputs: &[String]) -> u32 {
    let mut sum: u32 = 0;
    for input in inputs.iter() {
        let (winning_numbers, our_numbers) = get_winning_and_our_numbers(input);

        let overlapping_numbers = get_overlapping_numbers(&winning_numbers, &our_numbers);
        if overlapping_numbers > 0 {
            sum += 2_u32.pow(overlapping_numbers as u32 - 1);
        }
    }
    sum
}

fn get_overlapping_numbers(winning_numbers: &[u32], our_numbers: &[u32]) -> usize {
    our_numbers
        .iter()
        .filter(|i| winning_numbers.contains(i))
        .count()
}

pub fn part2(inputs: &[String]) -> u32 {
    let mut card_amounts: Vec<u32> = vec![1; inputs.len()];

    for (idx, input) in inputs.iter().enumerate() {
        let (winning_numbers, our_numbers) = get_winning_and_our_numbers(input);
        let overlapping_numbers = get_overlapping_numbers(&winning_numbers, &our_numbers);
        let x = *card_amounts.get(idx).unwrap();
        for card_id in idx + 1 .. idx + 1 + overlapping_numbers {
            *card_amounts.get_mut(card_id).unwrap() += x;
        }
    }
    card_amounts
        .iter()
        .sum()
}