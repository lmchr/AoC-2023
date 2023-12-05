use std::collections::HashMap;
use core::ops::Range;

pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

pub fn part1(inputs: &[String]) -> i64 {
    let first_line = inputs.first().unwrap();
    let seeds: Vec<i64> = first_line
        .strip_prefix("seeds: ")
        .unwrap().split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    println!("{:?}", seeds);

    let mut collect = vec![];
    let mut collect_tmp = vec![];
    for x in inputs
        .iter()
        .skip(2)
        .collect::<Vec<&String>>(){
        if !x.is_empty(){
            collect_tmp.push(x)
        } else {
            collect.push(collect_tmp.clone());
            collect_tmp.clear();
        }
    }
    collect.push(collect_tmp.clone());
    let mut main_collect: HashMap<(String, String), Vec<(Range<i64>, Range<i64>)>> = HashMap::new();
    for yy in &collect {
        let mut x_to_y_map: Vec<(Range<i64>, Range<i64>)> = Vec::new();
        let mut from_to_tuple: Option<(String, String)> = None;
        for (idx, yyy) in yy.iter().enumerate() {
            if idx == 0 {
                let from_to_map: Vec<&str> = yyy
                    .strip_suffix(" map:")
                    .unwrap()
                    .split("-to-")
                    .collect();
                let from = from_to_map.first().unwrap();
                let to = from_to_map.last().unwrap();
                from_to_tuple = Some((from.to_string(), to.to_string()));
            } else {
                let three_numbers: Vec<&str> = yyy.split(' ').collect();
                let start = three_numbers.get(0).unwrap().parse::<i64>().unwrap();
                let end = three_numbers.get(1).unwrap().parse::<i64>().unwrap();
                let length = three_numbers.get(2).unwrap().parse::<i64>().unwrap();
                let source_range = end..end+length;
                let destination_range = start..start+length;
                x_to_y_map.push((source_range, destination_range));
            }
        }
        main_collect.insert(from_to_tuple.unwrap(), x_to_y_map.clone());
        x_to_y_map.clear();
    }

    fn get_next_value(main_collect: &HashMap<(String, String), Vec<(Range<i64>, Range<i64>)>>,
                      from: &str, to: &str,
                      val: i64) -> i64 {
        let y = main_collect.get(&(from.to_string(), to.to_string())).unwrap();
        for (range1, range2) in y.iter(){
            if val >= range1.start && val <= range1.end {
                return range2.start + (val - range1.start)
            }
        }
        val  // Any source numbers that aren't mapped correspond to the same destination number.
    }

    println!("here: {:?}", main_collect);
    // map the initial seeds to the final destination
    let mut final_locations = Vec::with_capacity(seeds.len());
    for seed in seeds {
        let soil: i64 = get_next_value(&main_collect, "seed", "soil", seed);
        let fertilizer: i64 = get_next_value(&main_collect, "soil", "fertilizer", soil);
        let water: i64 = get_next_value(&main_collect, "fertilizer", "water", fertilizer);
        let light: i64 = get_next_value(&main_collect, "water", "light", water);
        let temperature: i64 = get_next_value(&main_collect, "light", "temperature", light);
        let humidity: i64 = get_next_value(&main_collect, "temperature", "humidity", temperature);
        let location: i64 = get_next_value(&main_collect, "humidity", "location", humidity);
        final_locations.push(location);
    }
    *final_locations.iter().min().unwrap()
}

pub fn part2(_inputs: &[String]) -> u32 {
    0
}