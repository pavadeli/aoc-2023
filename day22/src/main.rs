use common::{boilerplate, to_usize, Itertools, SS};
use std::{
    collections::{HashMap, HashSet},
    sync::atomic::{self, AtomicUsize},
};

struct Brick {
    id: usize,
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize),
    supports: Vec<usize>,
    supported_by: Vec<usize>,
}

impl Brick {
    fn parse(s: SS) -> Self {
        let (start, end) = s
            .split('~')
            .map(|s| -> (usize, usize, usize) {
                s.split(',').map(to_usize).collect_tuple().unwrap()
            })
            .collect_tuple()
            .expect("invalid input");
        assert!(start.0 <= end.0);
        assert!(start.1 <= end.1);
        assert!(start.2 <= end.2);

        static ID: AtomicUsize = AtomicUsize::new(0);
        Self {
            id: ID.fetch_add(1, atomic::Ordering::Relaxed),
            x: (start.0, end.0),
            y: (start.1, end.1),
            z: (start.2, end.2),
            supports: Default::default(),
            supported_by: Default::default(),
        }
    }

    fn same_horizontal_space(&self, other: &Self) -> bool {
        self.x.0 <= other.x.1
            && self.x.1 >= other.x.0
            && self.y.0 <= other.y.1
            && self.y.1 >= other.y.0
    }
}

fn parse(input: SS) -> HashMap<usize, Brick> {
    input.lines().map(Brick::parse).map(|b| (b.id, b)).collect()
}

fn play(bricks: &mut HashMap<usize, Brick>) {
    let mut top_z_map = bricks
        .values()
        .map(|b| (b.z.1, b.id))
        .into_grouping_map()
        .collect::<HashSet<_>>();
    let bottom_z_list = bricks
        .values()
        .sorted_by_key(|b| b.z.0)
        .map(|b| b.id)
        .collect_vec();

    'next_brick: for id in &bottom_z_list {
        for z in (0..bricks[id].z.0).rev() {
            let mut found_support = z == 0;
            for other_id in top_z_map.get(&z).iter().copied().flatten() {
                if bricks[other_id].same_horizontal_space(&bricks[id]) {
                    bricks.get_mut(other_id).unwrap().supports.push(*id);
                    bricks.get_mut(id).unwrap().supported_by.push(*other_id);
                    found_support = true;
                }
            }
            if !found_support {
                continue;
            }
            let b = bricks.get_mut(id).unwrap();
            top_z_map.get_mut(&b.z.1).unwrap().remove(id);
            b.z = (z + 1, z + 1 + b.z.1 - b.z.0);
            top_z_map.entry(b.z.1).or_default().insert(*id);
            continue 'next_brick;
        }
    }
}

fn part1(input: SS) -> usize {
    let mut bricks = parse(input);
    play(&mut bricks);

    bricks
        .values()
        .filter(|b| {
            b.supports
                .iter()
                .all(|id| bricks[id].supported_by.len() > 1)
        })
        .count()
}

fn part2(input: SS) -> usize {
    let mut bricks = parse(input);
    play(&mut bricks);

    bricks
        .values()
        .map(|brick| {
            let mut destroyed = vec![brick.id];
            let mut i = 0;
            while i < destroyed.len() {
                let b = &bricks[&destroyed[i]];
                for id in &b.supports {
                    if destroyed.contains(id) {
                        continue;
                    }
                    let supported_brick = &bricks[id];
                    if supported_brick
                        .supported_by
                        .iter()
                        .all(|b| destroyed.contains(b))
                    {
                        destroyed.push(*id);
                    }
                }
                i += 1;
            }
            destroyed.len() - 1
        })
        .sum()
}

boilerplate! {
    part1 => { test -> 5, real -> 426 }
    part2 => { test -> 7, real -> 61920 }
}
