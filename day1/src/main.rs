use common::boilerplate;
use regex::Regex;

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| -> usize {
            let mut digits = line.chars().filter(char::is_ascii_digit);
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            format!("{first}{last}").parse().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let re = Regex::new("one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    input
        .lines()
        .map(|line| -> usize {
            let first = re.find(line).unwrap();
            let last = (0..line.len())
                .rev()
                .find_map(|s| re.find(&line[s..]))
                .unwrap();
            let [first, last] = [first, last].map(|m| match m.as_str() {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                r => r,
            });
            format!("{first}{last}").parse().unwrap()
        })
        .sum()
}

boilerplate! {
    part1 => { test -> 142, real -> 56506 }
    part2 => { test2 -> 281, real -> 56017 }
}
