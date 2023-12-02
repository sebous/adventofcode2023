advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().filter(|char| char.is_digit(10));
            let first = chars.next();
            let last = chars.last().or(first);
            let num = [first, last]
                .into_iter()
                .filter_map(|v| v)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            num
        })
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let str_digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let res: u32 = input
        .lines()
        .map(|line| {
            let mut digits = vec![];

            let mut line_iter = (0..line.len()).into_iter();

            'outer: while let Some(i) = line_iter.next() {
                let num = line[i..i + 1].parse::<u8>();
                if num.is_ok() {
                    digits.push(num.unwrap());
                    continue;
                }

                for str_digit in &str_digits {
                    if i + str_digit.len() > line.len() {
                        continue;
                    }
                    if &line[i..i + str_digit.len()] == *str_digit {
                        let digit = str_digits.iter().position(|x| x == str_digit).unwrap() + 1;
                        digits.push(digit as u8);
                        line_iter.nth(str_digit.len() - 3);
                        continue 'outer;
                    }
                }
            }

            let first = digits.first().unwrap();
            let last = digits.last().unwrap_or(first);

            let str = format!("{}{}", first, last);
            str.parse::<u32>().unwrap()
        })
        .sum();

    Some(res)
}
