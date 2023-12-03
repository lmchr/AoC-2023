use std::collections::HashMap;

pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

pub fn part1(inputs: &[String]) -> usize {
    let game_cubes = HashMap::from([
       ("red", 12),
       ("green", 13),
       ("blue", 14)
    ]);
    let mut playable_games_sum = 0;
    for (idx, input) in inputs.iter().enumerate() {
        let cleaned_input = input
            .trim_start_matches("Game ")
            .trim_start_matches(char::is_numeric)  // cleans game number
            .replace(": ", "");

        let groups = cleaned_input.split("; ");
        let mut group_ok = true;
        for digit_color_strings in groups {
            let digit_color_string_groups = digit_color_strings.split(", ");

            let mut round_colors: HashMap<&str, i32> = HashMap::new();
            for digit_color_string_group in digit_color_string_groups {
                let digit_and_color = digit_color_string_group
                    .split(' ')
                    .collect::<Vec<&str>>();
                let digit = digit_and_color.first().unwrap().parse::<i32>().unwrap();
                let color = digit_and_color.last().unwrap();
                *round_colors.entry(color).or_insert(0) += digit;
            }
            // check if there are enough cubes for this round
            let mut round_ok = true;
            for (color, amount) in round_colors {
                if *game_cubes.get(color).unwrap() < amount {
                    round_ok = false;
                    break
                }
            }
            if !round_ok {
                group_ok = false;
                break
            }
        }
        if group_ok {
            playable_games_sum += idx + 1;
        }
    }
    playable_games_sum
}

pub fn part2(inputs: &[String]) -> i32 {
    let mut power_sum = 0;
    for input in inputs {
        let cleaned_input = input
            .trim_start_matches("Game ")
            .trim_start_matches(char::is_numeric)  // cleans game number
            .replace(": ", "");

        let groups = cleaned_input.split("; ");
        let mut minimum_colors_per_group: HashMap<&str, i32> = HashMap::new();
        for digit_color_strings in groups {
            let digit_color_string_groups = digit_color_strings.split(", ");

            for digit_color_string_group in digit_color_string_groups {
                let digit_and_color = digit_color_string_group
                    .split(' ')
                    .collect::<Vec<&str>>();
                let digit = digit_and_color.first().unwrap().parse::<i32>().unwrap();
                let color = digit_and_color.last().unwrap();
                match minimum_colors_per_group.get(color) {
                    Some(value) => {
                        // is the current color amount higher, overwrite it
                        if *value < digit {
                            *minimum_colors_per_group.get_mut(color).unwrap() = digit;
                        }
                    }
                    None => {
                        minimum_colors_per_group.insert(color, digit);
                    }
                }
            }
        }
        let power: i32 = minimum_colors_per_group
            .values()
            .filter(|val| **val != 0)
            .product();
        power_sum += power;
    }
    power_sum
}