use common::{boilerplate, Itertools, SS};
use pathfinding::matrix::Matrix;

fn part1(input: SS) -> usize {
    let lines = input.lines().collect_vec();
    let patterns = get_patterns(&lines);

    patterns
        .map(|(vertical, horizontal)| analyze(&horizontal, 0) + analyze(&vertical, 0) * 100)
        .sum()
}

fn get_patterns<'a>(lines: &'a [&str]) -> impl Iterator<Item = (Vec<usize>, Vec<usize>)> + 'a {
    lines.split(|line| line.is_empty()).map(|lines| {
        let matrix = Matrix::from_rows(lines.iter().map(|line| line.chars())).unwrap();
        (to_bits(&matrix), to_bits(&matrix.transposed()))
    })
}

fn to_bits(input: &Matrix<char>) -> Vec<usize> {
    input
        .iter()
        .map(|pattern| {
            pattern.iter().fold(0, |a, c| {
                (a << 1)
                    + match c {
                        '#' => 1,
                        '.' => 0,
                        _ => panic!("invalid input {c}"),
                    }
            })
        })
        .collect()
}

fn part2(input: SS) -> usize {
    let lines = input.lines().collect_vec();
    let patterns = get_patterns(&lines);

    patterns
        .map(|(vertical, horizontal)| analyze(&horizontal, 1) + analyze(&vertical, 1) * 100)
        .sum()
}

fn analyze(input: &[usize], hamming_dist: u32) -> usize {
    'outer: for i in 1..input.len() {
        let mut dist = 0;
        for (a, b) in input[0..i].iter().rev().zip(&input[i..]) {
            dist += (a ^ b).count_ones();
            if dist > hamming_dist {
                continue 'outer;
            }
        }
        if dist == hamming_dist {
            return i;
        }
    }
    0
}

boilerplate! {
    part1 => { test -> 405, real -> 35538 }
    part2 => { test -> 400, real -> 30442 }
}
