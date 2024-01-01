use common::{boilerplate, Itertools, SS};
use pathfinding::matrix::Matrix;
use std::collections::{HashMap, HashSet};

type Node = (usize, usize);

type Graph = HashMap<Node, HashMap<Node, usize>>;

fn create_graph(input: SS, directed: bool) -> (Graph, Node) {
    let matrix = Matrix::from_iter(input.lines().map(|line| line.chars()));
    let start = (0, 1);
    let end = (matrix.rows - 1, matrix.columns - 2);

    assert_eq!(matrix[(0, 0)], '#');
    assert_eq!(matrix[start], '.');
    assert_eq!(matrix[(0, 2)], '#');
    assert_eq!(matrix[(end.0, end.1 - 1)], '#');
    assert_eq!(matrix[end], '.');
    assert_eq!(matrix[(end.0, end.1 + 1)], '#');

    let mut graph: Graph = Default::default();
    let mut nodes_seen = HashSet::new();
    nodes_seen.insert(start);

    // `remaining` contains tuples of nodes and the first step into the path that should be
    // discovered.
    // By starting just below `start`, we can skip `>= 0` bounds checks.
    let mut remaining = vec![(start, (1, 1))];

    'remaining: while let Some((node, mut cursor)) = remaining.pop() {
        let mut path = vec![node, cursor];
        for length in 1.. {
            let (y, x) = cursor;
            let dirs = [
                ((y - 1, x), '^'),
                ((y + 1, x), 'v'),
                ((y, x - 1), '<'),
                ((y, x + 1), '>'),
            ]
            .into_iter()
            .filter(|&(p, ch)| {
                (if directed {
                    matrix[p] == '.' || matrix[p] == ch
                } else {
                    matrix[p] != '#'
                }) && !path.contains(&p)
            })
            .map(|(p, _)| p)
            .collect_vec();
            match &dirs[..] {
                [] => continue 'remaining,
                [next] => {
                    cursor = *next;
                    path.push(cursor);
                    if cursor == end {
                        add_to_graph(&mut graph, node, cursor, length);
                        continue 'remaining;
                    }
                }
                // multiple paths ahead, this is a node
                _ => {
                    add_to_graph(&mut graph, node, cursor, length);
                    if !directed {
                        add_to_graph(&mut graph, cursor, node, length)
                    }
                    if nodes_seen.insert(cursor) {
                        remaining.extend(dirs.into_iter().map(|p| (cursor, p)));
                    }
                    continue 'remaining;
                }
            }
        }
    }

    (graph, end)
}

fn add_to_graph(graph: &mut Graph, from: Node, to: Node, length: usize) {
    graph
        .entry(from)
        .or_default()
        .entry(to)
        .and_modify(|cost| *cost = (*cost).max(length))
        .or_insert(length);
}

fn longest_path_to_end_from(
    graph: &Graph,
    node: Node,
    end: Node,
    path: &mut Vec<Node>,
) -> Option<usize> {
    if node == end {
        return Some(0);
    }
    let len = path.len();
    path.push(node);

    let max = graph
        .get(&node)
        .unwrap()
        .iter()
        .filter_map(|(&p, &cost)| {
            if path.contains(&p) {
                None
            } else {
                Some(longest_path_to_end_from(graph, p, end, path)? + cost)
            }
        })
        .max();
    path.truncate(len);
    max
}

fn part1(input: SS) -> usize {
    let (graph, end) = create_graph(input, true);
    longest_path_to_end_from(&graph, (0, 1), end, &mut vec![]).unwrap() + 1
}

fn part2(input: SS) -> usize {
    let (graph, end) = create_graph(input, false);
    longest_path_to_end_from(&graph, (0, 1), end, &mut vec![]).unwrap() + 1
}

boilerplate! {
    part1 => { test -> 94, real -> 1930 }
    part2 => { test -> 154, real -> 6230 }
}
