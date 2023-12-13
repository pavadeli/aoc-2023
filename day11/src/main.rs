use common::{boilerplate, Itertools, SS};

fn part1(input: SS) -> usize {
    go(input, 2)
}

fn part2(input: SS, factor: usize) -> usize {
    go(input, factor)
}

fn go(input: SS, factor: usize) -> usize {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, ch)| (ch == '#').then_some((x, y)))
        })
        .collect_vec();
    let col_table = expansion_table(&galaxies, |p| p.0, factor);
    let row_table = expansion_table(&galaxies, |p| p.1, factor);

    galaxies
        .into_iter()
        .map(|(x, y)| (col_table[x], row_table[y]))
        .tuple_combinations()
        .map(|(a, b)| dist(a, b))
        .sum()
}

type Point = (usize, usize);

fn dist((xa, ya): Point, (xb, yb): Point) -> usize {
    xa.abs_diff(xb) + ya.abs_diff(yb)
}

fn expansion_table(input: &[Point], which: impl Fn(&Point) -> usize, factor: usize) -> Vec<usize> {
    let ordered = input.iter().map(which).sorted().dedup().collect_vec();
    let max = *ordered.last().unwrap();
    ordered
        .into_iter()
        .merge_join_by(0..=max, Ord::cmp)
        .scan(0, |offset, e| {
            if e.is_right() {
                *offset += factor - 1;
            }
            Some(e.into_right() + *offset)
        })
        .collect_vec()
}

boilerplate! {
    part1 => { test1 -> 374, real -> 10494813 }
    part2 => { test1(10) -> 1030, test1(100) -> 8410, real(1_000_000) -> 840988812853  }
}
