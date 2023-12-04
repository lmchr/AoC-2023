pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

pub fn part1(inputs: &[String]) -> u32 {
    let mut sum = 0;
    for input in inputs {
        let first_digit = input
            .chars()
            .find(|char| char.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
        let last_digit = input
            .chars()
            .rev()
            .find(|char| char.is_ascii_digit())
            .unwrap().
            to_digit(10)
            .unwrap();
        let s: u32 = first_digit * 10 + last_digit;
        sum += s;
    }
    sum
}

pub fn part2(inputs: &[String]) -> i32 {

    fn is_digit_at_idx(input: &str, idx: usize) -> Option<i32>{
        let char = &input[idx..idx+1];
        if let Ok(digit) = char.parse::<i32>(){
            return Some(digit);
        }
        None
    }

    fn forward_or_backward(input: &str,
                           numbers: &[(String, i32)],
                           forward: bool) -> Option<i32> {
        let mut digit = None;
        let iterator: Box<dyn Iterator<Item=usize>> = if forward {
            Box::new(0..input.len())
        } else {
            Box::new((0..input.len()).rev())
        };
        for idx in iterator {
            // simple digit check
            if let Some(x) = is_digit_at_idx(input, idx){
                digit = Some(x);
                break
            }
            // forward / backward search
            for (number_s, number_i) in numbers {
                if forward {
                    // check for every index in the line if the current digit string starts here
                    if number_s.len() + idx <= input.chars().count() {
                        let substr = &input[idx..idx + number_s.len()];
                        if substr == number_s {
                            digit = Some(*number_i);
                            break
                        }
                    }
                } else if idx >= number_s.len() {
                    let substr = &input[idx - number_s.len() + 1..idx - number_s.len() + 1 + number_s.len()];
                    if substr == number_s {
                        digit = Some(*number_i);
                        break
                    }
                }
            }
            if digit.is_some() {
                break
            }
        }
        digit
    }

    let numbers: Vec<(String, i32)> = vec!(
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9)
    );

    let mut sum = 0;
    for input in inputs {
        let first_digit = forward_or_backward(input, &numbers, true);
        let last_digit = forward_or_backward(input, &numbers, false);


        let s: i32 = first_digit.expect("First digit is none") * 10 +
                     last_digit.expect("Last digit is none");
        sum += s;
    }
    sum
}