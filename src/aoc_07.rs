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

    fn get_best_joker_hand_type(&self, ranks: &Vec<char>) -> Type {
        // get all possible permutations
        let a = if self.cards[0] == 'J' {&ranks} else {&self.cards[0..1]};
        let b = if self.cards[1] == 'J' {&ranks} else {&self.cards[1..2]};
        let c = if self.cards[2] == 'J' {&ranks} else {&self.cards[2..3]};
        let d = if self.cards[3] == 'J' {&ranks} else {&self.cards[3..4]};
        let e = if self.cards[4] == 'J' {&ranks} else {&self.cards[4..5]};
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

    let hands: Vec<Hand> = inputs
        .iter()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|list| Hand{ cards: list.first().unwrap().chars().collect::<Vec<char>>(),
                                       bid: list.last().unwrap().parse::<i32>().unwrap()})
        .collect();
    let sorted: Vec<_> = hands
        .iter()
        .sorted_by_key(|hand| (
            get_hand_type(&hand.cards).value(),
            hand.get_card_value_at_pos(&ranks, 0),
            hand.get_card_value_at_pos(&ranks, 1),
            hand.get_card_value_at_pos(&ranks, 2),
            hand.get_card_value_at_pos(&ranks, 3),
            hand.get_card_value_at_pos(&ranks, 4)
        ))
        .rev()
        .collect();
    println!("{:?}", sorted);
    sorted
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as i64 + 1) * hand.bid as i64)
        .sum()
}

pub fn part2(inputs: &[String]) -> i64 {
    let ranks_j: Vec<char> = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
    let ranks: Vec<char> = ranks_j.clone().into_iter().take(ranks_j.len()).collect();

    let hands: Vec<Hand> = inputs
        .iter()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|list| Hand{ cards: list.first().unwrap().chars().collect::<Vec<char>>(),
            bid: list.last().unwrap().parse::<i32>().unwrap()})
        .collect();
    let sorted: Vec<_> = hands
        .iter()
        .sorted_by_key(|hand| (
            hand.get_best_joker_hand_type(&ranks).value(),
            hand.get_card_value_at_pos(&ranks_j, 0),
            hand.get_card_value_at_pos(&ranks_j, 1),
            hand.get_card_value_at_pos(&ranks_j, 2),
            hand.get_card_value_at_pos(&ranks_j, 3),
            hand.get_card_value_at_pos(&ranks_j, 4)
        ))
        .rev()
        .collect();
    println!("{:?}", sorted);
    sorted
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as i64 + 1) * hand.bid as i64)
        .sum()
}