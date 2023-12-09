//! Day 7: Camel Cards
//!
//! <https://adventofcode.com/2023/day/7>
use std::{
    collections::HashMap,
    fmt::Display,
};
use aoc_2023::Solution;

pub struct Day7;

type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

impl Day7 {
    fn parse<'a, T: Display>(inp: T)
        -> (&'a str, Map<'a>)
    {
        let inp = inp.to_string();
        let (instructions, nodes) = inp
            .replace(|c: char| c.is_whitespace() && c != '\n', "")
            .split_once("\n\n")
            .unwrap();
        (
            instructions,
            nodes
                .lines()
                .map(|line| {
                    let (key, children) = line
                        .split_once('=')
                        .unwrap().clone()
                    (key, children
                        .trim_matches(|c| c == '(' || c == ')')
                        .split_once(',')
                        .unwrap()
                    )
                })
                .collect::<Map>(),
        )
    }

    fn count_instructions<S, F>(
        instructions: S,
        left: S,
        right: S,
        nodes: &Map,
        end_condition: F
    ) -> u64
    where
        S: AsRef<str>,
        F: Fn(&str) -> bool,
    {   
        let mut left = left
            .as_ref();
        let mut right = right
            .as_ref();
        let mut count = 0;
        for instruction in instructions
            .as_ref()
            .chars()
            .cycle()
        {
            count += 1;
            let key = match instruction {
                'L' => left,
                _ => right,
            };
            if end_condition(key) {
                break;
            } else {
                (left, right) = *nodes.get(key)
                    .unwrap();
            }
        }
        count
    }

    pub fn part_one<T: Display>(&self, inp: T) -> u64 {
        let (instructions, nodes) =
            Self::parse(inp);
        let (left, right) = nodes.get("AAA")
            .unwrap();
        Self::count_instructions(
            instructions, left, right, &nodes, |s| s == "ZZZ"
        )
    }

    pub fn part_two<T: Display>(&self, inp: T) -> u64 {
        let (instructions, nodes) =
            Self::parse(inp);
        
        nodes
            .iter()
            .filter_map(|(key, value)|
                key
                    .ends_with('A')
                    .then_some(value)
                    .map(|(left, right)| {
                        Self::count_instructions(
                            instructions, left, right, &nodes, |s| s.ends_with('Z')
                        )
                    })
            )
            .sum()
    }
}

impl Solution for Day7 {
    const NAME: &'static str = "Camel Cards";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 253_313_241);
        assert_eq!(p2, 253_362_743);
    }
}

fn main() {
    aoc_2023::run_day(7, &Day7);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}