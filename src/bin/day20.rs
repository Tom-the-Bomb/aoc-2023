//! Day 20: Pulse Propagation
//!
//! <https://adventofcode.com/2023/day/20>
use std::{
    collections::{VecDeque, HashMap},
    fmt::Display, os::windows::io::BorrowedHandle,
};
use aoc_2023::Solution;

#[derive(Debug, Clone)]
struct Destination {
    name: String,
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

    fn name(&self) -> &'a str {
        match self {
            Self::Flipper { name, .. } => name,
            Self::Conjunction { name, .. } => name,
        }
    }

    fn outputs(&self) -> &Vec<String> {
        match self {
            Self::Flipper { outputs, .. } => outputs,
            Self::Conjunction { outputs, .. } => outputs,
        }
    }
}

pub struct Day20;

impl Day20 {
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
                    modules.insert(
                        name.to_string(),
                        Module::new(name, targets),
                    );
                }
            }
        }

        for (name, module) in &modules {
            for output in module
                .outputs()
                .iter()
                .filter_map(|output| modules
                    .get_mut(output)
                )
            {
                if let Module::Conjunction { memory, .. } = output {
                    memory.insert(
                        name.clone(),
                        false
                    );
                }
            }
        }
        (modules, broadcast_targets)
    }
    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        todo!()
    }

    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        todo!()
    }
}

impl Solution for Day20 {
    const NAME: &'static str = "Pulse Propagation";

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