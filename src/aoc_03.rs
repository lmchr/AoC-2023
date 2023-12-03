use std::collections::HashMap;

pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

pub fn is_symbol_except_dot(s: &char) -> bool {
    s != &'.' && !s.is_alphanumeric()
}


pub fn is_star(s: &char) -> bool {
    s == &'*'
}

fn has_adjacent_symbols(inputs: &[String],
                        start_row_idx: usize, end_col_idx: usize,
                        len_digit: usize,
                        check_fn: &dyn Fn(&char) -> bool) -> Option<Vec<(usize, usize)>> {
    // performance-wise, this function does too much for part 1. we could simple return true/false
    // once the first 'is_symbol_except_dot' call is true. to reduce duplication, this function is
    // also reusable for part 2

    // check row of digit everytime
    let mut rows = vec![start_row_idx];
    // row above
    if start_row_idx > 0 {
        rows.push(start_row_idx - 1)
    }
    // row below
    if start_row_idx + 1 < inputs.len() {
        rows.push(start_row_idx + 1)
    }
    let mut result: Vec<(usize, usize)> = vec![];
    for row in rows {
        // special logic, only check start and end idx next to digit
        let cols_range: Vec<i32> = if row == start_row_idx {
            let mut cols: Vec<i32> = vec![end_col_idx as i32];
            if end_col_idx > (len_digit + 1){
                cols.push(end_col_idx as i32 - (len_digit as i32 + 1));
            }
            cols
        } else {
            ((end_col_idx as i32 - (len_digit as i32 + 1))..end_col_idx as i32 + 1).collect()
        };
        for col in cols_range {
            if let Some(char) = inputs.get(row).unwrap().as_bytes().get(col as usize) {
                if check_fn(&(*char as char)) {
                    // for part1, we could just return "true" here
                    result.push((row, col as usize));
                }
            }
        }
    }
    if !result.is_empty() {
        return Some(result)
    }
    None
}

pub fn part1(inputs: &[String]) -> u32 {
    let mut sum = 0;
    for (row_idx, input) in inputs.iter().enumerate() {
        let mut collector: Vec<u32> = vec![];
        // iterate one more index to also handle digits that are at the end of the line.
        // here the .nth() lookup will not work and trigger the neighbour check
        for col_idx in 0..input.len() + 1 {
            let char = input.as_bytes().get(col_idx);
            if char.is_some() && (*char.unwrap() as char).is_ascii_digit() {
                collector.push((*char.unwrap() as char).to_digit(10).unwrap());
            } else if !collector.is_empty() {  // check if we collected a number and a non-numeric character followed
                if has_adjacent_symbols(inputs, row_idx, col_idx, collector.len(), &is_symbol_except_dot).is_some() {
                    // vec of single digit numbers to "whole" number, use 10^idx
                    for (idx, digit) in collector.iter().enumerate(){
                        sum += digit * 10_u32.pow((collector.len() - idx - 1) as u32);
                    }
                }
                collector.clear();
            }

        }
    }
    sum
}

pub fn part2(inputs: &[String]) -> u32 {
    // (x, y): <full-value>
    // e.g. (30,50): 345
    // e.g. (30,51): 345
    // e.g. (30,52): 345
    let mut coordinates: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (row_idx, input) in inputs.iter().enumerate() {
        let mut collector: Vec<u32> = vec![];
        // iterate one more index to also handle digits that are at the end of the line.
        // here the .nth() lookup will not work and trigger the neighbour check
        for col_idx in 0..input.len() + 1 {
            let char = input.as_bytes().get(col_idx);
            if char.is_some() && (*char.unwrap() as char).is_ascii_digit() {
                collector.push((*char.unwrap() as char).to_digit(10).unwrap());
            } else if !collector.is_empty() {  // check if we collected a number and a non-numeric character followed
                if let Some(indices) = has_adjacent_symbols(inputs,
                                                                      row_idx,
                                                                      col_idx,
                                                                      collector.len(),
                                                                      &is_star) {
                    // vec of single digit numbers to "whole" number, use 10^idx
                    let mut whole_number = 0;
                    for (idx, digit) in collector.iter().enumerate(){
                        whole_number += digit * 10_u32.pow((collector.len() - idx - 1) as u32);
                    }
                    for (tmp_row_idx, tmp_col_idx) in &indices {
                        coordinates.entry((*tmp_row_idx, *tmp_col_idx)).or_default().push(whole_number);
                    }
                }
                collector.clear();
            }

        }
    }
    coordinates
        .iter()
        // only count those entries that have 2 digits (meaning this * has >=2 digits as neighbours)
        .filter(|(_, digits)| digits.len() > 1)
        .map(|(&coordinate, digits)| (coordinate, digits.iter().product()))
        .collect::<HashMap<(usize, usize), u32>>()
        .values()
        .sum()
}