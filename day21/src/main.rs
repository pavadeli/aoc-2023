use common::{boilerplate, Itertools, SS};
use pathfinding::grid::Grid;
use std::{collections::HashSet, mem::take};

fn part1(input: SS, steps: usize) -> usize {
    let coords = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, ch)| (ch != '#').then_some((x, y)))
        })
        .collect_vec();
    let grid = Grid::from_coordinates(&coords).unwrap();
    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, row)| Some((row.chars().position(|ch| ch == 'S')?, y)))
        .unwrap();

    let mut positions = HashSet::new();
    positions.insert(start);
    for _ in 0..steps {
        let old_positions = take(&mut positions);
        positions.extend(
            old_positions
                .into_iter()
                .flat_map(|pos| grid.neighbours(pos)),
        );
    }

    positions.len()
}

const STABLE_AFTER_ITERATION: usize = 1;

fn part2(input: SS) -> usize {
    let coords = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, ch)| (ch != '#').then_some((x, y)))
        })
        .collect_vec();
    let grid = Grid::from_coordinates(&coords).unwrap();
    let width = grid.width as isize;
    let height = grid.height as isize;
    let start = input
        .lines()
        .enumerate()
        .find_map(|(y, row)| Some((row.chars().position(|ch| ch == 'S')? as isize, y as isize)))
        .unwrap();

    // Important observations of input data:
    // - it is perfectly square
    // - S is in the exact middle
    // - no obstacles from S(tart) in the 4 directions N, E, S and W (besides the diamond shape
    //   if you squint your eyes). So every square to the N, E, S and W will be hit at the same
    //   time, every time, which should lead to an obvious pattern
    // Pattern should start at: (width - 1) / 2
    // Pattern length should be: width

    // Another nice property is:
    // - the required steps minus (width - 1) / 2 are a multitude of width, i.e.:
    // (26501365 - 65) % 131 == 0

    // positions for even and odd positions (an optimization)
    let mut positions = [HashSet::new(), HashSet::new()];
    let mut new_positions = HashSet::new();
    positions[0].insert(start);
    new_positions.insert(start);
    let mut deltas = vec![];
    let mut previous_count = 1;

    for i in 0.. {
        let phase = (i + 1) % 2;
        for (x, y) in take(&mut new_positions) {
            new_positions.extend(
                [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .into_iter()
                    .filter(|&(x, y)| {
                        grid.has_vertex((
                            (x.rem_euclid(width)) as usize,
                            (y.rem_euclid(height)) as usize,
                        )) && !positions[phase].contains(&(x, y))
                    }),
            );
        }
        positions[phase].extend(new_positions.iter().copied());
        if i < 64 || (i - 64) % 131 != 0 {
            continue;
        }

        let new_count = positions[phase].len();
        let new_delta = new_count - previous_count;
        previous_count = new_count;
        deltas.push(new_delta);
        if deltas.len() < 4 {
            continue;
        }
        let Ok(delta_per_round) = deltas
            .iter()
            .rev()
            .tuple_windows()
            .map(|(a, b)| a - b)
            .take(STABLE_AFTER_ITERATION)
            .all_equal_value()
        else {
            eprintln!("not stable yet: {deltas:?}");
            continue;
        };
        eprintln!("stable: {deltas:?}");
        dbg!(delta_per_round);

        assert_eq!((26501365 - i - 1) % 131, 0);
        let rounds_to_go = (26501365 - i - 1) / 131;
        dbg!(rounds_to_go);

        return get_projected_count(new_count, new_delta, rounds_to_go, delta_per_round);
    }
    unreachable!();
}

fn get_projected_count(
    base: usize,
    last_delta: usize,
    rounds_to_go: usize,
    delta_per_round: usize,
) -> usize {
    // rounds_to_go => positions
    // 0 => new_count
    // 1 => new_count + last + delta_per_round
    // 2 => new_count + last + delta_per_round + last + delta_per_round * 2
    // 3 => new_count + last + delta_per_round + last + delta_per_round * 2 + last + delta_per_round * 3
    // n => new_count + n * last + (n + 1) * n / 2 * delta_per_round
    base + rounds_to_go * last_delta + (rounds_to_go + 1) * rounds_to_go / 2 * delta_per_round
}

boilerplate! {
    part1 => { test(6) -> 16, real(64) -> 3671 }
    part2 => { real -> 609708004316870 }
}
