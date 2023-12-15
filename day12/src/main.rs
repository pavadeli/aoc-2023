use common::{boilerplate, repeat_n, to_usize, Itertools, SS};
use pathfinding::directed::count_paths::count_paths;
use rayon::prelude::*;

fn part1(input: SS) -> usize {
    input.lines().map(|l| count_arrangements(l, 1)).sum()
}

fn part2(input: SS) -> usize {
    let lines = input.lines().collect_vec();
    lines.par_iter().map(|l| count_arrangements(l, 5)).sum()
}

fn count_arrangements(input: SS, repeat: usize) -> usize {
    let (map, groups) = input.split_once(' ').unwrap();
    let map = repeat_n(map, repeat).join("?").chars().collect_vec();
    let groups = repeat_n(groups, repeat)
        .join(",")
        .split(',')
        .map(to_usize)
        .collect_vec();

    count_paths(
        (&map[..], &groups[..]),
        |&(map, groups)| {
            let mut next = vec![];
            match (map.first(), groups.first()) {
                (None, _) => (),
                (_, None) => {
                    if !map.contains(&'#') {
                        next.push((&[][..], &[][..]))
                    }
                }
                (Some('.'), _) => next.extend(assume_dot(map, groups)),
                (Some('#'), Some(&next_group)) => next.extend(assume_hash(map, groups, next_group)),
                (Some(_), Some(&next_group)) => {
                    next.extend(assume_dot(map, groups));
                    next.extend(assume_hash(map, groups, next_group));
                }
            };
            next
        },
        |(map, groups)| map.is_empty() && groups.is_empty(),
    )
}

fn assume_hash<'a>(
    map: &'a [char],
    groups: &'a [usize],
    next_group: usize,
) -> Option<(&'a [char], &'a [usize])> {
    if map.len() > next_group && !map[..next_group].contains(&'.') && map[next_group] != '#' {
        Some((&map[next_group + 1..], &groups[1..]))
    } else if map.len() == next_group && !map[..next_group].contains(&'.') {
        Some((&map[next_group..], &groups[1..]))
    } else {
        None
    }
}

fn assume_dot<'a>(map: &'a [char], groups: &'a [usize]) -> Option<(&'a [char], &'a [usize])> {
    if groups.is_empty() || map.len() >= groups.iter().sum::<usize>() + groups.len() - 1 {
        Some((&map[1..], groups))
    } else {
        None
    }
}

boilerplate! {
    part1 => { test1 -> 21, real -> 7653 }
    part2 => { test1 -> 525152, real -> 60681419004564 }
}
