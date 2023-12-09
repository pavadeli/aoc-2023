use common::{boilerplate, EitherOrBoth::Both, Itertools, SS};
use std::{collections::HashMap, usize};

fn part1(input: SS) -> usize {
    let (instr, nodes) = parse_input(input);
    instr
        .chars()
        .cycle()
        .scan("AAA", |node, dir| {
            *node = next_node(dir, &nodes, node);
            Some(*node)
        })
        .take_while_inclusive(|node| node != &"ZZZ")
        .count()
}

fn parse_input(input: SS) -> (SS, HashMap<SS, (SS, SS)>) {
    let mut lines = input.lines();
    let instr = lines.next().unwrap();
    assert_eq!(lines.next().unwrap(), "");
    let nodes: HashMap<_, _> = lines.map(parse_node).collect();
    (instr, nodes)
}

fn parse_node(input: SS) -> (SS, (SS, SS)) {
    let (name, branches) = input.split_once(" = ").unwrap();
    let branches = branches
        .strip_prefix('(')
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .split_once(", ")
        .unwrap();
    (name, branches)
}

fn next_node(dir: char, nodes: &HashMap<SS, (SS, SS)>, from: SS) -> SS {
    let (left, right) = nodes[from];
    match dir {
        'L' => left,
        'R' => right,
        _ => unreachable!(),
    }
}

fn part2(input: SS) -> usize {
    let (instr, nodes) = parse_input(input);
    let combined = nodes
        .keys()
        .filter(|key| key.ends_with('A'))
        .map(|start| Spectral::new(instr, &nodes, start))
        .tree_fold1(Spectral::combine_with)
        .unwrap();

    combined.pattern[0]
}

struct Spectral {
    pattern: Vec<usize>,
    pattern_start: usize,
    pattern_end: usize,
}

impl Spectral {
    fn new(instr: SS, nodes: &HashMap<SS, (SS, SS)>, start: SS) -> Self {
        let instructions = instr.chars().enumerate().cycle().enumerate();
        let mut node = start;
        let mut seen: HashMap<(usize, SS), usize> = HashMap::new();
        let mut pattern = vec![];
        for (step, (instr_id, instr)) in instructions {
            node = next_node(instr, nodes, node);
            if let Some(&pattern_start) = seen.get(&(instr_id, node)) {
                assert!(pattern.iter().all(|step| *step >= pattern_start));
                return Self {
                    pattern,
                    pattern_start,
                    pattern_end: step + 1,
                };
            }
            seen.insert((instr_id, node), step + 1);
            if node.ends_with('Z') {
                pattern.push(step + 1);
            }
        }
        unreachable!()
    }

    fn combine_with(self, other: Self) -> Self {
        let mut seen: HashMap<(usize, usize), usize> = HashMap::new();
        let mut pattern = vec![];
        for (self_id, other_id, step) in self
            .iter()
            .merge_join_by(other.iter(), |a, b| a.1.cmp(&b.1))
            .filter_map(|combo| match combo {
                Both((self_id, step), (other_id, _)) => Some((self_id, other_id, step)),
                _ => None,
            })
        {
            if let Some(&pattern_start) = seen.get(&(self_id, other_id)) {
                assert!(pattern.iter().all(|step| *step >= pattern_start));
                return Self {
                    pattern,
                    pattern_start,
                    pattern_end: step,
                };
            }
            seen.insert((self_id, other_id), step);
            pattern.push(step);
        }
        unreachable!()
    }

    fn iter(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let pattern_length = self.pattern_end - self.pattern_start;
        let pattern_items = self.pattern.len();
        (0..).map(move |i| {
            let idx = i % pattern_items;
            let round = i / pattern_items;
            (idx, self.pattern[idx] + round * pattern_length)
        })
    }
}

boilerplate! {
    part1 => { test1 -> 2, test2 -> 6, real -> 14257 }
    part2 => { test3 -> 6, real -> 16187743689077 }
}
