use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(4);

type Card = (HashSet<u32>, HashSet<u32>);

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            line.split(": ")
                .nth(1)
                .unwrap()
                .split(" | ")
                .map(|nums| {
                    nums.split_whitespace()
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<HashSet<u32>>()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_input(input);

    let total = cards.iter().fold(0, |total, (winning, actual)| {
        let count = (0..winning.intersection(actual).count()).fold(0, |card_total, i| match i {
            0 => 1,
            _ => card_total * 2,
        });
        total + count
    });

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut card_multipliers = HashMap::new();

    let cards = parse_input(input);

    let total_card_cnt = cards
        .iter()
        .enumerate()
        .fold(0u64, |total, (i, (winning, actual))| {
            let card_no = i as u32 + 1;
            let points_cnt = winning.intersection(actual).count() as u32;

            let card_mult = card_multipliers.get(&card_no).unwrap_or(&1).clone();

            (card_no + 1..=points_cnt + card_no).for_each(|number| {
                card_multipliers
                    .entry(number)
                    .and_modify(|mult| *mult += card_mult)
                    .or_insert(1 + card_mult);
            });

            total + card_mult
        });

    Some(total_card_cnt)
}
