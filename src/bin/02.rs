use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cube {
    BLUE,
    RED,
    GREEN,
}

type CubeConf = (u32, Cube);
type ParsedData = Vec<Vec<Vec<CubeConf>>>;

fn parse_input(input: &str) -> ParsedData {
    let mut data = vec![];
    for line in input.lines() {
        let (_game_info, turns) = line.split_once(':').unwrap();

        let turns = turns
            .split("; ")
            .map(|segment| {
                let x = segment
                    .trim()
                    .split(", ")
                    .map(|cubes_info| {
                        let cubes_info = cubes_info.trim();
                        let (num, color) = cubes_info.split_once(' ').unwrap();

                        let cube = match color {
                            "blue" => Cube::BLUE,
                            "red" => Cube::RED,
                            "green" => Cube::GREEN,
                            _ => unimplemented!(),
                        };
                        (num.parse::<u32>().unwrap(), cube)
                    })
                    .collect_vec();
                x
            })
            .collect_vec();
        data.push(turns);
    }
    data
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_input(input);
    let cube_limits = HashMap::from([(Cube::RED, 12), (Cube::GREEN, 13), (Cube::BLUE, 14)]);
    let mut impossible_game_ids = vec![];

    'game_loop: for (i, game) in data.iter().enumerate() {
        for segment in game {
            for (num, cube) in segment {
                if num > cube_limits.get(cube).unwrap() {
                    impossible_game_ids.push(i + 1);
                    continue 'game_loop;
                }
            }
        }
    }

    let res: usize = (1..=data.len())
        .filter(|x| !impossible_game_ids.contains(x))
        .sum();

    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse_input(input);

    let result = data
        .iter()
        .map(|game| {
            let mut game_cube_mins =
                HashMap::from([(Cube::RED, 0), (Cube::GREEN, 0), (Cube::BLUE, 0)]);
            for segment in game {
                for (num, cube) in segment {
                    if let Some(color_max_cnt) = game_cube_mins.get_mut(cube) {
                        if num > color_max_cnt {
                            *color_max_cnt = *num;
                        }
                    }
                }
            }
            game_cube_mins.values().fold(1, |acc, x| acc * x)
        })
        .sum::<u32>();

    Some(result)
}
