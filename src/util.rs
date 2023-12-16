use std::fs;
use std::path::Path;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone)]
pub enum Days {
    ONE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    ELEVEN = 11,
}

impl Days {
    pub fn value(&self) -> u8 {
        // needs Copy, Clone traits
        *self as u8
    }

    pub fn by_value(value: &u8) -> Days {
        for day in Days::iter() {
            if &day.value() == value {
                return day
            }
        }
        panic!("Invalid date: {}", value)
    }
}

pub fn read_input(day: &Days) -> Vec<String> {
    let path = Path::new("data").join(format!("aoc_{:0>2}.txt", day.value()));
    let lines: Vec<String> = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Read file from {:?} failed due to {}", &path, err))
        .lines()
        .map(|t| t.to_string())
        .collect();
    lines
}
