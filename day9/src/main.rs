use common::{boilerplate, to_isize, Itertools, SS};
use std::iter::from_fn;

fn part1(input: SS) -> isize {
    input.lines().map(eval_sequence1).sum()
}

fn eval_sequence1(input: SS) -> isize {
    derivatives(input, |n| *n.last().unwrap()).sum()
}

fn part2(input: SS) -> isize {
    input.lines().map(eval_sequence2).sum()
}

fn eval_sequence2(input: SS) -> isize {
    derivatives(input, |n| n[0])
        .collect_vec()
        .into_iter()
        .rev()
        .reduce(|a, b| b - a)
        .unwrap()
}

fn derivatives(
    input: SS,
    mut select: impl FnMut(&[isize]) -> isize,
) -> impl Iterator<Item = isize> {
    let mut numbers = input.split_whitespace().map(to_isize).collect_vec();
    from_fn(move || {
        if numbers.iter().all(|v| *v == 0) {
            return None;
        }
        let result = select(&numbers);
        numbers = numbers.iter().tuple_windows().map(|(a, b)| b - a).collect();
        Some(result)
    })
}

boilerplate! {
    part1 => { test -> 114, real -> 1980437560 }
    part2 => { test -> 2, real -> 977 }
}
