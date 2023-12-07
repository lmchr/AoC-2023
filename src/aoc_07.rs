use std::collections::HashSet;
use itertools::Itertools;
use strum_macros::EnumIter;

pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: i32
}

impl Hand {
    fn get_card_value_at_pos(&self, ranks: &Vec<char>, pos: usize) -> usize {
        let x =  self.cards.get(pos).unwrap();
        ranks
            .iter()
            .position(|c| c == x)
            .unwrap()
    }
}


fn get_best_joker_hand_type(cards: &Vec<char>, ranks: &Vec<char>) -> Type {
    // get all possible permutations
    let a = if cards[0] == 'J' {&ranks} else {&cards[0..1]};
    let b = if cards[1] == 'J' {&ranks} else {&cards[1..2]};
    let c = if cards[2] == 'J' {&ranks} else {&cards[2..3]};
    let d = if cards[3] == 'J' {&ranks} else {&cards[3..4]};
    let e = if cards[4] == 'J' {&ranks} else {&cards[4..5]};
    let mut best = Type::HighCard;
    for aa in a{
        for bb in b{
            for cc in c {
                for dd in d {
                    for ee in e{
                        let v = vec![*aa,*bb,*cc,*dd,*ee];
                        let check = get_hand_type(&v);
                        if check.value() < best.value() {
                            best = check;
                        }
                    }
                }
            }
        }
    }
    best
}

fn get_hand_type(cards: &Vec<char>) -> Type {
    let unique_elements = cards
        .iter()
        .collect::<HashSet<&char>>()
        .into_iter()
        .collect::<Vec<_>>();
    let unique_no = unique_elements.len();
    let counts_per_unique_element = unique_elements
        .iter()
        .map(|ele| cards.iter().filter(|card_hand| *card_hand == *ele).count())
        .collect::<Vec<usize>>();
    // println!("{cards:?}, unique_no:{unique_no}, counts_per_unique_element:{counts_per_unique_element:?}");
    if unique_no == 1 {
        return Type::FiveOfAKid;
    } else if unique_no == 2 { // 22233 or 22223
        return if counts_per_unique_element.iter().any(|ele| *ele == 4) {
            Type::FourOfAKid
        } else {
            Type::FullHouse
        }
    } else if unique_no == 3 { // 22334 or 23444
        return if counts_per_unique_element.iter().any(|ele| *ele == 3){
            Type::ThreeOfAKind
        } else {
            Type::TwoPair
        }
    } else if unique_no == 4 {
        return Type::OnePair;
    } else if unique_no == 5 {
        return Type::HighCard;
    }
    panic!();
}

#[derive(Debug)]
#[derive(EnumIter, Copy, Clone)]
enum Type {
    FiveOfAKid = 0,
    FourOfAKid = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPair = 4,
    OnePair = 5,
    HighCard = 6
}

impl Type {
    pub fn value(&self) -> u8 {
        // needs Copy, Clone traits
        *self as u8
    }
}

pub fn part1(inputs: &[String]) -> i64 {
    let ranks: Vec<char> = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
    let hands = get_hands(inputs);
    let sorted = sort_hand(&ranks, &ranks, &hands, true);
    println!("{:?}", sorted);
    get_end_result(sorted)
}

pub fn part2(inputs: &[String]) -> i64 {
    let ranks_j: Vec<char> = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    let ranks: Vec<char> = ranks_j.clone().into_iter().take(ranks_j.len()).collect();
    let hands = get_hands(inputs);
    let sorted = sort_hand(&ranks_j, &ranks, &hands, false);
    println!("{:?}", sorted);
    get_end_result(sorted)
}

fn get_hand_type_p1_p2(cards: &Vec<char>, ranks: &Vec<char>, is_p1: bool) -> u8 {
    return if is_p1 {
        get_hand_type(cards).value()
    } else {
        get_best_joker_hand_type(cards, ranks).value()
    }
}

fn get_end_result(sorted: Vec<&Hand>) -> i64 {
    sorted
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as i64 + 1) * hand.bid as i64)
        .sum()
}

fn sort_hand<'a>(ranks_j: &'a Vec<char>, ranks: &'a Vec<char>, hands: &'a Vec<Hand>, is_p1: bool) -> Vec<&'a Hand> {
    hands
        .iter()
        .sorted_by_key(|hand| (
            get_hand_type_p1_p2(&hand.cards, &ranks, is_p1),
            hand.get_card_value_at_pos(&ranks_j, 0),
            hand.get_card_value_at_pos(&ranks_j, 1),
            hand.get_card_value_at_pos(&ranks_j, 2),
            hand.get_card_value_at_pos(&ranks_j, 3),
            hand.get_card_value_at_pos(&ranks_j, 4)
        ))
        .rev()
        .collect()
}

fn get_hands(inputs: &[String]) -> Vec<Hand> {
    let hands: Vec<Hand> = inputs
        .iter()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|list| Hand {
            cards: list.first().unwrap().chars().collect::<Vec<char>>(),
            bid: list.last().unwrap().parse::<i32>().unwrap()
        })
        .collect();
    hands
}
