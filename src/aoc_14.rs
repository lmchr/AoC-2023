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
    let mut map = parse_input(inputs);
    let col_len = map.get(0).unwrap().len();
    // iterate per col
    move_north(&mut map, col_len);
    map.iter().for_each(|e| println!("{:?}", e));
    count_north_load(&map)
}

fn move_north(map: &mut [Vec<char>], col_len: usize) {
    for col_idx in 0..col_len {
        // start at row 1, can't move row 0 anyways
        for row_idx in 0..map.len() {
            let row = map.get(row_idx).unwrap();
            let ele = row.get(col_idx).unwrap();
            if ele == &'O' {
                // move that shit
                let mut move_to_this = row_idx;
                for row_idx_pointer in (0..row_idx).rev() {
                    let row_move = map[row_idx_pointer][col_idx];
                    // stop here
                    if row_move == '#' || row_move == 'O' {
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
    }
}

fn move_south(map: &mut Vec<Vec<char>>, col_len: usize) {
    for col_idx in 0..col_len {
        // start at row 1, can't move row 0 anyways
        for row_idx in (0..map.len() - 1).rev() {
            let row = map.get(row_idx).unwrap();
            let ele = row.get(col_idx).unwrap();
            if ele == &'O' {
                // move that shit
                let mut move_to_this = row_idx;
                for row_idx_pointer in row_idx + 1..map.len() {
                    let row_move = map[row_idx_pointer][col_idx];
                    // stop here
                    if row_move == '#' || row_move == 'O' {
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
    }
}

fn move_west(map: &mut Vec<Vec<char>>, col_len: usize) {
    for row_idx in 0..map.len() {
        // start at row 1, can't move row 0 anyways
        for col_idx in 1..col_len {
            let row = map.get(row_idx).unwrap();
            let ele = row.get(col_idx).unwrap();
            if ele == &'O' {
                // move that shit
                let mut move_to_this = col_idx;
                for col_idx_pointer in (0..col_idx).rev() {
                    let col_move = row.get(col_idx_pointer).unwrap();
                    // stop here
                    if col_move == &'#' || col_move == &'O' {
                        break
                    } else {
                        move_to_this = col_idx_pointer;
                    }
                }
                // check if we actually moves to north
                if move_to_this != col_idx {
                    map[row_idx][move_to_this] = 'O';
                    map[row_idx][col_idx] = '.';
                }
            }
        }
    }
}

fn move_east(map: &mut Vec<Vec<char>>, col_len: usize) {
    for row_idx in 0..map.len() {
        for col_idx in (0..col_len).rev() {
            let row = map.get(row_idx).unwrap();
            let ele = row.get(col_idx).unwrap();
            if ele == &'O' {
                // move that shit
                let mut move_to_this = col_idx;
                for col_idx_pointer in col_idx + 1..col_len {
                    let col_move = map.get(row_idx).unwrap().get(col_idx_pointer).unwrap();
                    // stop here
                    if col_move == &'#' || col_move == &'O' {
                        break
                    } else {
                        move_to_this = col_idx_pointer;
                    }
                }
                // check if we actually moves to north
                if move_to_this != col_idx {
                    map[row_idx][move_to_this] = 'O';
                    map[row_idx][col_idx] = '.';
                }
            }
        }
    }
}

fn count_north_load(map: &[Vec<char>]) -> usize {
    map
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, line)| (idx + 1) * line.iter().filter(|ele| *ele == &'O').count())
        .sum()
}

pub fn part2(inputs: &[String]) -> usize {
    let mut map = parse_input(inputs);
    let col_len = map.get(0).unwrap().len();
    // old state, new state
    for i in 0..1 {
        move_north(&mut map, col_len);
        move_west(&mut map, col_len);
        move_south(&mut map, col_len);
        move_east(&mut map, col_len);
        if i % 1_000_000 == 0 {
            println!("{}/{} ({:.1})%", i, 1_000_000_000, 100.0 * i as f32/1_000_000_000.0)
        }
    }
    map.iter().for_each(|e| println!("{:?}", e));
    count_north_load(&map)
}
