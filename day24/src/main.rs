use common::{boilerplate, Itertools, SS};
use std::ops::RangeBounds;
use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
};

#[derive(Clone, Debug)]
struct Hailstone {
    pos: (i64, i64, i64),
    dir: (i64, i64, i64),
}

impl Hailstone {
    fn parse(input: SS) -> Self {
        let (pos, dir) = input.split_once('@').unwrap();
        Self {
            pos: parse_triple(pos),
            dir: parse_triple(dir),
        }
    }

    fn intersecting_path_in_range(&self, other: &Self, range: &impl RangeBounds<f64>) -> bool {
        let Self {
            pos: (x1, y1, _),
            dir: (dx1, dy1, _),
        } = self;
        let Self {
            pos: (x2, y2, _),
            dir: (dx2, dy2, _),
        } = other;
        let [x1, y1, dx1, dy1, x2, y2, dx2, dy2] =
            [x1, y1, dx1, dy1, x2, y2, dx2, dy2].map(|n| *n as f64);
        let n1 = (dx2 * (y1 - y2) - dy2 * (x1 - x2)) / (dy2 * dx1 - dx2 * dy1);
        let n2 = (dx1 * (y2 - y1) - dy1 * (x2 - x1)) / (dy1 * dx2 - dx1 * dy2);
        let x = x1 + dx1 * n1;
        let y = y1 + dy1 * n1;
        n1 >= 0.0 && n2 >= 0.0 && range.contains(&x) && range.contains(&y)
    }
}

fn parse_triple(input: &str) -> (i64, i64, i64) {
    input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect_tuple()
        .unwrap()
}

fn part1(input: SS, min: isize, max: isize) -> usize {
    let hailstones = input.lines().map(Hailstone::parse).collect_vec();
    let range = min as f64..=max as f64;
    hailstones
        .into_iter()
        .tuple_combinations()
        .filter(|(a, b)| a.intersecting_path_in_range(b, &range))
        .count()
}

fn part2(input: SS) -> i64 {
    let hailstones = input.lines().map(Hailstone::parse).collect_vec();
    let ctx = Context::new(&Config::new());

    let rock_x = Int::new_const(&ctx, "rock_x");
    let rock_y = Int::new_const(&ctx, "rock_y");
    let rock_z = Int::new_const(&ctx, "rock_z");
    let rock_dx = Int::new_const(&ctx, "rock_dx");
    let rock_dy = Int::new_const(&ctx, "rock_dy");
    let rock_dz = Int::new_const(&ctx, "rock_dz");

    let solver = Solver::new(&ctx);
    for Hailstone {
        pos: (x, y, z),
        dir: (dx, dy, dz),
    } in hailstones
    {
        let [x, y, z, dx, dy, dz] = [x, y, z, dx, dy, dz].map(|v| Int::from_i64(&ctx, v));
        let n = Int::fresh_const(&ctx, "n");
        solver.assert(&(&x + &dx * &n)._eq(&(&rock_x + &rock_dx * &n)));
        solver.assert(&(&y + &dy * &n)._eq(&(&rock_y + &rock_dy * &n)));
        solver.assert(&(&z + &dz * &n)._eq(&(&rock_z + &rock_dz * &n)));
    }
    solver.check();
    let model = solver.get_model().unwrap();

    [&rock_x, &rock_y, &rock_z]
        .into_iter()
        .map(|v| model.get_const_interp(v).unwrap().as_i64().unwrap())
        .sum()
}

boilerplate! {
    part1 => { test(7, 27) -> 2, real(200_000_000_000_000, 400_000_000_000_000) -> 29142 }
    part2 => { test -> 47, real -> 848947587263033 }
}
