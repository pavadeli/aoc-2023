use common::{boilerplate, Itertools, SS};
use pathfinding::matrix::Matrix;
use rand::{seq::IteratorRandom, thread_rng};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::{BTreeSet, HashMap};

#[derive(Clone)]
struct Graph {
    edges: Matrix<bool>,
    nodesets: Matrix<bool>,
}

type Edge = (usize, usize);

impl Graph {
    fn parse(input: SS) -> Self {
        let mut input_edges = HashMap::new();
        let mut nodes = BTreeSet::new();
        for line in input.lines() {
            let (left, rights) = line.split_once(": ").unwrap();
            let rights = rights.split_whitespace().collect_vec();
            nodes.insert(left);
            nodes.extend(&rights);
            input_edges.insert(left, rights);
        }
        let nodes = nodes.into_iter().collect_vec();
        let count = nodes.len();

        Self {
            edges: {
                let mut edges = Matrix::new(count, count, false);
                for (left, rights) in input_edges {
                    let left = nodes.binary_search(&left).unwrap();
                    for right in rights {
                        let right = nodes.binary_search(&right).unwrap();
                        edges[(left, right)] = true;
                        edges[(right, left)] = true;
                    }
                }
                edges
            },
            nodesets: {
                let mut nodesets = Matrix::new(count, count, false);
                for i in 0..count {
                    nodesets[(i, i)] = true;
                }
                nodesets
            },
        }
    }

    fn nodes(&self) -> impl Iterator<Item = usize> + '_ {
        self.nodesets
            .iter()
            .enumerate()
            // Only report the first node in each set.
            .filter(|&(node, set)| !set[0..node].contains(&true))
            .map(|(node, _)| node)
    }

    fn edges(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.iter().enumerate().flat_map(move |(i, row)| {
            row.iter()
                .copied()
                .enumerate()
                .skip(i + 1)
                .filter_map(move |(j, b)| (b && !self.nodesets[(i, j)]).then_some((i, j)))
        })
    }

    fn nodeset(&self, node: usize) -> impl Iterator<Item = usize> + '_ {
        self.nodesets
            .iter()
            .nth(node)
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.then_some(i))
    }

    fn merge_nodesets(&mut self, a: usize, b: usize) {
        let set_a = self.nodeset(a).collect_vec();
        let set_b = self.nodeset(b).collect_vec();
        for (a, b) in set_a.into_iter().cartesian_product(set_b) {
            self.nodesets[(a, b)] = true;
            self.nodesets[(b, a)] = true;
        }
    }
}

fn part1(input: SS) -> usize {
    let original_graph = Graph::parse(input);

    // See: https://en.wikipedia.org/wiki/Minimum_cut
    (0..)
        .par_bridge()
        .find_map_any(|_| {
            let rng = &mut thread_rng();
            let mut graph = original_graph.clone();
            while graph.nodes().nth(20).is_some() {
                let (a, b) = graph.edges().choose(rng).unwrap();
                graph.merge_nodesets(a, b);
            }
            let base_graph = graph;
            for _ in 0..20 {
                let mut graph = base_graph.clone();
                while graph.nodes().nth(2).is_some() {
                    let (a, b) = graph.edges().choose(rng).unwrap();
                    graph.merge_nodesets(a, b);
                }
                let count = graph.edges().count();
                if count == 3 {
                    let (first, second) = graph
                        .nodes()
                        .map(|node| graph.nodeset(node).collect_vec())
                        .collect_tuple()
                        .unwrap();
                    return Some(first.len() * second.len());
                }
            }
            None
        })
        .unwrap()
}

boilerplate! {
    part1 => { test -> 54, real -> 543256 }
}
