use common::boilerplate;
use itertools::Itertools;

fn part1(input: &str) -> usize {
    let symbols = get_symbols(input).collect_vec();
    let spans = get_spans(input);
    spans
        .into_iter()
        .filter(|span| symbols.iter().any(|symbol| span_near_symbol(span, symbol)))
        .map(|(_x1, _x2, _y, n)| n)
        .sum()
}

fn part2(input: &str) -> usize {
    let spans = get_spans(input);
    get_symbols(input)
        .filter(|&(_, _, c)| c == '*')
        .filter_map(|symbol| {
            let (a, b) = spans
                .iter()
                .filter(|span| span_near_symbol(span, &symbol))
                .collect_tuple()?;
            Some(a.3 * b.3)
        })
        .sum()
}

type Span = (usize, usize, usize, usize);

fn get_spans(input: &str) -> Vec<Span> {
    let mut spans = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        let mut cur_span = None;
        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => {
                    let d = c as usize - '0' as usize;
                    let (_x1, x2, _y, n) = cur_span.get_or_insert((x, x, y, 0));
                    *x2 = x;
                    *n = *n * 10 + d;
                }
                _ => {
                    spans.extend(cur_span.take());
                }
            }
        }
        spans.extend(cur_span.take());
    });
    spans
}

type Symbol = (usize, usize, char);

fn get_symbols(input: &str) -> impl Iterator<Item = Symbol> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| !matches!(c, '.' | '0'..='9'))
            .map(move |(x, c)| (x, y, c))
    })
}

fn span_near_symbol(&(x1, x2, y, _n): &Span, &(sx, sy, _c): &Symbol) -> bool {
    y.saturating_sub(1) <= sy && sy <= y + 1 && x1.saturating_sub(1) <= sx && sx <= x2 + 1
}

boilerplate! {
    part1 => { test -> 4361, real -> 537832 }
    part2 => { test -> 467835, real -> 81939900 }
}
