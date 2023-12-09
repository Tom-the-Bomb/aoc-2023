#![feature(iter_map_windows)]

//! Day 9: Mirage Maintenance
//!
//! <https://adventofcode.com/2023/day/7>
use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day9;

impl Day9 {
    fn get_differences(sequence: Vec<i32>) -> Vec<Vec<i32>> {
        let mut differences = vec![sequence];

        while differences
            .first()
            .unwrap()
            .iter()
            .sum::<i32>() != 0
        {
            differences.insert(0,
                differences
                    .first()
                    .unwrap()
                    .iter()
                    .map_windows(|[t1, t2]| *t2 - *t1)
                    .collect::<Vec<i32>>()
            );
        }
        differences
    }

    /// # Panics
    ///
    /// If sequences or differences are somehow empty
    pub fn part_one<T: Display>(&self, inp: T) -> i32 {
        inp
            .to_string()
            .lines()
            .map(|sequence| {
                let mut differences = Self::get_differences(
                    sequence
                        .split_whitespace()
                        .filter_map(|t| t.parse::<i32>().ok())
                        .collect::<Vec<i32>>()
                );
                for (i, difference) in differences
                    .clone()
                    .into_iter()
                    .enumerate()
                {
                    let new_term = difference
                        .last()
                        .map(|d| d
                        + if i == 0 { 0 }
                        else {
                            *differences
                                .get(i - 1)
                                .and_then(|difference| difference.last())
                                .unwrap()
                        })
                        .unwrap();
                    differences
                        .get_mut(i)
                        .unwrap()
                        .push(new_term);
                }
                *differences
                    .last()
                    .and_then(|difference| difference.last())
                    .unwrap()
            })
            .sum()
    }

    /// # Panics
    ///
    /// If sequences or differences are somehow empty
    pub fn part_two<T: Display>(&self, inp: T) -> i32 {
        inp
            .to_string()
            .lines()
            .map(|sequence| {
                let mut differences = Self::get_differences(
                    sequence
                        .split_whitespace()
                        .filter_map(|t| t.parse::<i32>().ok())
                        .collect::<Vec<i32>>()
                );
                for (i, difference) in differences
                    .clone()
                    .into_iter()
                    .enumerate()
                {
                    let new_term = difference
                        .first()
                        .map(|d| d
                        - if i == 0 { 0 }
                        else {
                            *differences
                                .get(i - 1)
                                .and_then(|difference| difference.first())
                                .unwrap()
                        })
                        .unwrap();
                    differences
                        .get_mut(i)
                        .unwrap()
                        .insert(0, new_term);
                }
                *differences
                    .last()
                    .and_then(|difference| difference.first())
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