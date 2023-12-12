use common::{boilerplate, Itertools, SS};
use pathfinding::{
    directed::dijkstra::{build_path, dijkstra_all},
    matrix::directions,
};
use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut},
};

type Point = (usize, usize);
type Direction = (isize, isize);

fn part1(input: SS) -> usize {
    let matrix = Matrix::from_input(input);

    dijkstra_all(&matrix.start_point(), |&point| {
        matrix.reachable_from(point).map(|p| (p, 1))
    })
    .values()
    .map(|&(_, c)| c)
    .max()
    .unwrap() as usize
}

fn part2(input: SS) -> usize {
    let matrix = Matrix::from_input(input);
    let start_point = matrix.start_point();
    let parents = dijkstra_all(&start_point, |&point| {
        matrix.reachable_from(point).map(|p| (p, 1))
    });

    // Find the two ends of the cycle:
    let (&end_point_a, (parent_a, _cost_a)) = parents.iter().max_by_key(|e| e.1 .1).unwrap();
    let (&end_point_b, (_parent_b, _cost_b)) = parents
        .iter()
        .filter(|&e| e.0 != &end_point_a && e.0 != parent_a)
        .max_by_key(|e| e.1 .1)
        .unwrap();

    // Make sure the ends connect:
    assert!(matrix.reachable_from(end_point_a).contains(&end_point_b));
    assert!(matrix.reachable_from(end_point_b).contains(&end_point_a));

    let mut path = build_path(&end_point_a, &parents);
    path.extend(build_path(&end_point_b, &parents).into_iter().rev());

    // Sanity check:
    let mut counts = path.iter().counts();
    assert_eq!(counts.remove(&start_point), Some(2));
    assert_eq!(counts.values().all_equal_value(), Ok(&1));

    let mut canvas = Matrix::new(matrix.rows, matrix.columns);
    for &p in &path {
        canvas[p].0 = matrix[p].0; //'█';
    }

    for (&from, &to) in path.iter().tuple_windows() {
        canvas[from].0 = '*';
        let dir = direction(from, to);

        for point in matrix[from]
            .right_side(dir)
            .iter()
            .filter_map(|&p| canvas.move_in_direction(from, p))
            .collect_vec()
        {
            assert_ne!(canvas[point].0, 'l');
            canvas.paint(point, 'r')
        }
        for point in matrix[from]
            .left_side(dir)
            .iter()
            .filter_map(|&p| canvas.move_in_direction(from, p))
            .collect_vec()
        {
            assert_ne!(canvas[point].0, 'r');
            canvas.paint(point, 'l')
        }
    }

    let which = match canvas[(0, 0)].0 {
        'l' => 'r',
        'r' => 'l',
        _ => panic!(),
    };

    canvas.values().filter(|v| v.0 == which).count()
}

#[derive(Debug)]
struct Matrix(pathfinding::matrix::Matrix<Tile>);

