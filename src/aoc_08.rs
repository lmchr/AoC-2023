use std::collections::HashMap;

pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

pub fn part1(inputs: &[String]) -> i64 {
    let (instructions, navigation_map) = prepare_input(inputs);
    let mut current_pos = "AAA";
    let mut counter = 0;
    for instruction in instructions.iter().cycle() {
        let left_or_right = if instruction == &'L' { 0 } else { 1 };
        println!("{:?} {}", navigation_map, current_pos);
        current_pos = (*navigation_map.get(current_pos).unwrap())
            .get(left_or_right)
            .unwrap();
        counter += 1;
        if current_pos == "ZZZ" {
            break;
        }
    }
    counter
}

pub fn lcm(nums: &[u128]) -> u128 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn part2(inputs: &[String]) -> u128 {
    let (instructions, navigation_map) = prepare_input(inputs);
    // get nodes starting ending with "A"
    let mut current_nodes = navigation_map
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();
    let mut cycle_lengths: Vec<u128> = vec![];
     for node in current_nodes.iter_mut() {
        let mut curr_node = *node;
        for (instruction_idx, instruction) in instructions.iter().cycle().enumerate() {
            let left_or_right = if instruction == &'L' { 0 } else { 1 };
            // update all positions at once
            let new = (*navigation_map.get(*curr_node).unwrap())
                .get(left_or_right)
                .unwrap();
            curr_node = new;
            if curr_node.ends_with('Z') {
                cycle_lengths.push((instruction_idx + 1) as u128);
                break
            }
        }
    }
    println!("{:?}",cycle_lengths);
    lcm(&cycle_lengths)
}

fn prepare_input(inputs: &[String]) -> (Vec<char>, HashMap<&str, Vec<&str>>) {
    let instructions: Vec<_> = inputs.get(0).unwrap().chars().collect();

    let navigation_map: HashMap<&str, Vec<&str>> = HashMap::from_iter(
        inputs
            .iter()
            .skip(2)
            .map(|line| line.split(" = ").collect::<Vec<_>>())
            .map(|split| (*split.first().unwrap(), *split.last().unwrap()))
            .map(|(first, second)| (first, vec![&second[1..4], &second[6..9]]))
            .collect::<Vec<_>>()
    );
    (instructions, navigation_map)
}
