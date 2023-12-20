use common::{boilerplate, Itertools, SS};
use std::collections::HashMap;

fn part1(input: SS) -> usize {
    let mut lines = input.lines();
    let workflows: HashMap<_, _> = lines
        .take_while_ref(|line| !line.is_empty())
        .map(parse_workflow)
        .collect();
    assert_eq!(lines.next().unwrap(), "");
    lines
        .map(parse_part)
        .filter(|part| {
            let mut workflow = &workflows["in"];
            'workflow: loop {
                for rule in workflow {
                    if let Some((cat, op, value)) = &rule.condition {
                        let ok = match *op {
                            "<" => part[*cat] < *value,
                            ">" => part[*cat] > *value,
                            _ => unreachable!(),
                        };
                        if !ok {
                            continue;
                        }
                    }
                    match rule.outcome {
                        "A" => break 'workflow true,
                        "R" => break 'workflow false,
                        name => {
                            workflow = &workflows[name];
                            continue 'workflow;
                        }
                    }
                }
            }
        })
        .flatten()
        .sum()
}

fn parse_workflow(line: SS) -> (SS, Vec<Rule>) {
    let (name, rules) = line.split_once('{').unwrap();
    let rules = rules
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .map(parse_rule)
        .collect_vec();
    (name, rules)
}

fn parse_rule(input: SS) -> Rule {
    let (condition, outcome) = input
        .split_once(':')
        .map(|(cond, outcome)| {
            let (cat, txt) = cond.split_at(1);
            let cat = match cat {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                _ => unreachable!(),
            };
            let (op, value) = txt.split_at(1);
            let value = value.parse().unwrap();
            (Some((cat, op, value)), outcome)
        })
        .unwrap_or((None, input));
    Rule { condition, outcome }
}

fn parse_part(input: SS) -> [usize; 4] {
    let (x, m, a, s) = input
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .collect_tuple()
        .unwrap();
    let x = x.strip_prefix("x=").unwrap().parse().unwrap();
    let m = m.strip_prefix("m=").unwrap().parse().unwrap();
    let a = a.strip_prefix("a=").unwrap().parse().unwrap();
    let s = s.strip_prefix("s=").unwrap().parse().unwrap();
    [x, m, a, s]
}

struct Rule {
    condition: Option<(usize, SS, usize)>,
    outcome: SS,
}

fn part2(input: SS) -> usize {
    let workflows: HashMap<_, _> = input
        .lines()
        .take_while_ref(|line| !line.is_empty())
        .map(parse_workflow)
        .collect();

    let mut accepted = 0;
    let mut worklist = vec![("in", [(1, 4000); 4])];
    while let Some((name, mut ranges)) = worklist.pop() {
        let workflow = &workflows[name];
        for rule in workflow {
            if let Some((cat, op, value)) = rule.condition {
                let mut matching = ranges;
                let (start, end) = ranges[cat];
                match op {
                    "<" => {
                        matching[cat] = (start, value - 1);
                        ranges[cat] = (value, end);
                    }
                    ">" => {
                        matching[cat] = (value + 1, end);
                        ranges[cat] = (start, value);
                    }
                    _ => unreachable!(),
                };
                assert_ne!(matching[cat].0, matching[cat].1);
                assert_ne!(ranges[cat].0, ranges[cat].1);
                match rule.outcome {
                    "A" => accepted += calc_combinations(&matching),
                    "R" => (),
                    name => worklist.push((name, matching)),
                }
            } else {
                match rule.outcome {
                    "A" => accepted += calc_combinations(&ranges),
                    "R" => (),
                    name => worklist.push((name, ranges)),
                }
            }
        }
    }

    accepted
}

fn calc_combinations(part_ranges: &[(usize, usize)]) -> usize {
    part_ranges
        .iter()
        .map(|(start, end)| end + 1 - start)
        .product()
}

boilerplate! {
    part1 => { test -> 19114, real -> 352052 }
    part2 => { test -> 167409079868000, real -> 116606738659695 }
}
