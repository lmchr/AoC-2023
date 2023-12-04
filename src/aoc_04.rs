use std::collections::HashMap;

pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}
fn str_to_digits(s: &str) -> Vec<u32> {
    s.split(" ")
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
        .strip_prefix(" ")
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

fn get_overlapping_numbers(winning_numbers: &Vec<u32>, our_numbers: &Vec<u32>) -> usize {
    our_numbers
        .iter()
        .filter(|i| winning_numbers.contains(i))
        .count()
}

pub fn part2(inputs: &[String]) -> u32 {
    // card_id : amount
    let mut card_amounts: HashMap<usize, u32> = HashMap::new();

    for (idx, input) in inputs.iter().enumerate() {
        // add 1 to count for the initial card. there might be values added already
        let current_card_amount = card_amounts.entry(idx + 1).or_insert(0);
        *current_card_amount += 1;

        // dereference, otherwise the borrowchecker will complain in the loop
        // that a mutable borrow is used again
        let current_card_amount_ref = *current_card_amount;
        let (winning_numbers, our_numbers) = get_winning_and_our_numbers(input);
        let overlapping_numbers = get_overlapping_numbers(&winning_numbers, &our_numbers);
        for card_id in idx + 2 .. idx + 2 + overlapping_numbers {
            *card_amounts.entry(card_id).or_default() += current_card_amount_ref
        }
    }
    card_amounts
        .values()
        .sum()
}