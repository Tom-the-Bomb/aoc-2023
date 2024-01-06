#![feature(map_try_insert)]

//! Day 20: Pulse Propagation
//!
//! <https://adventofcode.com/2023/day/20>
use std::{
    collections::{VecDeque, HashMap},
    fmt::Display,
};
use aoc_2023::{Solution, lcm};

#[derive(Debug, Clone)]
struct Destination<'a> {
    name: &'a str,
    target: String,
    pulse: bool,
}

#[derive(Debug, Clone)]
enum Module<'a> {
    Flipper {
        name: &'a str,
        outputs: Vec<String>,
        status: bool,
    },
    Conjunction {
        name: &'a str,
        outputs: Vec<String>,
        memory: HashMap<String, bool>,
    }
}

impl<'a> Module<'a> {
    #[must_use]
    fn new(raw: &'a str, outputs: Vec<String>) -> Self {
        match raw
            .split_at(1)
        {
            ("%", name) => Self::Flipper {
                name, outputs,
                status: false,
            },
            ("&", name) => Self::Conjunction {
                name, outputs,
                memory: HashMap::new(),
            },
            _ => panic!("Invalid module prefix"),
        }
    }

    #[inline]
    #[must_use]
    const fn name(&self) -> &'a str {
        match self {
            Self::Flipper { name, .. }
            | Self::Conjunction { name, .. } => name,
        }
    }

    #[inline]
    #[must_use]
    const fn outputs(&self) -> &Vec<String> {
        match self {
            Self::Flipper { outputs, .. }
            | Self::Conjunction { outputs, .. } => outputs,
        }
    }
}

pub struct Day20;

impl Day20 {
    #[must_use]
    fn parse_input<T>(inp: &T) -> (HashMap<String, Module<'_>>, Vec<String>)
    where
        T: AsRef<str>,
    {
        let mut modules = HashMap::new();
        let mut broadcast_targets = Vec::new();

        for line in inp
            .as_ref()
            .lines()
        {
            let (name, targets) = line
                .split_once("->")
                .unwrap();
            let targets = targets
                .split(',')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>();

            match name.trim() {
                "broadcaster" => broadcast_targets = targets,
                name => {
                    let module = Module::new(name, targets);
                    modules.insert(
                        module.name().to_string(),
                        module,
                    );
                }
            }
        }

        for (name, module) in modules
            .clone()
        {
            for output in module
                .outputs()
            {
                if let Some(Module::Conjunction {
                    memory, ..
                }) = modules.get_mut(output) {
                    memory.insert(
                        name.to_string(),
                        false
                    );
                }
            }
        }
        (modules, broadcast_targets)
    }

    fn run_modules<'a>(
        destinations: &mut VecDeque<Destination<'a>>,
        mut module: &mut Module<'a>,
        source: &'a str,
        pulse: bool,
    ) {
        let pulse_to_send = match (&mut module, pulse) {
            (Module::Flipper { status, .. }, false) => {
                *status = !*status;
                *status
            },
            (Module::Conjunction { memory, .. }, pulse) => {
                memory.insert(source.to_string(), pulse);
                !memory
                    .values()
                    .all(|x| *x)
            },
            _ => return,
        };

        for output in module.outputs() {
            destinations.push_back(
                Destination {
                    target: output.to_string(),
                    pulse: pulse_to_send,
                    name: module.name(),
                }
            );
        }
    }

    #[inline]
    #[must_use]
    fn get_destinations<'a, T>(broadcast_targets: T) -> VecDeque<Destination<'a>>
    where
        T: AsRef<[String]>,
    {
        broadcast_targets
            .as_ref()
            .iter()
            .map(|target| Destination {
                target: target.to_string(),
                pulse: false,
                name: "broadcaster",
            })
            .collect::<VecDeque<Destination>>()
    }
}

impl Solution for Day20 {
    const NAME: &'static str = "Pulse Propagation";

    fn part_one<T: Display>(&self, inp: T) -> usize {
        let inp = inp.to_string();
        let (mut modules, broadcast_targets) =
            Self::parse_input(&inp);

        let mut n_low = 0;
        let mut n_high = 0;

        for _ in 0..1000 {
            n_low += 1;
            let mut destinations =
                Self::get_destinations(&broadcast_targets);

            while let Some(source) = destinations.pop_front() {
                if source.pulse {
                    n_high += 1;
                } else {
                    n_low += 1;
                }

                if let Some(module) = modules.get_mut(&source.target) {
                    Self::run_modules(
                        &mut destinations,
                        module,
                        source.name,
                        source.pulse
                    );
                }
            }
        }
        n_low * n_high
    }

    /// # Panics
    ///
    /// If the module that feeds into 'rx' is not found
    fn part_two<T: Display>(&self, inp: T) -> usize {
        let inp = inp.to_string();
        let (mut modules, broadcast_targets) =
            Self::parse_input(&inp);

        let rx_feeder = modules
            .iter()
            .find_map(|(name, module)| module
                .outputs()
                .contains(&"rx".to_string())
                .then_some(name)
            )
            .unwrap()
            .to_string();

        let mut seen = modules
            .iter()
            .filter_map(|(name, module)|
                module
                    .outputs()
                    .contains(&rx_feeder)
                    .then_some((
                        name.to_string(),
                        false,
                    ))
            )
            .collect::<HashMap<String, bool>>();

        let mut press_amounts = HashMap::new();

        for n_presses in 1.. {
            let mut destinations =
                Self::get_destinations(&broadcast_targets);

            while let Some(source) = destinations.pop_front() {
                if let Some(module) = modules.get_mut(&source.target) {
                    if module.name() == rx_feeder && source.pulse {
                        seen.insert(
                            source.name.to_string(),
                            true,
                        );
                        press_amounts
                            .try_insert(source.name, n_presses)
                            .ok();

                        if seen
                            .values()
                            .all(|b| *b)
                        {
                            return lcm(press_amounts
                                .values()
                                .copied()
                            );
                        }
                    }
                    Self::run_modules(
                        &mut destinations,
                        module,
                        source.name,
                        source.pulse,
                    );
                }
            }
        }
        unreachable!()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 680_278_040);
        assert_eq!(p2, 243_548_140_870_057);
    }
}

fn main() {
    aoc_2023::run_day(20, &Day20);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}