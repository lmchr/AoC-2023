pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

fn parse_input(inputs: &[String]) -> Vec<Vec<&str>>{
    inputs
        .iter()
        .map(|line| line
            .split("")
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
        )
        .collect::<Vec<Vec<&str>>>()
}

pub fn part1(inputs: &[String]) -> i32 {
    let parse = parse_input(&inputs);
    let s_find = parse
        .iter()
        .enumerate()
        .map(|(idx, line)| line
            .iter()
            .enumerate()
            .filter(|(idx2, char)| *char == &"S" )
            .map(|(idx2, char)| (idx, idx2) )
            .collect::<Vec<_>>()
        )
        .filter(|x| !x.is_empty())
        .next()
        .unwrap();
    let position_s = s_find.first().unwrap();
    println!("s={:?}", position_s);
    0
}

pub fn part2(inputs: &[String]) -> i32 {
    0
}
