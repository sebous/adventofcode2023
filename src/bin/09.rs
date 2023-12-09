use itertools::Itertools;

advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn fill_dataset(values: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut lines = vec![values.clone()];
    while !lines.last().unwrap().iter().all(|x| *x == 0) {
        let last_ln = lines.last().unwrap();
        let ln = last_ln.windows(2).map(|x| x[1] - x[0]).collect_vec();
        lines.push(ln);
    }
    lines
}

fn extrapolate(values: &Vec<i64>) -> i64 {
    let mut lines = fill_dataset(values);

    let mut last = 0;
    for ln in lines.iter_mut().rev() {
        let new_last = ln.last().unwrap() + last;
        ln.push(new_last);
        last = new_last;
    }

    lines.first().unwrap().last().unwrap().clone()
}

pub fn part_one(input: &str) -> Option<i64> {
    let values = parse(input);
    let res = values.iter().map(|x| extrapolate(x)).sum::<i64>();
    Some(res)
}

fn extrapolate_left(values: &Vec<i64>) -> i64 {
    let mut lines = fill_dataset(values);

    let mut last = 0;
    for ln in lines.iter_mut().rev() {
        let new_first = ln.first().unwrap() - last;
        ln.insert(0, new_first);
        last = new_first;
    }

    lines.first().unwrap().first().unwrap().clone()
}

pub fn part_two(input: &str) -> Option<i64> {
    let values = parse(input);
    let res = values.iter().map(|x| extrapolate_left(x)).sum::<i64>();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
