use common::{boilerplate, Itertools, SS};
use num_integer::lcm;
use std::collections::{HashMap, VecDeque};

fn part1(input: SS) -> usize {
    let mut map = parse_input(input);

    let mut count = [0, 0];
    for _ in 0..1000 {
        let mut pending = VecDeque::from_iter([("broadcaster", "button", Pulse::Low)]);
        while let Some((to, from, pulse)) = pending.pop_front() {
            count[pulse as usize] += 1;
            pending.extend(map.get_mut(to).unwrap().receive(from, pulse));
        }
    }

    count.iter().product()
}

fn part2(input: SS) -> usize {
    let mut map = parse_input(input);

    // Using some knowledge of the input:
    let rx = &map["rx"];
    assert_eq!(rx.wiring().inputs.len(), 1);
    let Module::Conjunction(wiring) = &map[rx.wiring().inputs.keys().next().unwrap()] else {
        panic!();
    };
    let mut first_high_pulses: HashMap<SS, Option<usize>> =
        wiring.inputs.keys().map(|&name| (name, None)).collect();

    for i in 1.. {
        let mut pending = VecDeque::from_iter([("broadcaster", "button", Pulse::Low)]);
        while let Some((to, from, pulse)) = pending.pop_front() {
            pending.extend(map.get_mut(to).unwrap().receive(from, pulse));
            if pulse == Pulse::High {
                if let Some(value) = first_high_pulses.get_mut(from) {
                    value.get_or_insert(i);
                }
            }
        }
        if let Some(result) = first_high_pulses
            .values()
            .try_fold(1, |a, &v| Some(lcm(a, v?)))
        {
            return result;
        }
    }

    unreachable!()
}

fn parse_input(input: SS) -> HashMap<SS, Module> {
    let mut map: HashMap<SS, _> = input
        .lines()
        .map(|line| {
            let (name, outputs) = line.split_once(" -> ").unwrap();
            let outputs = outputs.split(", ").collect_vec();
            let module = match name.split_at(1) {
                ("%", name) => Module::FlipFlop(FlipFlop::new(name, outputs)),
                ("&", name) => Module::Conjunction(Wiring::new(name, outputs)),
                _ => Module::Broadcaster(Wiring::new(name, outputs)),
            };
            (module.wiring().name, module)
        })
        .collect();
    for key in map.keys().copied().collect_vec() {
        let outputs = map[key].wiring().outputs.to_vec();
        for output in outputs {
            map.entry(output)
                .or_insert(Module::Output(Wiring::new(output, vec![])))
                .wiring_mut()
                .inputs
                .insert(key, Pulse::Low);
        }
    }
    map.get_mut("broadcaster")
        .unwrap()
        .wiring_mut()
        .inputs
        .insert("button", Pulse::Low);
    map
}

#[derive(Clone, Debug)]
struct Wiring {
    name: SS,
    inputs: HashMap<SS, Pulse>,
    outputs: Vec<SS>,
}

impl Wiring {
    fn new(name: SS, outputs: Vec<SS>) -> Self {
        Self {
            name,
            inputs: Default::default(),
            outputs,
        }
    }
}

#[derive(Clone, Debug)]
struct FlipFlop {
    wiring: Wiring,
    on: bool,
}

impl FlipFlop {
    fn new(name: SS, outputs: Vec<SS>) -> Self {
        Self {
            wiring: Wiring::new(name, outputs),
            on: false,
        }
    }
}

#[derive(Clone, Debug)]
enum Module {
    Broadcaster(Wiring),
    FlipFlop(FlipFlop),
    Conjunction(Wiring),
    Output(Wiring),
}

impl Module {
    fn wiring(&self) -> &Wiring {
        match self {
            Module::Broadcaster(wiring) => wiring,
            Module::FlipFlop(FlipFlop { wiring, .. }) => wiring,
            Module::Conjunction(wiring) => wiring,
            Module::Output(wiring) => wiring,
        }
    }

    fn wiring_mut(&mut self) -> &mut Wiring {
        match self {
            Module::Broadcaster(wiring) => wiring,
            Module::FlipFlop(FlipFlop { wiring, .. }) => wiring,
            Module::Conjunction(wiring) => wiring,
            Module::Output(wiring) => wiring,
        }
    }

    fn receive(&mut self, from: SS, pulse: Pulse) -> Vec<(SS, SS, Pulse)> {
        *self.wiring_mut().inputs.get_mut(from).unwrap() = pulse;
        let pulse = match self {
            Module::Broadcaster(_) | Module::Output(_) => pulse,
            Module::FlipFlop(FlipFlop { on, .. }) => {
                if pulse == Pulse::High {
                    return vec![];
                }
                *on = !*on;
                if *on {
                    Pulse::High
                } else {
                    Pulse::Low
                }
            }
            Module::Conjunction(Wiring { inputs, .. }) => {
                if inputs.values().all(|p| p == &Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
        };
        let wiring = self.wiring();
        wiring
            .outputs
            .iter()
            .map(|to| (*to, wiring.name, pulse))
            .collect()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pulse {
    Low,
    High,
}

boilerplate! {
    part1 => { test1 -> 32000000, test2 -> 11687500, real -> 712543680 }
    part2 => { real -> 238920142622879 }
}
