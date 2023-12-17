pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical
}

fn parse_input(inputs: &[String]) -> Vec<Vec<Vec<char>>> {
    let mut out = vec![];
    let mut tmp = vec![];
    for input in inputs {
        if input.is_empty(){
            out.push(tmp.clone());
            tmp.clear();
        } else {
            tmp.push(input.chars().collect::<Vec<_>>());
        }
    }
    out.push(tmp);
    out
}


fn get_col(pattern: &Vec<Vec<char>>, col_idx: i32) -> Option<Vec<char>> {
    if col_idx < 0 || col_idx > (pattern.get(0).unwrap().len() as i32) - 1  {
        None
    } else {
        Some(pattern
            .iter()
            .map(|row| *row
                .get(col_idx as usize)
                .unwrap()
            )
            .collect::<_>())
    }
}

fn get_row(pattern: &Vec<Vec<char>>, row_idx: i32) -> Option<Vec<char>> {
    if row_idx < 0 || row_idx > (pattern.len() as i32) - 1 {
        None
    } else {
        Some(pattern.get(row_idx as usize).unwrap().clone())
    }
}

fn calculate_score(orientation: Orientation, row_or_col_idx: i32) -> u32 {
    if orientation == Orientation::Horizontal {
        (100 * (row_or_col_idx + 1)) as u32
    } else {
        (row_or_col_idx + 1) as u32
    }
}

fn find_reflection(pattern: &Vec<Vec<char>>, col_idx_to_skip: i32, row_idx_to_skip: i32) -> Option<(Orientation, i32)> {
    let col_result = scan_row_or_col(pattern, col_idx_to_skip, Orientation::Vertical);
    if let Some(row_or_col) = col_result {
        return Some((Orientation::Vertical, row_or_col));
    }
    let row_result = scan_row_or_col(pattern, row_idx_to_skip, Orientation::Horizontal);
    if let Some(row_or_col) = row_result {
        return Some((Orientation::Horizontal, row_or_col));
    }
    None
}

fn scan_row_or_col(pattern: &Vec<Vec<char>>, row_or_col_idx_to_skip: i32, orientation: Orientation) -> Option<i32>{
    let len = if orientation == Orientation::Horizontal {pattern.len() as i32} else {pattern.get(0).unwrap().len() as i32};
    for idx in 0..len - 1 {
        if idx == row_or_col_idx_to_skip {
            continue
        }
        let mut equal = false;
        let mut outward_idx = 0;
        loop {
            let get_fn = if orientation == Orientation::Vertical {get_col} else {get_row};
            let row_or_col_1 = get_fn(pattern, idx - outward_idx);
            let row_or_col_2 = get_fn(pattern, idx + 1 + outward_idx);
            if let (Some(r1), Some(r2)) = (row_or_col_1, row_or_col_2) {
                if r1 != r2 {
                    equal = false;
                    break
                }
                equal = true;
            } else {
                // end reached for one or both columns
                break
            }
            outward_idx += 1;
        }
        if equal {
            return Some(idx);
        }
    }
    None
}

pub fn part1(inputs: &[String]) -> u32 {
    let patterns = parse_input(inputs);
    let mut result: u32 = 0;
    for pattern in patterns {
        let (orientation, col_or_row_idx) = find_reflection(&pattern, -1, -1).unwrap();
        result += calculate_score(orientation, col_or_row_idx);
    }
    result
}

pub fn part2(inputs: &[String]) -> u32 {
    let parsed = parse_input(inputs);
    println!("{parsed:?}");
    let mut result: u32 = 0;
    for parse in parsed {
        let mut success = false;
        let (orientation_og, row_or_col_og) = find_reflection(&parse, -1, -1).unwrap();
        // brute-force every pixel
        'outer: for x in 0..parse.len() {
            for y in 0..parse.get(0).unwrap().len() {
                let mut parse_smudge_fix: Vec<Vec<char>> = parse.clone();
                let char = parse_smudge_fix.get(x).unwrap().get(y).unwrap();
                parse_smudge_fix[x][y] = if char == &'.' {'#'} else {'.'};

                let col_idx = if orientation_og == Orientation::Vertical {row_or_col_og} else {-1};
                let row_idx = if orientation_og == Orientation::Horizontal {row_or_col_og} else {-1};
                let possible_result = find_reflection(&parse_smudge_fix, col_idx, row_idx);
                if let Some((orientation, row_or_col)) = possible_result {
                    // dont consider the original result without the smudge fix
                    if orientation != orientation_og || row_or_col != row_or_col_og {
                        result += calculate_score(orientation, row_or_col);
                        success = true;
                        break 'outer;
                    }
                }
            }
        }
        if !success {
            panic![];
        }
    }
    result
}
