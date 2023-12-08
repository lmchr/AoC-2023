use std::collections::HashMap;

pub fn main(inputs: &[String]) {
    //println!("Part 1: {}", part1(inputs));
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

pub fn part2(inputs: &[String]) -> i64 {
    let (instructions, navigation_map) = prepare_input(inputs);
    // get nodes starting ending with "A"
    let mut current_nodes = navigation_map
        .keys()
        .filter(|node| node.ends_with('A'))
        .collect::<Vec<_>>();
    println!("{:?}", &current_nodes);
    let mut counter = 0;

    for instruction in instructions.iter().cycle() {
        counter += 1;
        if counter % 100_000 == 0 {
            println!("{counter}");
        }
        let left_or_right = if instruction == &'L' { 0 } else { 1 };
        // update all positions at once
        for node in current_nodes.iter_mut() {
            let new = (*navigation_map.get(*node).unwrap())
                .get(left_or_right)
                .unwrap();
            *node = new;
        }
        // all end with Z?
        let done = current_nodes
            .iter()
            .all(|node| node.ends_with('Z'));
        if done {
            break
        }
    }
    counter
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
