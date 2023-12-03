pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

pub fn is_symbol_except_dot(s: &char) -> bool {
    s != &'.' && !s.is_alphanumeric()
}

pub fn part1(inputs: &[String]) -> u32 {

    fn has_adjacent_symbols(inputs: &[String],
                            start_row_idx: usize, end_col_idx: usize,
                            len_digit: usize) -> bool {
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
        for row in rows {
            let cols_range: Vec<i32> = if row == start_row_idx {  // special logic, only check start and end idx next to digit
                let mut cols: Vec<i32> = vec![end_col_idx as i32];
                if end_col_idx > (len_digit + 1){
                    cols.push(end_col_idx as i32 - (len_digit as i32 + 1));
                }
                cols
            } else {
                ((end_col_idx as i32 - (len_digit as i32 + 1))..end_col_idx as i32 + 1).collect()
            };
            for col in cols_range {
                if let Some(char) = inputs.get(row).unwrap().chars().nth(col as usize) {
                    if is_symbol_except_dot(&char) {
                        return true
                    }
                }
            }
        }
        false
    }

    let mut sum = 0;
    for (row_idx, input) in inputs.iter().enumerate() {
        let mut collector: Vec<u32> = vec![];
        // iterate one more index to also handle digits that are at the end of the line.
        // here the .nth() lookup will not work and trigger the neighbour check
        for col_idx in 0..input.len() + 1 {
            let x = input.chars().nth(col_idx);
            if x.is_some() && x.unwrap().is_ascii_digit() {
                collector.push(x.unwrap().to_digit(10).unwrap());
            } else if !collector.is_empty() {  // check if we collected a number and a non-numeric character followed
                if has_adjacent_symbols(inputs, row_idx, col_idx, collector.len()) {
                    // vec of single digit numbers to "whole" number, use 10^idx
                    for (idx, digit) in collector.iter().enumerate(){
                        sum += digit * 10_u32.pow((collector.len() - idx - 1) as u32);
                    }
                }
                collector.clear();
            }

        }
        println!("\n")
    }
    sum
}

pub fn part2(inputs: &[String]) -> i32 {
    0
}