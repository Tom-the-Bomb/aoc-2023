#![feature(iter_map_windows)]

//! Day 9: Mirage Maintenance
//!
//! <https://adventofcode.com/2023/day/7>
use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day9;

impl Day9 {
    fn get_diffs(sequence: Vec<i32>) -> Vec<Vec<i32>> {
        let mut diffs = vec![sequence];

        while diffs
            .first()
            .unwrap()
            .iter()
            .sum::<i32>() != 0
        {
            diffs.insert(0,
                diffs
                    .first()
                    .unwrap()
                    .iter()
                    .map_windows(|[t1, t2]| *t2 - *t1)
                    .collect::<Vec<i32>>()
            );
        }
        diffs
    }

    /// # Panics
    ///
    /// If sequences or diffs are somehow empty
    pub fn part_one<T: Display>(&self, inp: T) -> i32 {
        inp
            .to_string()
            .lines()
            .map(|sequence| {
                let mut diffs = Self::get_diffs(
                    sequence
                        .split_whitespace()
                        .filter_map(|t| t.parse::<i32>().ok())
                        .collect::<Vec<i32>>()
                );
                for (i, diff) in diffs
                    .clone()
                    .into_iter()
                    .enumerate()
                {
                    let new_term = diff
                        .last()
                        .and_then(|previous_term|
                            if i == 0 { Some(&0) }
                            else {
                                diffs
                                    .get(i - 1)
                                    .and_then(|previous_diff| previous_diff.last())
                            }
                            .map(|new_diff| previous_term + new_diff)
                        )
                        .unwrap();
                    diffs[i].push(new_term);
                }
                *diffs
                    .last()
                    .and_then(|sequence| sequence.last())
                    .unwrap()
            })
            .sum()
    }

    /// # Panics
    ///
    /// If sequences or diffs are somehow empty
    pub fn part_two<T: Display>(&self, inp: T) -> i32 {
        inp
            .to_string()
            .lines()
            .map(|sequence| {
                let mut diffs = Self::get_diffs(
                    sequence
                        .split_whitespace()
                        .filter_map(|t| t.parse::<i32>().ok())
                        .collect::<Vec<i32>>()
                );
                for (i, diff) in diffs
                    .clone()
                    .into_iter()
                    .enumerate()
                {
                    let new_term = diff
                        .first()
                        .and_then(|previous_term|
                            if i == 0 { Some(&0) }
                            else {
                                diffs
                                    .get(i - 1)
                                    .and_then(|previous_diff| previous_diff.first())
                            }
                            .map(|new_diff| previous_term - new_diff)
                        )
                        .unwrap();
                    diffs[i].insert(0, new_term);
                }
                *diffs
                    .last()
                    .and_then(|diff| diff.first())
                    .unwrap()
            })
            .sum()
    }
}

impl Solution for Day9 {
    const NAME: &'static str = "Mirage Maintenance";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 1_647_269_739);
        assert_eq!(p2, 864);
    }
}

fn main() {
    aoc_2023::run_day(9, &Day9);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}