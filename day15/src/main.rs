use common::{boilerplate, SS};
use std::array;

fn part1(input: SS) -> usize {
    input.split(',').map(hash).sum()
}

fn hash(input: SS) -> usize {
    input.chars().fold(0, |a, c| (a + c as usize) * 17 % 256)
}

fn part2(input: SS) -> usize {
    let mut boxes: [Vec<(SS, u8)>; 256] = array::from_fn(|_| Vec::new());
    for step in input.split(',') {
        if let Some(label) = step.strip_suffix('-') {
            let bx = &mut boxes[hash(label)];
            bx.retain(|(l, _)| *l != label);
        } else {
            let (label, focal) = step.split_once('=').unwrap();
            let focal = focal.parse().unwrap();
            let bx = &mut boxes[hash(label)];
            if let Some(lens) = bx.iter_mut().find(|(l, _)| *l == label) {
                lens.1 = focal;
            } else {
                bx.push((label, focal));
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(b, vec)| {
            vec.into_iter()
                .enumerate()
                .map(move |(l, (_, f))| (b + 1) * (l + 1) * f as usize)
        })
        .sum()
}

boilerplate! {
    part1 => { test -> 1320, real -> 505459 }
    part2 => { test -> 145, real -> 228508 }
}
