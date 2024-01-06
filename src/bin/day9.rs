#![feature(iter_map_windows)]

//! Day 9: Mirage Maintenance
//!
//! <https://adventofcode.com/2023/day/9>
use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day9;

impl Day9 {
    fn get_diffs(sequence: Vec<isize>) -> Vec<Vec<isize>> {
        let mut diffs = vec![sequence];

        while diffs
            .first()
            .unwrap()
            .iter()
            .sum::<isize>() != 0
        {
            diffs.insert(0,
                diffs
                    .first()
                    .unwrap()
                    .iter()
                    .map_windows(|[t1, t2]| *t2 - *t1)
                    .collect::<Vec<isize>>()
            );
        }
        diffs
    }
}

impl Solution for Day9 {
    const NAME: &'static str = "Mirage Maintenance";

    /// # Panics
    ///
    /// If sequences or diffs are somehow empty
    #[allow(clippy::cast_sign_loss)]
    fn part_one<T: Display>(&self, inp: T) -> usize {
        inp
            .to_string()
            .lines()
            .map(|sequence| {
                let mut diffs = Self::get_diffs(
                    sequence
                        .split_whitespace()
                        .filter_map(|t| t.parse::<isize>().ok())
                        .collect::<Vec<isize>>()
                );
                for i in 0..diffs.len() {
                    let new_term = diffs[i]
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
            .sum::<isize>() as usize
    }

    /// # Panics
    ///
    /// If sequences or diffs are somehow empty
    #[allow(clippy::cast_sign_loss)]
    fn part_two<T: Display>(&self, inp: T) -> usize {
        inp
            .to_string()
            .lines()
            .map(|sequence| {
                let mut diffs = Self::get_diffs(
                    sequence
                        .split_whitespace()
                        .filter_map(|t| t.parse::<isize>().ok())
                        .collect::<Vec<isize>>()
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
            .sum::<isize>() as usize
    }

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