pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

fn prepare_input(inputs: &[String]) -> Vec<Vec<i32>>{
    inputs
        .iter()
        .map(|digit_s| digit_s.split(' '))
        .map(|digits_s_list| digits_s_list
            .collect::<Vec<_>>()
            .iter()
            .map(|i| i.parse::<i32>().unwrap())
            .collect()
        )
        .collect::<Vec<_>>()
}

#[allow(clippy::get_first)]
fn get_recursive_diff_last_values(values: Vec<i32>) -> Vec<Vec<i32>> {
    let mut diff_collect = vec![];
    let mut current_diff = values;
    loop {
        let diffs_of_diffs = current_diff
            .windows(2)
            .map(|w| *w.get(1).unwrap() - w.get(0).unwrap())
            .collect::<Vec<_>>();
        // println!("diffs: {diffs_of_diffs:?}");
        if diffs_of_diffs.is_empty() || diffs_of_diffs.iter().all(|ele| *ele == 0) {
            break
        }
        diff_collect.push(diffs_of_diffs.clone());
        current_diff = diffs_of_diffs;
    }
    diff_collect
}

pub fn part1(inputs: &[String]) -> i32 {
    let mut extrapolated_values_sum = 0;
    for input in prepare_input(inputs){
        // given vector is pushed as-is
        let collected_rows = collect_rows(input);
        let current_row = collected_rows
            .get(0)
            .unwrap();

        let diff_lists = get_recursive_diff_last_values(current_row.clone());
        let next_val_diff: i32 = diff_lists
            .iter()
            .map(|vec| vec.last().unwrap())
            .sum();
        let next_val = current_row.last().unwrap() + next_val_diff;
        // println!("current_row={current_row:?}");
        // println!("next_val={next_val_diff:?} --> {next_val:?}");
        extrapolated_values_sum += next_val;
    }
    extrapolated_values_sum
}

pub fn part2(inputs: &[String]) -> i32 {
    let mut extrapolated_values_sum = 0;
    for input in prepare_input(inputs){
        // given vector is pushed as-is
        let collected_rows = collect_rows(input);
        let current_row = collected_rows
            .get(0)
            .unwrap();

        println!("current_row={current_row:?}");
        let mut current_diff = current_row.clone();
        current_diff.reverse();
        let diff_lists = get_recursive_diff_last_values(current_diff);
        let next_val_diff: i32 = diff_lists
            .iter()
            .map(|vec| vec.last().unwrap())
            .sum();
        let next_val = current_row.first().unwrap() + next_val_diff;
        println!("next_val={next_val_diff:?} --> {next_val:?}");
        extrapolated_values_sum += next_val;
    }
    extrapolated_values_sum
}

#[allow(clippy::get_first)]
fn collect_rows(input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut collected_rows = vec![input.clone()];
    let mut current_vector = collected_rows.first().unwrap();
    'reach_0: loop {
        let mut collected_row_current = vec![];
        for num in current_vector.windows(2) {
            let diff = (*num).get(1).unwrap() - (*num).get(0).unwrap();
            collected_row_current.push(diff);
        }
        if collected_row_current.iter().all(|d| d == &0) {
            collected_rows.push(collected_row_current);
            break 'reach_0
        }
        collected_rows.push(collected_row_current.clone());
        current_vector = collected_rows.last().unwrap();
    }
    collected_rows
}
