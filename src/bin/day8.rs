//! Day 7: Camel Cards
//!
//! <https://adventofcode.com/2023/day/7>
use std::{
    collections::HashMap,
    fmt::Display,
};
use aoc_2023::Solution;

pub struct Day8;

type Map = HashMap<String, (String, String)>;

impl Day8 {
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    fn lcm<I>(nums: I) -> u64
    where
        I: Iterator<Item = u64>,
    {
        nums.fold(
            1,
            |num, ans| num * ans / Self::gcd(num, ans),
        )
    }

    fn parse<T: Display>(inp: T)
        -> (String, Map)
    {
        let inp = inp
            .to_string()
            .replace(|c: char| c.is_whitespace() && c != '\n', "");
        let (instructions, nodes) = inp
            .split_once("\n\n")
            .unwrap();
        (
            instructions
                .to_string(),
            nodes
                .lines()
                .map(|line| {
                    let (key, children) = line
                        .split_once('=')
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .unwrap();
                    (key, children
                        .trim_matches(|c| c == '(' || c == ')')
                        .split_once(',')
                        .map(|(k, v)| (k.to_string(), v.to_string()))
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
        S: Display,
        F: Fn(&String) -> bool,
    {
        let mut left = &left
            .to_string();
        let mut right = &right
            .to_string();
        let mut count = 0;
        for instruction in instructions
            .to_string()
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
            }
            let values = nodes.get(key)
                .unwrap();
            left = &values.0;
            right = &values.1;
        }
        count
    }

    /// # Panics
    ///
    /// Panics if the AAA node does not exist for some reason
    pub fn part_one<T: Display>(&self, inp: T) -> u64 {
        let (instructions, nodes) =
            Self::parse(inp);
        let (left, right) = nodes.get("AAA")
            .unwrap();
        Self::count_instructions(
            &instructions, left, right, &nodes, |s| s == "ZZZ"
        )
    }

    pub fn part_two<T: Display>(&self, inp: T) -> u64 {
        let (instructions, nodes) =
            Self::parse(inp);

        Self::lcm(nodes
            .iter()
            .filter_map(|(key, value)|
                key
                    .ends_with('A')
                    .then_some(value)
                    .map(|(left, right)| {
                        Self::count_instructions(
                            &instructions, left, right, &nodes, |s| s.ends_with('Z')
                        )
                    })
            )
        )
    }
}

impl Solution for Day8 {
    const NAME: &'static str = "Camel Cards";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 18727);
        assert_eq!(p2, 18_024_643_846_273);
    }
}

fn main() {
    aoc_2023::run_day(8, &Day8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}