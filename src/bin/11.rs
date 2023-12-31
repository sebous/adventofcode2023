use std::collections::HashSet;

use advent_of_code::grid::Coord;
use itertools::Itertools;

advent_of_code::solution!(11);

fn parse(input: &str, multiplier: u32) -> HashSet<Coord> {
    let mut coord_set = HashSet::new();

    let input_2d = input
        .lines()
        .map(|ln| ln.chars().collect_vec())
        .collect_vec();

    let height = input_2d.len();
    let width = input_2d[0].len();

    let mut expand_columns = vec![];
    let mut expand_rows = vec![];

    for (y, line) in input.lines().enumerate() {
        if line.chars().all(|ch| ch == '.') {
            expand_rows.push(y);
        }
    }
    for x in 0..width {
        if (0..height).map(|y| input_2d[y][x]).all(|ch| ch == '.') {
            expand_columns.push(x);
        }
    }

    let multiplier = multiplier - 1;

    for y in 0..height {
        for x in 0..width {
            match input_2d[y][x] {
                '#' => {
                    let move_right = expand_columns.iter().filter(|a| **a < x).count();
                    let move_bot = expand_rows.iter().filter(|a| **a < y).count();

                    coord_set.insert((
                        x + move_right * multiplier as usize,
                        y + move_bot * multiplier as usize,
                    ));
                }
                _ => {}
            }
        }
    }

    coord_set
}

fn manhattan_dist((x1, y1): &Coord, (x2, y2): &Coord) -> u64 {
    let x = *x2 as isize - *x1 as isize;
    let y = *y2 as isize - *y1 as isize;
    (x.abs() + y.abs()) as u64
}

fn sum_distances(coords: &HashSet<Coord>) -> u64 {
    let unique_pairs = coords
        .iter()
        .permutations(2)
        .map(|vals| {
            vals.iter()
                .sorted()
                .cloned()
                .collect_tuple::<(&Coord, &Coord)>()
                .unwrap()
        })
        .unique()
        .collect_vec();

    unique_pairs
        .iter()
        .map(|(a, b)| manhattan_dist(a, b))
        .sum::<u64>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse(input, 2);
    Some(sum_distances(&coords))
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse(input, 1000000);
    Some(sum_distances(&coords))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }
}
