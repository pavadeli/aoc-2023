use common::{boilerplate, to_usize, Itertools};
use range_collections::{RangeSet, RangeSet2};

fn part1(input: &'static str) -> usize {
    let mut lines = input.lines();
    let mut items = RangeSet::empty();
    for seed in parse_seeds(&mut lines) {
        items.union_with(&RangeSet2::from(seed..seed + 1));
    }
    go(lines, items)
}

fn part2(input: &'static str) -> usize {
    let mut lines = input.lines();
    let mut items = RangeSet::empty();
    for (start, length) in parse_seeds(&mut lines).tuples() {
        items.union_with(&RangeSet2::from(start..start + length));
    }
    go(lines, items)
}

fn go(mut lines: impl Iterator<Item = &'static str>, mut items: RangeSet<[usize; 2]>) -> usize {
    assert_eq!(lines.next().unwrap(), "");
    for mapping in parse_mappings(lines) {
        let intersections = mapping
            .iter()
            .map(|map| (map, &map.src & &items))
            .collect_vec();
        for (_, intersection) in &intersections {
            items.difference_with(intersection)
        }
        for (map, intersection) in intersections {
            items.union_with(
                &RangeSet2::new(
                    intersection
                        .into_inner()
                        .into_iter()
                        .map(|v| v - map.src_start() + map.dest_start())
                        .collect(),
                )
                .unwrap(),
            );
        }
    }
    items.boundaries()[0]
}

fn parse_mappings(lines: impl Iterator<Item = &'static str>) -> impl Iterator<Item = Vec<Mapping>> {
    lines.batching(|it| {
        assert!(it.next()?.ends_with("map:"));
        Some(
            it.take_while(|line| !line.is_empty())
                .map(Mapping::parse)
                .sorted_by_key(|map| map.src_start())
                .collect_vec(),
        )
    })
}

fn parse_seeds(mut lines: impl Iterator<Item = &'static str>) -> impl Iterator<Item = usize> {
    lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(to_usize)
}

struct Mapping {
    src: RangeSet2<usize>,
    dest: RangeSet2<usize>,
}

impl Mapping {
    fn parse(s: &str) -> Self {
        let (dest_start, source_start, length) =
            s.split_whitespace().map(to_usize).collect_tuple().unwrap();
        Self {
            dest: RangeSet2::from(dest_start..dest_start + length),
            src: RangeSet2::from(source_start..source_start + length),
        }
    }

    fn src_start(&self) -> usize {
        self.src.boundaries()[0]
    }

    fn dest_start(&self) -> usize {
        self.dest.boundaries()[0]
    }
}

boilerplate! {
    part1 => { test -> 35, real -> 806029445 }
    part2 => { test -> 46, real -> 59370572 }
}
