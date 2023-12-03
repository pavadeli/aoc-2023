use common::boilerplate;
use std::collections::HashMap;

fn part1(input: &str) -> usize {
    // only 12 red cubes, 13 green cubes, and 14 blue cubes
    let q = HashMap::from([("red", 12usize), ("green", 13), ("blue", 14)]);
    input
        .lines()
        .filter_map(|line| {
            let line = line.strip_prefix("Game ").unwrap();
            let (id, rest) = line.split_once(": ").unwrap();
            let id: usize = id.parse().unwrap();
            let possible = rest.split("; ").all(|set| {
                set.split(", ").all(|combo| {
                    let (count, color) = combo.split_once(' ').unwrap();
                    let Some(allowed) = q.get(color) else {
                        return false;
                    };
                    *allowed >= count.parse().unwrap()
                })
            });
            possible.then_some(id)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| -> usize {
            let mut game: HashMap<&str, usize> = HashMap::new();
            let line = line.strip_prefix("Game ").unwrap();
            let (_, rest) = line.split_once(": ").unwrap();
            let sets = rest.split("; ");
            sets.for_each(|set| {
                set.split(", ").for_each(|combo| {
                    let (count, color) = combo.split_once(' ').unwrap();
                    let count = count.parse().unwrap();
                    game.entry(color)
                        .and_modify(|c| *c = (*c).max(count))
                        .or_insert(count);
                })
            });
            game.values().product()
        })
        .sum()
}

boilerplate! {
    part1 => { test -> 8, real -> 2727 }
    part2 => { test -> 2286, real -> 56580 }
}
