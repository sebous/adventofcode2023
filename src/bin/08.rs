use std::collections::HashMap;

use itertools::Itertools;

use num::integer::lcm;

advent_of_code::solution!(8);
#[derive(Debug)]
enum Dir {
    Left,
    Right,
}

struct Instruction {
    directions: Vec<Dir>,
    nodes: HashMap<String, (String, String)>,
}

fn parse(input: &str) -> Instruction {
    let mut lines_iter = input.lines().enumerate();

    let directions = lines_iter
        .next()
        .unwrap()
        .1
        .chars()
        .map(|ch| match ch {
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => unimplemented!(),
        })
        .collect_vec();
    lines_iter.next();

    let mut nodes = HashMap::new();

    while let Some((_, line)) = lines_iter.next() {
        let (node, branches) = line.split_once(" = (").unwrap();
        let branches = branches.replace(")", "");
        let (left, right) = branches.split_once(", ").unwrap();

        nodes.insert(node.to_string(), (left.to_string(), right.to_string()));
    }

    Instruction { directions, nodes }
}

fn find_path_end(
    node_loc: &str,
    end_cond: impl Fn(&str) -> bool,
    instruction: &Instruction,
) -> (String, u32) {
    let mut i = 0;
    let mut next_node = String::from(node_loc);

    loop {
        let dir = instruction
            .directions
            .iter()
            .nth(i % instruction.directions.len())
            .unwrap();

        let node = instruction
            .nodes
            .get(&next_node)
            .expect("next node not found");

        if end_cond(&next_node) {
            break;
        }

        let next = match dir {
            Dir::Left => node.0.clone(),
            Dir::Right => node.1.clone(),
        };
        next_node = next;
        i += 1;
    }

    (next_node, i as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = parse(input);
    let (_, i) = find_path_end("AAA", |loc| loc == "ZZZ", &instructions);
    Some(i as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = parse(input);
    let next_nodes = instructions
        .nodes
        .keys()
        .filter(|loc| loc.contains("A"))
        .collect_vec();

    let results = next_nodes
        .iter()
        .map(|loc| find_path_end(loc, |x| x.contains("Z"), &instructions))
        .collect_vec();

    let least_multiple = results.iter().fold(0_u64, |acc, (_, num)| {
        if acc == 0 {
            *num as u64
        } else {
            lcm(acc, *num as u64)
        }
    });

    Some(least_multiple)
}
