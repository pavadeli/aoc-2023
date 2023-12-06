use common::{boilerplate, to_usize};

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_whitespace()
        .map(to_usize);
    let records = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_whitespace()
        .map(to_usize);
    times
        .zip(records)
        .map(|(time, record)| nr_of_winning_options(time, record))
        .product()
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = to_usize(
        lines
            .next()
            .unwrap()
            .strip_prefix("Time:")
            .unwrap()
            .replace(' ', ""),
    );
    let record = to_usize(
        lines
            .next()
            .unwrap()
            .strip_prefix("Distance:")
            .unwrap()
            .replace(' ', ""),
    );
    nr_of_winning_options(time, record)
}

fn nr_of_winning_options(time: usize, record: usize) -> usize {
    // distance(x) = (time - x) * x
    // surplus(x) = distance(x) - record = -x^2 + time*x - record
    // surplus(x) = 0
    //          ->     x = 1/2 (time - sqrt(time^2 - 4 record))
    //          ->     x = 1/2 (sqrt(time^2 - 4 record) + time)
    let time = time as f64;
    let record = record as f64;
    let x1 = (time - (time * time - 4.0 * record).sqrt()) / 2.0;
    let x2 = ((time * time - 4.0 * record).sqrt() + time) / 2.0;
    let x1 = if x1.fract() == 0.0 {
        x1 as usize + 1
    } else {
        x1.ceil() as usize
    };
    let x2 = if x2.fract() == 0.0 {
        x2 as usize - 1
    } else {
        x2.floor() as usize
    };
    x2 - x1 + 1
}

boilerplate! {
    part1 => { test -> 288, real -> 128700 }
    part2 => { test -> 71503, real -> 39594072 }
}
