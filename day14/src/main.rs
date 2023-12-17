use common::{boilerplate, SS};
use pathfinding::{directed::cycle_detection, matrix::Matrix};

fn part1(input: SS) -> usize {
    let mut matrix = Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap();
    for col in 0..matrix.columns {
        tilt(&mut matrix, (0, col), (1, 0));
    }
    matrix
        .items()
        .map(|((row, _), c)| if *c == 'O' { matrix.rows - row } else { 0 })
        .sum()
}

fn tilt(matrix: &mut Matrix<char>, start: (usize, usize), direction: (isize, isize)) {
    let mut free_spot = start;
    for cell in matrix.in_direction(start, direction) {
        while matrix[free_spot] != '.' && free_spot != cell {
            free_spot = matrix.move_in_direction(free_spot, direction).unwrap();
        }
        match matrix[cell] {
            'O' => matrix.swap(cell, free_spot),
            '#' => free_spot = cell,
            _ => (),
        }
    }
}

fn part2(input: SS) -> usize {
    let matrix = Matrix::from_rows(input.lines().map(|line| line.chars())).unwrap();
    let (cycle_size, mut matrix, cycle_start) = cycle_detection::brent(matrix, one_cycle);
    let cycles_still_needed = (1_000_000_000 - cycle_start) % cycle_size;

    for _ in 0..cycles_still_needed {
        matrix = one_cycle(matrix);
    }

    matrix
        .items()
        .map(|((row, _), c)| if *c == 'O' { matrix.rows - row } else { 0 })
        .sum()
}

fn one_cycle(mut matrix: Matrix<char>) -> Matrix<char> {
    for col in 0..matrix.columns {
        tilt(&mut matrix, (0, col), (1, 0));
    }
    for row in 0..matrix.rows {
        tilt(&mut matrix, (row, 0), (0, 1));
    }
    for col in 0..matrix.columns {
        let rows = matrix.rows;
        tilt(&mut matrix, (rows - 1, col), (-1, 0));
    }
    for row in 0..matrix.rows {
        let columns = matrix.columns;
        tilt(&mut matrix, (row, columns - 1), (0, -1));
    }
    matrix
}

boilerplate! {
    part1 => { test -> 136, real -> 113486 }
    part2 => { test -> 64, real -> 104409 }
}
