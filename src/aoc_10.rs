use strum_macros::EnumIter;
use strum::IntoEnumIterator;

pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}

fn parse_input(inputs: &[String]) -> Vec<Vec<char>>{
    inputs
        .iter()
        .map(|line| line
            .chars()
            .collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>()
}

fn get_char_at_pos<T: std::fmt::Debug + Clone + Copy>(inputs: &[Vec<T>], pos: &(i32, i32)) -> Option<T> {
    if let Some(x) = inputs.get(pos.0 as usize) {
        if let Some(&y) = x.get(pos.1 as usize) {
            return Some(y);
        }
    }
    None
}

#[derive(EnumIter, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
enum Symbols {
    PipeVertical = b'|',
    PipeHorizontal = b'-',
    J = b'J',
    L = b'L',
    Seven = b'7',
    F = b'F',
    S = b'S',
    Dot = b'.'
}

impl Symbols {
    pub fn value(&self) -> ((i32, i32), (i32, i32)) {
        match self.as_char() {
            '|' => ((1, 0), // south
                    (-1, 0)),  // north
            '-' => ((0, 1),  // east
                    (0, -1)),  // west
            'J' => ((-1, 0),  // north
                    (0, -1)),  // west
            'L' => ((-1, 0),  // north
                    (0, 1)),  // east
            '7' => ((0, -1),  // west
                    (1, 0)),  // south
            'F' => ((0, 1),  // east
                    (1, 0)),  // south
            'S' => ((0, 0),
                    (0, 0)),
            '.' => ((0, 0),
                    (0, 0)),
            _ => panic!(),
        }
    }

    pub fn by_value(value: &char) -> Symbols {
        for symbol in Symbols::iter() {
            if symbol.as_char() == *value {
                return symbol
            }
        }
        panic!("Invalid symbol: {}", value)
    }

    fn as_char(&self) -> char {
        *self as u8 as char
    }
}

fn recurse(inputs: &Vec<Vec<char>>, mut distances: Vec<Vec<i32>>, distance_from_s: i32, current_pos: &(i32, i32)) -> Vec<Vec<i32>> {
    //println!("recursion at {:?}", current_pos);
    if let Some(current_pos_char) = get_char_at_pos(inputs, current_pos) {
        //println!("Recurse, level: {distance_from_s}, currently at {current_pos:?}, char: {current_pos_char}");
        let lookup = Symbols::by_value(&current_pos_char);
        if lookup != Symbols::Dot {
            // println!("Setting for char {}={}; current symbol: {lookup:?}", inputs[current_pos.0 as usize][current_pos.1 as usize], distance_from_s);
            distances[current_pos.0 as usize][current_pos.1 as usize] = distance_from_s;
            let mut current_pos_new = *current_pos;
            if lookup != Symbols::S {
                let distance_lookup = get_char_at_pos(&distances, &(current_pos_new.0 + lookup.value().0.0, current_pos_new.1 + lookup.value().0.1));
                let input_lookup = get_char_at_pos(inputs, &(current_pos_new.0 + lookup.value().0.0, current_pos_new.1 + lookup.value().0.1));
                if input_lookup.is_some() && input_lookup.unwrap() != '.' && (distance_lookup.unwrap() == -1_i32 || distance_lookup.unwrap() > distance_from_s + 1) {
                    current_pos_new = (current_pos_new.0 + lookup.value().0.0, current_pos_new.1 + lookup.value().0.1);
                } else {
                    let distance_lookup = get_char_at_pos(&distances, &(current_pos_new.0 + lookup.value().1.0, current_pos_new.1 + lookup.value().1.1));
                    let input_lookup = get_char_at_pos(inputs, &(current_pos_new.0 + lookup.value().1.0, current_pos_new.1 + lookup.value().1.1));
                    // println!("input_lookup={:?} at pos {:?}", input_lookup, (current_pos_new.0 + lookup.value().1.0, current_pos_new.1 + lookup.value().1.1));
                    if input_lookup.is_some() && input_lookup.unwrap() != '.' && (distance_lookup.unwrap() == -1_i32 || distance_lookup.unwrap() > distance_from_s + 1) {
                        current_pos_new = (current_pos_new.0 + lookup.value().1.0, current_pos_new.1 + lookup.value().1.1);
                    }
                }
                // check if any of the conditions above checked, else don't recurse
                if &current_pos_new != current_pos {
                    distances = recurse(inputs, distances, distance_from_s + 1, &current_pos_new);
                }
            } else if lookup == Symbols::S {
                // figure out which symbol the start is
                let start_symbol = figure_out_s_symbol(&inputs, &current_pos, &mut current_pos_new);
                //println!("S={:?}", start_symbol);
                // start off by going in both directions
                let v1 = start_symbol.value().0;
                let v2 = start_symbol.value().1;
                distances = recurse(inputs, distances, distance_from_s + 1, &(current_pos_new.0 + v1.0, current_pos_new.1 + v1.1));
                distances = recurse(inputs, distances, distance_from_s + 1, &(current_pos_new.0 + v2.0, current_pos_new.1 + v2.1));
            }
        }
    }
    distances
}

fn figure_out_s_symbol(inputs: &&Vec<Vec<char>>, current_pos: &&(i32, i32), current_pos_new: &mut (i32, i32)) -> Symbols {
    let north = get_char_at_pos(inputs, &(current_pos_new.0 - 1, current_pos.1));
    let east = get_char_at_pos(inputs, &(current_pos_new.0, current_pos.1 + 1));
    let south = get_char_at_pos(inputs, &(current_pos_new.0 + 1, current_pos.1));
    let west = get_char_at_pos(inputs, &(current_pos_new.0, current_pos.1 - 1));
    let is_north = if let Some(north_) = north {[Symbols::PipeVertical.as_char(), Symbols::F.as_char(), Symbols::Seven.as_char()].contains(&north_)} else { false };
    let is_east = if let Some(east_) = east {[Symbols::PipeHorizontal.as_char(), Symbols::J.as_char(), Symbols::Seven.as_char()].contains(&east_)} else { false };
    let is_south = if let Some(south_) = south {[Symbols::PipeVertical.as_char(), Symbols::J.as_char(), Symbols::L.as_char()].contains(&south_)} else { false };
    let is_west = if let Some(west_) = west {[Symbols::PipeHorizontal.as_char(), Symbols::F.as_char(), Symbols::L.as_char()].contains(&west_)} else { false };

    let num_directions_to_go = is_north as u8 + is_east as u8 + is_south as u8 + is_west as u8;
    if num_directions_to_go != 2 {
        panic!("too many or too few possible directions: {is_north} {is_east} {is_south} {is_west}")
    }
    if is_north && is_east {
        return Symbols::L;
    }
    if is_north && is_south {
        return Symbols::PipeVertical;
    }
    if is_north && is_west {
        return Symbols::J;
    }
    if is_east && is_south {
        return Symbols::F;
    }
    if is_east && is_west {
        return Symbols::PipeHorizontal;
    }
    if is_west && is_south {
        return Symbols::Seven;
    }
    panic!("Unknown direction to go")
}

pub fn part1(inputs: &[String]) -> i32 {
    let map = parse_input(inputs);
    let s_find = map
        .iter()
        .enumerate()
        .map(|(idx, line)| line
            .iter()
            .enumerate()
            .filter(|(_idx2, char)| *char == &'S' )
            .map(|(idx2, _char)| (idx, idx2) )
            .collect::<Vec<_>>()
        )
        .find(|x| !x.is_empty())
        .unwrap();

    println!("map=");
    map.iter().for_each(|it| {
        println!("{:?}", it);
    });

    let position_s = s_find.first().unwrap();
    let position_s_i32 = (position_s.0 as i32, position_s.1 as i32);

    let distances = create_empty_distance_2d_vector(&map);
    let distances = recurse(&map, distances, 0, &position_s_i32);
    println!("distances=");
    distances.iter().for_each(|it| {
        println!("{:?}", it);
    });
    *distances
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
}

fn create_empty_distance_2d_vector(parse: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let mut distances = vec![];
    for _ in 0..parse.len() {
        let mut tmp_l: Vec<i32> = vec![];
        for _ in 0..parse.get(0).unwrap().len() {
            tmp_l.push(-1);
        }
        distances.push(tmp_l);
    }
    distances
}

pub fn part2(_inputs: &[String]) -> i32 {
    0
}
