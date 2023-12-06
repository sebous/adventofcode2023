use itertools::Itertools;

advent_of_code::solution!(5);

#[derive(Clone, Copy, Debug)]
struct Range {
    source: i64,
    dest: i64,
    size: i64,
}
#[derive(Clone, Debug)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}

struct Almanac {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

fn parse_input(input: &str) -> Almanac {
    let mut seeds = vec![];

    let mut maps = vec![];

    for line in input.lines() {
        if line.starts_with("seeds") {
            seeds = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect_vec();
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        if line.ends_with("map:") {
            let mut ln_iter = line.split_whitespace().next().unwrap().split("-");

            maps.push(Map {
                source: ln_iter.next().unwrap().to_string(),
                destination: ln_iter.nth(1).unwrap().to_string(),
                ranges: vec![],
            });
            continue;
        }

        if line.chars().next().unwrap_or('a').is_digit(10) {
            let mut nums_iter = line.split_whitespace().map(|n| n.parse::<i64>().unwrap());
            let (dest, src, size) = nums_iter.next_tuple().unwrap();

            if let Some(map) = maps.last_mut() {
                map.ranges.push(Range {
                    source: src,
                    dest,
                    size,
                })
            }
        }
    }

    Almanac { seeds, maps }
}

fn find_lowest_loc(maps: &Vec<Map>, seed_ranges: &Vec<i64>) -> Option<i64> {
    let mut lowest = None;

    for chunk in seed_ranges.iter().chunks(2).into_iter() {
        let (seed_start, size) = chunk.cloned().into_iter().next_tuple().unwrap();
        for seed in seed_start..seed_start + size {
            // println!("seed: {}", seed);
            let mut current = seed;
            'map_loop: for map in maps {
                // println!("source: {}, dest: {}", map.source, map.destination);
                for range in &map.ranges {
                    if current >= range.source && current <= (range.source + range.size) {
                        current = range.dest + current - range.source;
                        continue 'map_loop;
                    }
                }

                // println!("current: {}, next: {}", current, next.unwrap_or(current));
            }

            match lowest {
                Some(l) if l > current => {
                    lowest = Some(current);
                }
                None => lowest = Some(current),
                _ => continue,
            }
        }
    }

    // println!("{:?}", locations);
    lowest
}

pub fn part_one(input: &str) -> Option<i64> {
    let almanac = parse_input(input);
    println!(
        "{:?}",
        &almanac.seeds.iter().flat_map(|x| [*x, 1_i64]).collect_vec()
    );
    find_lowest_loc(
        &almanac.maps,
        &almanac.seeds.iter().flat_map(|x| [*x, 1_i64]).collect_vec(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let almanac = parse_input(input);
    find_lowest_loc(&almanac.maps, &almanac.seeds)
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
