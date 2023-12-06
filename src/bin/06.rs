use itertools::Itertools;

advent_of_code::solution!(6);

fn parse(input: &str) -> Vec<(u64, u64)> {
    let a = input
        .lines()
        .map(|line| {
            line.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    a[0].clone()
        .into_iter()
        .zip(a[1].clone().into_iter())
        .collect_vec()
}

fn solve_races(races: &Vec<(u64, u64)>) -> u64 {
    let mut possibilities = 1;

    for (race_t, distance) in races {
        let mut lowest = None;
        let mut highest = None;

        for charge_t in 1..=*race_t {
            if (race_t - charge_t) * charge_t > *distance {
                lowest = Some(charge_t);
                break;
            }
        }

        for charge_t in (1..=*race_t).rev() {
            if (race_t - charge_t) * charge_t > *distance {
                highest = Some(charge_t);
                break;
            }
        }

        if highest.is_some() && lowest.is_some() {
            possibilities *= highest.unwrap() - lowest.unwrap() + 1
        }
    }

    possibilities
}

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse(input);
    let possibilities = solve_races(&races);
    Some(possibilities)
}

pub fn part_two(input: &str) -> Option<u64> {
    let races = parse(input);
    let updated_races = races
        .iter()
        .fold(("".to_string(), "".to_string()), |mut acc, (a, b)| {
            acc.0.push_str(a.to_string().as_str());
            acc.1.push_str(b.to_string().as_str());
            acc
        });

    let possibilities = solve_races(&vec![(
        updated_races.0.parse::<u64>().unwrap(),
        updated_races.1.parse::<u64>().unwrap(),
    )]);
    Some(possibilities)
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
