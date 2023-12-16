use std::collections::HashMap;

pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

fn parse_input(inputs: &[String]) -> Vec<&str> {
    inputs.get(0).unwrap().split(',').collect()
}

pub fn part1(inputs: &[String]) -> u32 {
    let parsed = parse_input(inputs);
    let mut result: u32 = 0;
    for parse in parsed{
        let value = hash(parse);
        result += value;
    }
    result
}

fn hash(parse: &str) -> u32 {
    let mut value: u32 = 0;
    for char in parse.chars() {
        value += char as u32;
        value *= 17;
        value %= 256;
    }
    value
}

pub fn part2(inputs: &[String]) -> u32 {
    let parsed = parse_input(inputs);
    let parsed2: Vec<(&str, i32)> = parsed
        .iter()
        .map(|line| {
          if line.ends_with('-'){
              (line.strip_suffix('-').unwrap(), -1_i32)
          } else {
              let split = line.split('=').collect::<Vec<_>>();
              (*split.first().unwrap(), split.last().unwrap().parse::<i32>().unwrap())
          }
        })
        .collect();
    let mut map: HashMap<u32, Vec<(&str, i32)>> = HashMap::new();
    for (key, focal_length) in &parsed2 {
        let value = hash(key);
        let entry = map.contains_key(&value);
        if focal_length == &-1 { // remove
            if entry {
                let existing_values = map.get_mut(&value).unwrap();
                existing_values.retain(|(key_iter, _)| key_iter != key);
            }
        } else if !entry {  // doesnt exist at all
            map.insert(value, vec![(*key, *focal_length)]);
        } else {
            // replace existing key
            let mut replaced = false;
            let new_value = (*key, *focal_length);
            for c in map.get_mut(&value).unwrap(){
                if c.0 == *key {
                    *c = new_value;
                    replaced = true;
                    break
                }
            }
            if !replaced {
                map.get_mut(&value).unwrap().push((*key, *focal_length));
            }
        }
    }
    let mut sum: u32 = 0;
    for (idx, content) in map.iter() {
        for (idx_splot, (_, focal_length)) in content.iter().enumerate() {
            sum += (idx + 1) * (idx_splot as u32 + 1) * (*focal_length as u32)
        }
    }
    sum
}