impl Deref for Matrix {
    type Target = pathfinding::matrix::Matrix<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for cell in row {
                cell.fmt(f)?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Matrix {
    fn new(rows: usize, columns: usize) -> Self {
        Self(pathfinding::matrix::Matrix::new(rows, columns, Tile('.')))
    }

    fn from_input(input: &str) -> Self {
        Self(
            pathfinding::matrix::Matrix::from_rows(
                input.lines().map(|line| line.chars().map(Tile)),
            )
            .unwrap(),
        )
    }

    fn reachable_from(&self, point: Point) -> impl Iterator<Item = Point> + '_ {
        self[point]
            .possible_directions()
            .iter()
            .filter_map(move |&dir| {
                let other = self.move_in_direction(point, dir)?;
                self[other]
                    .possible_directions()
                    .contains(&inverse_direction(dir))
                    .then_some(other)
            })
    }

    fn start_point(&self) -> Point {
        self.items()
            .find_map(|(point, tile)| tile.is_start().then_some(point))
            .unwrap()
    }

    fn paint(&mut self, point: Point, ch: char) {
        if self[point].0 != '.' {
            return;
        }
        for p in self.bfs_reachable(point, false, |p| self[p].0 == '.') {
            assert_eq!(self[p].0, '.');
            self[p].0 = ch;
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Tile(char);

impl Tile {
    fn possible_directions(&self) -> &[Direction] {
        use directions::*;
        match self.0 {
            // | is a vertical pipe connecting north and south.
            '|' => &[N, S],
            // - is a horizontal pipe connecting east and west.
            '-' => &[E, W],
            // L is a 90-degree bend connecting north and east.
            'L' => &[N, E],
            // J is a 90-degree bend connecting north and west.
            'J' => &[N, W],
            // 7 is a 90-degree bend connecting south and west.
            '7' => &[S, W],
            // F is a 90-degree bend connecting south and east.
            'F' => &[E, S],
            // . is ground; there is no pipe in this tile.
            '.' => &[],
            // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
            'S' => &DIRECTIONS_4,
            _ => panic!("Invalid char: {}", self.0),
        }
    }

    fn right_side(&self, dir: Direction) -> &[Direction] {
        use directions::*;
        match (self.0, dir) {
            // | is a vertical pipe connecting north and south.
            ('|', N) => &[NE, E, SE],
            ('|', S) => &[SW, W, NW],
            // - is a horizontal pipe connecting east and west.
            ('-', E) => &[SE, S, SW],
            ('-', W) => &[NW, N, NE],
            // L is a 90-degree bend connecting north and east.
            ('L', N) => &[NE],
            ('L', E) => &[SE, S, SW, W, NW],
            // J is a 90-degree bend connecting north and west.
            ('J', N) => &[NE, E, SE, S, SW],
            ('J', W) => &[NW],
            // 7 is a 90-degree bend connecting south and west.
            ('7', S) => &[SW],
            ('7', W) => &[NW, N, NE, E, SE],
            // F is a 90-degree bend connecting south and east.
            ('F', E) => &[SE],
            ('F', S) => &[SW, W, NW, N, NE],
            // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
            ('S', _) => &[],
            (c, dir) => panic!("Invalid char: {c} and dir: {dir:?}"),
        }
    }

    fn left_side(&self, dir: Direction) -> &[Direction] {
        use directions::*;
        match (self.0, dir) {
            // | is a vertical pipe connecting north and south.
            ('|', N) => &[SW, W, NW],
            ('|', S) => &[NE, E, SE],
            // - is a horizontal pipe connecting east and west.
            ('-', E) => &[NW, N, NE],
            ('-', W) => &[SE, S, SW],
            // L is a 90-degree bend connecting north and east.
            ('L', N) => &[SE, S, SW, W, NW],
            ('L', E) => &[NE],
            // J is a 90-degree bend connecting north and west.
            ('J', N) => &[NW],
            ('J', W) => &[NE, E, SE, S, SW],
            // 7 is a 90-degree bend connecting south and west.
            ('7', S) => &[NW, N, NE, E, SE],
            ('7', W) => &[SW],
            // F is a 90-degree bend connecting south and east.
            ('F', E) => &[SW, W, NW, N, NE],
            ('F', S) => &[SE],
            // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
            ('S', _) => &[],
            (c, dir) => panic!("Invalid char: {c} and dir: {dir:?}"),
        }
    }

    fn is_start(&self) -> bool {
        self.0 == 'S'
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

fn inverse_direction((x, y): Direction) -> Direction {
    (-x, -y)
}

fn direction(from: Point, to: Point) -> Direction {
    (
        to.0 as isize - from.0 as isize,
        to.1 as isize - from.1 as isize,
    )
}

boilerplate! {
    part1 => { test1 -> 4, test2 -> 8, real -> 6831 }
    part2 => { test3 -> 4, test4 -> 8, test5 -> 10, test6 -> 4, real -> 305 }
}
