pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

fn parse_input(inputs: &[String]) -> Vec<Vec<char>> {
    inputs
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}


pub fn part1(inputs: &[String]) -> usize {
    let mut map = parse_input(&inputs);
    // iterate per col
    for col_idx in 0..map.get(0).unwrap().len() {
        // start at row 1, can't move row 0 anyways
        for row_idx in 1..map.len() {
            let row = map.get_mut(row_idx).unwrap();
            let ele = row.get(col_idx).unwrap();
            if ele == &'O' {
                // move that shit
                let mut move_to_this = row_idx;
                for row_idx_pointer in (0..row_idx).rev() {
                    let row_move = map.get(row_idx_pointer).unwrap().get(col_idx).unwrap();
                    // stop here
                    if row_move == &'#' || row_move == &'O' {
                        break
                    } else {
                        move_to_this = row_idx_pointer;
                    }
                }
                // check if we actually moves to north
                if move_to_this != row_idx {
                    map[move_to_this][col_idx] = 'O';
                    map[row_idx][col_idx] = '.';
                }
            }
        }
        println!("col {col_idx} done");
        map.iter().for_each(|r| println!("{:?}", r));
    }
    map
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, line)| (idx + 1) * line.iter().filter(|ele| *ele == &'O').count())
        .sum()
}

pub fn part2(inputs: &[String]) -> u32 {
    0
}
