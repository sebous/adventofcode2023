use std::collections::HashMap;

use advent_of_code::grid::{Coord, Grid};
use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(PartialEq, Eq)]
enum Tile {
    Pipe(char),
    Ground,
    Start,
}

const PIPE_CHARS: [char; 6] = ['J', 'F', '7', 'L', '|', '-'];

fn parse(input: &str) -> Grid<Tile> {
    let mut map = HashMap::new();
    let height = input.lines().count();
    let mut width = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, point) in line.chars().enumerate() {
            map.insert(
                (x, y),
                match point {
                    'S' => Tile::Start,
                    char if PIPE_CHARS.iter().contains(&char) => Tile::Pipe(char),
                    '.' => Tile::Ground,
                    _ => unimplemented!(),
                },
            );
        }
        if width == 0 {
            width = line.len();
        }
    }

    Grid { map, height, width }
}

enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

fn lookup_next_position(
    direction: Direction,
    input_route: &Vec<Coord>,
    (pos_x, pos_y): &Coord,
) -> Option<Coord> {
    let lookup_pos = match direction {
        Direction::Left => {
            if *pos_x > 0 {
                return Some((pos_x - 1, *pos_y));
            }
            None
        }
        Direction::Top => {
            if *pos_y > 0 {
                return Some((*pos_x, pos_y - 1));
            }
            None
        }
        Direction::Right => Some((pos_x + 1, *pos_y)),
        Direction::Bottom => Some((*pos_x, pos_y + 1)),
    };

    if lookup_pos.is_some_and(|pos| input_route.contains(&pos)) {
        return None;
    }
    lookup_pos
}

fn find_possible_routes_recursively(
    grid: &Grid<Tile>,
    input_route: &Vec<Coord>,
    position: &Coord,
) -> Vec<Vec<Coord>> {
    // println!("position: {:?}, input route: {:?}", position, input_route);

    let mut input_route = input_route.clone();
    input_route.push(*position);

    let mut routes = vec![];

    // LEFT
    let left_c = lookup_next_position(Direction::Left, &input_route, position);
    if let Some(tile) = left_c.and_then(|pos| grid.map.get(&pos)) {
        match tile {
            Tile::Pipe(ch) if *ch == 'F' || *ch == 'L' || *ch == '-' => {
                routes.extend(find_possible_routes_recursively(
                    grid,
                    &input_route,
                    &left_c.unwrap(),
                ));
            }
            _ => {}
        }
    }

    // TOP
    let top_c = lookup_next_position(Direction::Top, &input_route, position);
    if let Some(tile) = top_c.and_then(|pos| grid.map.get(&pos)) {
        match tile {
            Tile::Pipe(ch) if *ch == 'F' || *ch == '7' || *ch == '|' => {
                routes.extend(find_possible_routes_recursively(
                    grid,
                    &input_route,
                    &top_c.unwrap(),
                ));
            }
            _ => {}
        }
    }

    // RIGHT
    let right_c = lookup_next_position(Direction::Right, &input_route, position);
    if let Some(tile) = right_c.and_then(|pos| grid.map.get(&pos)) {
        match tile {
            Tile::Pipe(ch) if *ch == '7' || *ch == 'J' || *ch == '-' => {
                routes.extend(find_possible_routes_recursively(
                    grid,
                    &input_route,
                    &right_c.unwrap(),
                ));
            }
            _ => {}
        }
    }

    // BOTTOM
    let bot_c = lookup_next_position(Direction::Bottom, &input_route, position);
    if let Some(tile) = bot_c.and_then(|pos| grid.map.get(&pos)) {
        match tile {
            Tile::Pipe(ch) if *ch == 'J' || *ch == 'L' || *ch == '|' => {
                routes.extend(find_possible_routes_recursively(
                    grid,
                    &input_route,
                    &bot_c.unwrap(),
                ));
            }
            _ => {}
        }
    }

    if routes.len() == 0 {
        routes.push(vec![*position])
    } else {
        for route in routes.iter_mut() {
            route.insert(0, *position);
        }
    }

    // println!("return: {:?}", routes);
    routes
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse(input);

    let (start_c, _) = grid
        .map
        .iter()
        .find(|(_, tile)| **tile == Tile::Start)
        .unwrap();

    let routes = find_possible_routes_recursively(&grid, &vec![], start_c);

    println!("{:?}", routes.iter().map(|r| r.len()).max());

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
