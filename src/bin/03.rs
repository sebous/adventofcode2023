use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::grid::{Coord, Grid};

advent_of_code::solution!(3);

enum Point {
    Value(u32),
    Symbol(char),
    Nothing,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let print_val = match self {
            Self::Nothing => ".".to_string(),
            Self::Symbol(char) => char.to_string(),
            Self::Value(val) => val.to_string(),
        };
        write!(f, "{:?}", print_val)
    }
}

fn parse_input(input: &str) -> Grid<Point> {
    let mut map = HashMap::new();
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    for (y, line) in input.lines().enumerate() {
        for (x, point) in line.chars().enumerate() {
            let point = match point {
                point if point.is_digit(10) => Point::Value(point.to_digit(10).unwrap()),
                '.' => Point::Nothing,
                char => Point::Symbol(char),
            };
            map.insert((x, y), point);
        }
    }

    Grid { map, height, width }
}

fn get_whole_num((from_x, from_y): &Coord, grid: &Grid<Point>) -> VecDeque<(Coord, u32)> {
    let mut num_parts = VecDeque::new();

    for x in (0..*from_x).rev() {
        if let Some(point) = grid.map.get(&(x, *from_y)) {
            match point {
                Point::Value(n) => num_parts.push_front(((x, *from_y), n.to_owned())),
                _ => break,
            }
        }
    }

    for x in *from_x..grid.width {
        if let Some(point) = grid.map.get(&(x, *from_y)) {
            match point {
                Point::Value(n) => num_parts.push_back(((x, *from_y), n.to_owned())),
                _ => break,
            }
        }
    }

    num_parts
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut nums = vec![];
    let mut visited_coords = HashSet::new();

    for (coord, point) in grid.map.iter() {
        match point {
            Point::Symbol(_) => {
                for c in grid.get_adjacent_coords(coord, true) {
                    let c = match c {
                        Some(c) if visited_coords.get(&c).is_some() => continue,
                        Some(c) => c,
                        None => continue,
                    };

                    let point = grid.map.get(&c).unwrap();

                    match point {
                        Point::Value(_) => {
                            let whole_num_info = get_whole_num(&c, &grid);

                            whole_num_info.iter().for_each(|(c, _)| {
                                visited_coords.insert(*c);
                            });

                            let whole_num = whole_num_info
                                .iter()
                                .map(|(_, val)| val)
                                .fold(String::new(), |acc, x| acc + x.to_string().as_str())
                                .parse::<u32>()
                                .unwrap();

                            nums.push(whole_num);
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }

    Some(nums.iter().sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut visited_coords = HashSet::new();
    let mut total_gear_ratio = 0;

    for (coord, point) in grid.map.iter() {
        match point {
            Point::Symbol(s) if *s == '*' => {
                let mut nums = vec![];
                for c in grid.get_adjacent_coords(coord, true) {
                    let c = match c {
                        Some(c) if visited_coords.get(&c).is_some() => continue,
                        Some(c) => c,
                        None => continue,
                    };

                    let point = grid.map.get(&c).unwrap();

                    match point {
                        Point::Value(_) => {
                            let whole_num_info = get_whole_num(&c, &grid);

                            whole_num_info.iter().for_each(|(c, _)| {
                                visited_coords.insert(*c);
                            });

                            let whole_num = whole_num_info
                                .iter()
                                .map(|(_, val)| val)
                                .fold(String::new(), |acc, x| acc + x.to_string().as_str())
                                .parse::<u32>()
                                .unwrap();

                            nums.push(whole_num);
                        }
                        _ => continue,
                    }
                }

                if nums.len() == 2 {
                    total_gear_ratio += nums[0] * nums[1];
                }
            }
            _ => continue,
        }
    }
    Some(total_gear_ratio)
}
