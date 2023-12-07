use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

advent_of_code::solution!(7);

const CARD_ORDER: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARD_ORDER_P2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Handtype {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfaKind,
    FullHouse,
    FourOfaKind,
    FiveOfaKind,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Version {
    One,
    Two,
}

#[derive(Clone, Debug, Eq)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    hand_type: Handtype,
    ver: Version,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                let order = if self.ver == Version::One {
                    CARD_ORDER
                } else {
                    CARD_ORDER_P2
                };

                for i in 0..5 {
                    if order.iter().position(|x| *x == self.cards[i])
                        > order.iter().position(|x| *x == other.cards[i])
                    {
                        return Ordering::Greater;
                    } else if order.iter().position(|x| *x == self.cards[i])
                        < order.iter().position(|x| *x == other.cards[i])
                    {
                        return Ordering::Less;
                    } else if i == 4
                        && order.iter().position(|x| *x == self.cards[i])
                            == order.iter().position(|x| *x == other.cards[i])
                    {
                        return Ordering::Equal;
                    } else {
                        continue;
                    }
                }
                unimplemented!()
            }
            x => x,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

fn parse(input: &str) -> Vec<Hand> {
    let mut hands = vec![];
    for line in input.lines() {
        let (hand, bid) = line.split_whitespace().next_tuple().unwrap();
        let cards = hand.chars().collect_vec();
        let bid = bid.parse().unwrap();
        let mut hand_type = Handtype::HighCard;

        let mut card_counts = HashMap::new();
        cards.iter().for_each(|card| {
            card_counts.entry(card).and_modify(|x| *x += 1).or_insert(1);
        });

        if card_counts.values().any(|cnt| *cnt == 5) {
            hand_type = Handtype::FiveOfaKind;
        } else if card_counts.values().any(|cnt| *cnt == 4) {
            hand_type = Handtype::FourOfaKind;
        } else if card_counts.values().any(|cnt| *cnt == 3)
            && card_counts.values().any(|cnt| *cnt == 2)
        {
            hand_type = Handtype::FullHouse;
        } else if card_counts.values().any(|cnt| *cnt == 3) {
            hand_type = Handtype::ThreeOfaKind;
        } else if card_counts.values().filter(|cnt| **cnt == 2).count() == 2 {
            hand_type = Handtype::TwoPair;
        } else if card_counts.values().any(|cnt| *cnt == 2) {
            hand_type = Handtype::OnePair;
        }

        hands.push(Hand {
            bid,
            cards,
            hand_type,
            ver: Version::One,
        })
    }

    hands
}

pub fn part_one(input: &str) -> Option<u32> {
    let hands = parse(input);
    let score = hands
        .iter()
        .sorted()
        .enumerate()
        .fold(0, |score, (i, hand)| score + (i as u32 + 1) * hand.bid);

    Some(score)
}

fn is_full_house(card_counts: &HashMap<&char, u32>, jokers_cnt: u32) -> bool {
    let three_item = card_counts.iter().find(|(_, cnt)| *cnt + jokers_cnt == 3);
    let three_item = if three_item.is_none() && jokers_cnt == 3 {
        Some((&&'J', &3))
    } else {
        three_item
    };

    return match three_item {
        Some((three_card, three_cnt)) => {
            let jokers_remaining = if *three_cnt >= 3 {
                0
            } else {
                jokers_cnt - (3 - three_cnt)
            };
            card_counts
                .iter()
                .filter(|(card, _)| *card != three_card)
                .any(|(_, cnt)| (*cnt as i32 + jokers_remaining as i32) == 2)
                || jokers_remaining == 2
        }
        None => false,
    };
}

fn are_two_pairs(card_counts: &HashMap<&char, u32>, jokers_cnt: u32) -> bool {
    let first_pair = card_counts.iter().find(|(_, cnt)| *cnt + jokers_cnt == 2);
    let first_pair = if first_pair.is_none() && jokers_cnt == 2 {
        Some((&&'J', &2))
    } else {
        first_pair
    };

    match first_pair {
        Some((first_card, first_cnt)) => {
            let jokers_remaining = if *first_cnt >= 2 {
                0
            } else {
                jokers_cnt - (2 - first_cnt)
            };

            card_counts
                .iter()
                .filter(|(card, _)| *card != first_card)
                .any(|(_, cnt)| (*cnt as i32 + jokers_remaining as i32) == 2)
                || jokers_remaining == 2
        }
        None => false,
    }
}

fn parse_two(input: &str) -> Vec<Hand> {
    let mut hands = vec![];
    for line in input.lines() {
        let (hand, bid) = line.split_whitespace().next_tuple().unwrap();
        let cards = hand.chars().collect_vec();
        let bid = bid.parse().unwrap();
        let mut hand_type = Handtype::HighCard;

        let jokers_cnt = cards.iter().filter(|x| **x == 'J').count() as u32;

        let mut card_counts = HashMap::new();
        cards.iter().filter(|card| **card != 'J').for_each(|card| {
            card_counts.entry(card).and_modify(|x| *x += 1).or_insert(1);
        });

        if card_counts.values().any(|cnt| *cnt + jokers_cnt == 5) || jokers_cnt == 5 {
            hand_type = Handtype::FiveOfaKind;
        } else if card_counts.values().any(|cnt| *cnt + jokers_cnt == 4) || jokers_cnt == 4 {
            hand_type = Handtype::FourOfaKind;
        } else if is_full_house(&card_counts, jokers_cnt) {
            hand_type = Handtype::FullHouse;
        } else if card_counts.values().any(|cnt| *cnt + jokers_cnt == 3) {
            hand_type = Handtype::ThreeOfaKind;
        } else if are_two_pairs(&card_counts, jokers_cnt) {
            hand_type = Handtype::TwoPair;
        } else if card_counts.values().any(|cnt| *cnt + jokers_cnt == 2) {
            hand_type = Handtype::OnePair;
        }

        hands.push(Hand {
            bid,
            cards,
            hand_type,
            ver: Version::Two,
        })
    }

    hands
}

pub fn part_two(input: &str) -> Option<u64> {
    let hands = parse_two(input);
    let score: u64 = hands
        .iter()
        .sorted()
        .enumerate()
        .fold(0, |score, (i, hand)| {
            score + (i as u64 + 1) * hand.bid as u64
        });

    Some(score)
}
