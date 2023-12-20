use std::fs::read_to_string;

use itertools::Itertools;
use num::integer::lcm;
use rustc_hash::FxHashMap;

use crate::{Solution, SolutionPair};
use Module::*;

///////////////////////////////////////////////////////////////////////////////

type ModuleMap<'a> = FxHashMap<&'a str, Module<'a>>;
type TargetedPulse<'a> = (bool, &'a str, &'a str);

enum Module<'a> {
    Conjunction { 
        input_labels: Vec<&'a str>,
        state: Vec<bool>,
        outputs: Vec<&'a str>
    },

    FlipFlop {
        state: bool,
        outputs: Vec<&'a str>
    },

    Broadcast {
        outputs: Vec<&'a str>
    }    
}

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day20.txt").unwrap();
    let mut modules: ModuleMap = input.lines().map(parse_line).collect();
    for line in input.lines() {
        update_inputs(line, &mut modules);
    }

    let sol1 = count_pulses(&mut modules);
    let labels_rx = find_rx_components(&modules);
    let sol2 = labels_rx.into_iter()
        .map(|label| find_on_cycle(&mut modules, label))
        .reduce(lcm).unwrap();

    (Solution::from(sol1), Solution::from(sol2))
}

///////////////////////////////////////////////////////////////////////////////

fn find_on_cycle(modules: &mut ModuleMap, label: &str) -> u64 {
    reset_modules(modules);
    let mut i = 0;

    loop {
        i += 1;
        if (0, 0) == push_button(modules, Some(label)) {
            break i;
        }
    }
}

fn count_pulses(modules: &mut ModuleMap) -> u64 {
    let (mut lo, mut hi) = (0, 0);

    for _ in 0..1000 {
        let res = push_button(modules, None);
        lo += res.0;
        hi += res.1;
    }

    lo * hi
}

fn find_rx_components<'a>(modules: &ModuleMap<'a>) -> Vec<&'a str> {
    let module = modules.values().find(|m| m.outputs().contains(&"rx")).unwrap();
    if let Conjunction{input_labels, ..} = module {
        return input_labels.clone();
    }

    panic!("Expected the module that outputs to RX to be a conjunction");
}

fn reset_modules(modules: &mut ModuleMap) {
    for module in modules.values_mut() {
        if let Conjunction{state, ..} = module {
            state.iter_mut().for_each(|x| *x = false);
        } else if let FlipFlop{state, ..} = module {
            *state = false;
        }
    }
}

fn push_button(modules: &mut ModuleMap, break_on: Option<&str>) -> (u64, u64) {
    let mut lo = 1;
    let mut hi = 0;

    let mut to_process = modules["broadcaster"].emit_pulse();

    while !to_process.is_empty() {
        let mut new_to_process = vec![];

        for (pulse, from, to) in to_process {
            if pulse { hi += 1} else { lo += 1 };

            if pulse && break_on == Some(from) {
                return (0, 0);
            }

            if let Some(recv) = modules.get_mut(to) {
                new_to_process.extend(recv.accept_pulse(pulse, from, to));
            }
        }

        to_process = new_to_process;
    }

    (lo, hi)
}

fn parse_line(line: &str) -> (&str, Module) {
    let (head, body) = line.split_once(" -> ").unwrap();
    let outputs = body.split(", ").collect_vec();

    if let Some(label) = head.strip_prefix('%') {
        (label, FlipFlop { state: false, outputs })
    } else if let Some(label) = head.strip_prefix('&') {
        (label, Conjunction { input_labels: vec![], state: vec![], outputs })
    } else {
        (head, Broadcast { outputs })
    }
}

fn update_inputs<'a>(line: &'a str, modules: &mut ModuleMap<'a>) {
    let (head, body) = line.split_once(" -> ").unwrap();
    let label = head.strip_prefix(['&', '%']).unwrap_or(head);
    let outputs = body.split(", ").collect_vec();

    for out_label in outputs {
        if let Some(Conjunction{input_labels, state, ..}) = modules.get_mut(out_label) {
            input_labels.push(label);
            state.push(false);
        }
    }
}

impl<'a> Module<'a> {
    fn emit_pulse<'b>(&'b self) -> Vec<TargetedPulse<'a>> {
        if let Broadcast { outputs } = self {
            return outputs.iter().copied().map(|label| (false, "broadcast", label)).collect();
        }

        unreachable!()
    }

    fn accept_pulse<'b>(&'b mut self, pulse: bool, from: &'a str, this: &'a str) -> Vec<TargetedPulse<'a>> {
        match self {
            Conjunction { input_labels, state, outputs } => {
                let ix = input_labels.iter().position(|x| *x == from).unwrap();
                state[ix] = pulse;
                let out = !state.iter().all(|x| *x);
                outputs.iter().copied().map(|label| (out, this, label)).collect()
            },
            FlipFlop { state, outputs } => {
                if pulse {
                    vec![]
                } else {
                    *state = !*state;
                    outputs.iter().copied().map(|label| (*state, this, label)).collect()
                }
            },
            _ => unreachable!()
        }
    }

    fn outputs(&self) -> &[&str] {
        match self {
            Conjunction { outputs, .. } => outputs,
            FlipFlop { outputs, .. } => outputs,
            Broadcast { outputs } => outputs,
        }
    }
}
