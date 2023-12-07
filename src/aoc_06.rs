pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

fn part1(inputs: &[String]) -> i64 {
    let times: Vec<i32> = inputs
        .get(0)
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let distances: Vec<i32> = inputs
        .get(1)
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut num_ways_to_win = vec![];
    for (a, b) in times.iter().zip(&distances){
        let mut race_ways_to_win = 0;
        for hold_time_ms in 0..*a {
            let travel_time = a - hold_time_ms;
            let distance = travel_time * hold_time_ms;
            if distance > *b {
                race_ways_to_win += 1;
            }
        }
        num_ways_to_win.push(race_ways_to_win);
    }
    num_ways_to_win.iter().product()
}

pub fn part2(inputs: &[String]) -> i64 {
    let time = &inputs
        .get(0)
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    let distance = &inputs
        .get(1)
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    let mut race_ways_to_win: i64 = 0;
    for hold_time_ms in 0..*time {
        let travel_time = time - hold_time_ms;
        let distance_race: i64 = travel_time * hold_time_ms;
        if distance_race > *distance {
            race_ways_to_win += 1;
        }
    }
    race_ways_to_win
}