//! Day 1: Trebuchet!?
//!
//! <https://adventofcode.com/2023/day/1>
use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day1;

impl Day1 {
    /// # Panics
    ///
    /// If no digits exist on a line
    pub fn part_one<T: Display>(&self, inp: T) -> u32 {
        inp.to_string()
            .lines()
            .map(|line| {
                let digits = line.chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>();
                digits.first().unwrap() * 10 +
                digits.last().unwrap()
            })
            .sum()
    }

    pub fn part_two<T: Display>(&self, inp: T) -> u32 {
        let mut inp = inp.to_string();
        let map = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ];
        for (key, val) in map {
            inp = inp.replace(key, format!("{key}{val}{key}").as_str());
        }
        self.part_one(inp)
    }
}

impl Solution for Day1 {
    const NAME: &'static str = "Trebuchet!?";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 53651);
        assert_eq!(p2, 53894);
    }
}

fn main() {
    aoc_2023::run_day(1, &Day1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}