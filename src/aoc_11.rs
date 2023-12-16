use std::cmp::Ordering;
use std::collections::HashMap;

pub fn main(inputs: &[String]) {
    println!("Part 1: {}", part1(inputs));
    println!("Part 2: {}", part2(inputs));
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coordinate {
    is_universe: bool,
    x: usize,
    y: usize,
    x_offset: usize,
    y_offset: usize
}

impl Coordinate {
    fn new(x: usize, y: usize, is_universe: bool) -> Self {
        Coordinate {
            is_universe,
            x,
            y,
            x_offset: 0,
            y_offset: 0
        }
    }

    fn add_x_offset(&mut self, offset: usize) {
        self.x_offset += offset;
    }

    fn add_y_offset(&mut self, offset: usize) {
        self.y_offset += offset;
    }

    fn get_x(&self) -> usize {
        self.x + self.x_offset
    }
    fn get_y(&self) -> usize {
        self.y + self.y_offset
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.get_x(), self.get_y()).cmp(&(other.get_x(), other.get_y())))
    }
}


fn parse_input(inputs: &[String]) -> Vec<Vec<Coordinate>> {
    inputs
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .enumerate()
        .map(|(idx_row, line)| line
            .iter()
            .enumerate()
            .map(|(idx_col, char)| Coordinate::new(idx_row, idx_col, char == &'#'))
            .collect()
        )
        .collect::<Vec<Vec<Coordinate>>>()
}

fn expand(mut universe: Vec<Vec<Coordinate>>, additional_dimensions: usize) -> Vec<Vec<Coordinate>> {
    let first_row_len = universe.get(0).unwrap().len();
    for col in (0..first_row_len).rev() {
        let empty_col = universe
            .iter()
            .map(|row| row.get(col).unwrap())
            .all(|coordinate| !coordinate.is_universe);
        if empty_col {
            for row in 0..universe.len() {
                for col_ in col..first_row_len {
                    // add factor to all coordinates "right to" this col
                    universe
                        .get_mut(row)
                        .unwrap()
                        .get_mut(col_)
                        .unwrap()
                        .add_y_offset(additional_dimensions);
                }
            }
        }
    }
    for row in (0..universe.len()).rev() {
        let row_iter = universe
            .get(row)
            .unwrap();
        let empty_row = row_iter
            .iter()
            .all(|coordinate| !coordinate.is_universe);
        if empty_row {
            // add factor to all coordinates "below" this row
            for row in row..universe.len() {
                universe
                    .get_mut(row)
                    .unwrap()
                    .iter_mut()
                    .for_each(|coordinate| coordinate.add_x_offset(additional_dimensions))
            }
        }
    }
    universe
}

fn get_shortest_distance_between_points(point_larger: &Coordinate, point_smaller: &Coordinate) -> usize {
    point_larger.get_x().abs_diff(point_smaller.get_x()) + point_larger.get_y().abs_diff(point_smaller.get_y())
}

fn get_universe_coordinates(universe: &[Vec<Coordinate>]) -> Vec<&Coordinate> {
    universe
        .iter()
        .flat_map(|row| row
            .iter()
            .filter(|x| x.is_universe)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>()
}

fn part_1_and_2(inputs: &[String], additional_dimensions: usize) -> usize {
    let parsed = parse_input(inputs);
    let expanded = expand(parsed, additional_dimensions);
    let coordinates = get_universe_coordinates(&expanded);
    let distances = calculate_min_distances(&coordinates);
    distances.values().sum()
}

fn calculate_min_distances<'a>(coordinates: &'a Vec<&'a Coordinate>) -> HashMap<(&'a Coordinate, &'a Coordinate), usize> {
    let mut distances: HashMap<(&Coordinate, &Coordinate), usize> = HashMap::new();
    for coordinate_1 in coordinates {
        for coordinate_2 in coordinates {
            if coordinate_1 == coordinate_2 {
                continue
            }
            // only consider tuple of coordinates sorted to avoid duplicates of (x,y) <-> (y,x)
            let coordinate_merge = if coordinate_1 > coordinate_2 {
                (*coordinate_1, *coordinate_2)
            } else {
                (*coordinate_2, *coordinate_1)
            };
            distances.entry(coordinate_merge).or_insert_with(|| {
                // use sorted tuples to pass into the fn
                get_shortest_distance_between_points(coordinate_merge.0, coordinate_merge.1)
            });
        }
    }
    distances
}

pub fn part1(inputs: &[String]) -> usize {
    part_1_and_2(inputs, 1)
}

pub fn part2(inputs: &[String]) -> usize {
    part_1_and_2(inputs, 999_999)
}
