use common::{boilerplate, Itertools, SS};
use pathfinding::{
    directed::astar::astar,
    matrix::{
        directions::{DIRECTIONS_4, E, S},
        Matrix,
    },
};

type Dir = (isize, isize);

type Pos = (usize, usize);

fn part1(input: SS) -> usize {
    go::<3, 1>(input)
}

fn opposite(dir: Dir) -> Dir {
    (-dir.0, -dir.1)
}

fn part2(input: SS) -> usize {
    go::<10, 4>(input)
}

fn go<const MAX_STREAK: usize, const MIN_STREAK: usize>(input: SS) -> usize {
    let matrix = &Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().map(|c| c as usize - '0' as usize)),
    )
    .unwrap();
    let start: ([Dir; MAX_STREAK], Pos) = ([(0, 0); MAX_STREAK], (0, 0));
    let (_, cost) = astar(
        &start,
        |&(dirs, pos)| {
            let mut new_dirs = Vec::with_capacity(3);
            let last_dir = dirs[MAX_STREAK - 1];
            if last_dir == (0, 0) {
                new_dirs.extend([E, S]);
            } else if dirs.iter().all_equal() {
                new_dirs.extend(
                    DIRECTIONS_4
                        .iter()
                        .copied()
                        .filter(|&d| d != last_dir && d != opposite(last_dir)),
                )
            } else if dirs[MAX_STREAK - MIN_STREAK..].iter().all_equal() {
                new_dirs.extend(
                    DIRECTIONS_4
                        .iter()
                        .copied()
                        .filter(|&d| d != opposite(last_dir)),
                );
            } else {
                new_dirs.push(last_dir);
            }
            new_dirs.into_iter().flat_map(move |dir| {
                let next_pos = matrix.move_in_direction(pos, dir)?;
                let mut dirs = dirs;
                dirs.rotate_left(1);
                dirs[MAX_STREAK - 1] = dir;
                Some(((dirs, next_pos), matrix[next_pos]))
            })
        },
        |&(_, pos)| matrix.rows.abs_diff(pos.0) + matrix.columns.abs_diff(pos.1) - 2,
        |&(dirs, pos)| {
            pos == (matrix.rows - 1, matrix.columns - 1)
                && dirs[MAX_STREAK - MIN_STREAK..].iter().all_equal()
        },
    )
    .unwrap();
    cost
}

boilerplate! {
    part1 => { test -> 102, real -> 724 }
    part2 => { test -> 94, test2 -> 71, real -> 877 }
}
