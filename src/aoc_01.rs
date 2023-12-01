pub fn main(inputs: Vec<String>) {
    println!("Part 1: {}", part1(&inputs));
    println!("Part 2: {}", part2(&inputs));
}

fn part1(inputs: &Vec<String>) -> i32 {
    let mut sum = 0;
    for input in inputs {
        let first_digit = input
            .chars()
            .filter(|char| char.is_digit(10))
            .next()
            .unwrap().to_digit(10)
            .unwrap();
        let last_digit = input
            .chars()
            .rev()
            .filter(|char| char.is_digit(10))
            .next()
            .unwrap().to_digit(10)
            .unwrap();
        let s: i32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
        sum = sum + s;
    }
    sum
}

fn part2(inputs: &Vec<String>) -> i32 {

    fn is_digit_at_idx(input: &String, idx: usize) -> Option<i32>{
        let char: Vec<char> = input.chars().skip(idx).take(1).collect();
        if let Some(digit) = char.get(0).unwrap().to_digit(10){
            return Some(digit as i32);
        }
        return None
    }

    fn forward_or_backward(input: &String,
                           numbers: &Vec<(String, i32)>,
                           forward: bool) -> Option<i32> {
        let mut digit = None;
        let iterator: Box<dyn Iterator<Item=usize>>;
        if forward {
            iterator = Box::new(0..input.len());
        } else {
            iterator = Box::new((0..input.len()).rev());
        }
        for idx in iterator {
            // simple digit check
            if let Some(x) = is_digit_at_idx(&input, idx){
                digit = Some(x);
                break
            }
            // forward / backward search
            for (number_s, number_i) in numbers {
                if forward {
                    // check for every index in the line if the current digit string starts here
                    if number_s.len() + idx <= input.chars().count() {
                        let substr: String = input.chars().skip(idx).take(number_s.len()).collect();
                        if substr == number_s.to_string() {
                            digit = Some(*number_i);
                            break
                        }
                    }
                } else {
                    if idx >= number_s.len() {
                        let substr: String = input.chars().skip(idx - number_s.len() + 1).take(number_s.len()).collect();
                        if substr == number_s.to_string() {
                            digit = Some(*number_i);
                            break
                        }
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
        let first_digit = forward_or_backward(&input, &numbers, true);
        let last_digit = forward_or_backward(&input, &numbers, false);


        let s: i32 = format!("{}{}",
                             first_digit.expect("First digit is none"),
                             last_digit.expect("Last digit is none")).parse().unwrap();
        sum = sum + s;
    }
    sum
}