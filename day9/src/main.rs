use common::{boilerplate, to_isize, Itertools, SS};
use std::iter::from_fn;

fn part1(input: SS) -> isize {
    input.lines().map(eval_sequence1).sum()
}

fn eval_sequence1(input: SS) -> isize {
    let mut numbers = input.split_whitespace().map(to_isize).collect_vec();
    from_fn(|| {
        let result = numbers
            .iter()
            .any(|v| *v != 0)
            .then_some(*numbers.last().unwrap());
        numbers = numbers.iter().tuple_windows().map(|(a, b)| b - a).collect();
        result
    })
    .sum()
}

fn part2(input: SS) -> isize {
    input.lines().map(eval_sequence2).sum()
}

fn eval_sequence2(input: SS) -> isize {
    let mut numbers = input.split_whitespace().map(to_isize).collect_vec();
    from_fn(|| {
        let result = numbers
            .iter()
            .any(|v| *v != 0)
            .then_some(*numbers.first().unwrap());
        numbers = numbers.iter().tuple_windows().map(|(a, b)| b - a).collect();
        result
    })
    .collect_vec()
    .into_iter()
    .rev()
    .reduce(|a, b| b - a)
    .unwrap()
}

boilerplate! {
    part1 => { test -> 114, real -> 1980437560 }
    part2 => { test -> 2, real -> 977 }
}
