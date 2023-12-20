use common::{boilerplate, Itertools, SS};
use pathfinding::{
    directed::bfs::bfs_reach,
    matrix::{directions::*, Matrix},
};

fn part1(input: SS) -> usize {
    let matrix = &Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap();
    count_energized(matrix, ((0, 0), E))
}

fn count_energized(matrix: &Matrix<char>, start: ((usize, usize), (isize, isize))) -> usize {
    bfs_reach(start, |&(pos, dir)| {
        let dirs = match (matrix[pos], dir) {
            ('.', _) | ('-', E | W) | ('|', N | S) => vec![dir],
            ('|', E | W) => vec![S, N],
            ('-', N | S) => vec![E, W],
            ('\\', E) => vec![S],
            ('\\', S) => vec![E],
            ('\\', N) => vec![W],
            ('\\', W) => vec![N],
            ('/', E) => vec![N],
            ('/', N) => vec![E],
            ('/', S) => vec![W],
            ('/', W) => vec![S],
            _ => unreachable!(),
        };
        dirs.into_iter()
            .filter_map(move |dir| Some((matrix.move_in_direction(pos, dir)?, dir)))
    })
    .map(|(pos, _)| pos)
    .unique()
    .count()
}

fn part2(input: SS) -> usize {
    let matrix = &Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap();
    let mut starts = vec![];
    starts.extend((0..matrix.rows).map(|x| ((x, 0), E)));
    starts.extend((0..matrix.rows).map(|x| ((x, matrix.columns - 1), W)));
    starts.extend((0..matrix.columns).map(|y| ((0, y), S)));
    starts.extend((0..matrix.columns).map(|y| ((matrix.rows - 1, y), N)));
    starts
        .into_iter()
        .map(|start| count_energized(matrix, start))
        .max()
        .unwrap()
}

boilerplate! {
    part1 => { test -> 46, real -> 8034 }
    part2 => { test -> 51, real -> 8225 }
}
