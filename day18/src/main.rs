use common::{boilerplate, Itertools, SS};
use geo::{Area, EuclideanLength, Polygon};

fn part1(input: SS) -> usize {
    go(parse_input(input).map(|(dir, amount, _)| {
        let amount = amount.parse().unwrap();
        match dir {
            "R" => (amount, 0.0),
            "D" => (0.0, amount),
            "L" => (-amount, 0.0),
            "U" => (0.0, -amount),
            _ => unreachable!(),
        }
    }))
}

fn part2(input: SS) -> usize {
    go(parse_input(input).map(|(_, _, color)| {
        let (amount, dir) = color.split_at(color.len() - 1);
        let amount = usize::from_str_radix(amount, 16).unwrap() as f64;
        match dir {
            "0" => (amount, 0.0),
            "1" => (0.0, amount),
            "2" => (-amount, 0.0),
            "3" => (0.0, -amount),
            _ => unreachable!(),
        }
    }))
}

fn parse_input(input: SS) -> impl Iterator<Item = (SS, SS, SS)> {
    input.lines().map(|line| {
        let (dir, amount, color) = line.split_whitespace().collect_tuple().unwrap();
        let color = color.trim_matches(&['(', '#', ')'][..]);
        (dir, amount, color)
    })
}

fn go(moves: impl Iterator<Item = (f64, f64)>) -> usize {
    let vertices = moves
        .scan((0f64, 0f64), |pos, m| {
            pos.0 += m.0;
            pos.1 += m.1;
            Some(*pos)
        })
        .collect_vec();

    let polygon = Polygon::new(vertices.into(), vec![]);
    polygon.unsigned_area() as usize + (polygon.exterior().euclidean_length() / 2.0) as usize + 1
}

boilerplate! {
    part1 => { test -> 62, real -> 70253 }
    part2 => { test -> 952408144115, real -> 131265059885080 }
}
