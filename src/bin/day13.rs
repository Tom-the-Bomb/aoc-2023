//! Day 13: Point of Incidence
//!
//! <https://adventofcode.com/2023/day/13>
use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day13;

impl Day13 {
    fn find_mirror<T>(grid: T, smudge: usize) -> usize
    where
        T: AsRef<[Vec<u8>]>,
    {
        let grid = grid.as_ref();

        for line in 1..grid.len() {
            let mut left = grid
                .iter()
                .take(line)
                .rev()
                .collect::<Vec<&Vec<u8>>>();
            let mut right = grid
                .iter()
                .skip(line)
                .collect::<Vec<&Vec<u8>>>();
            let left_size = left.len();
            let right_size = right.len();

            if left_size > right_size {
                left = left[..right_size]
                    .to_vec();
            } else {
                right = right[..left_size]
                    .to_vec();
            }

            if smudge == 0
                && left == right
                || left
                    .into_iter()
                    .flatten()
                    .zip(right
                        .into_iter()
                        .flatten()
                    )
                    .map(|(t1, t2)| usize::from(t1 != t2))
                    .sum::<usize>()
                    == smudge
            {
                return line;
            }
        }
        0
    }

    fn get_summary<T: Display>(inp: T, smudge: usize) -> usize {
        inp
            .to_string()
            .replace('\r', "")
            .split("\n\n")
            .map(|grid| {
                let grid = grid.lines()
                    .map(|line| line
                        .as_bytes()
                        .to_vec()
                    )
                    .collect::<Vec<Vec<u8>>>();
                let n_rows = grid.len();
                let n_cols = grid
                    .first()
                    .unwrap()
                    .len();
                100 * Self::find_mirror(&grid, smudge)
                    + Self::find_mirror(
                        (0..n_cols)
                            .map(|i| (0..n_rows)
                                .map(|j| grid[j][i])
                                .collect::<Vec<u8>>()
                            )
                        .collect::<Vec<Vec<u8>>>(),
                        smudge
                    )
            })
            .sum()
    }
}

impl Solution for Day13 {
    const NAME: &'static str = "Point of Incidence";

    fn part_one<T: Display>(&self, inp: T) -> usize {
        Self::get_summary(inp, 0)
    }

    fn part_two<T: Display>(&self, inp: T) -> usize {
        Self::get_summary(inp, 1)
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 35210);
        assert_eq!(p2, 31974);
    }
}

fn main() {
    aoc_2023::run_day(13, &Day13);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}