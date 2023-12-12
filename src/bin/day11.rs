//! Day 11: Cosmic Expansion
//!
//! <https://adventofcode.com/2023/day/11>
use std::fmt::Display;
use aoc_2023::Solution;
use itertools::Itertools;

pub struct Day11;

impl Day11 {
    fn get_galaxies(universe: &[Vec<char>]) -> Vec<(usize, usize)> {
        universe
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row
                .iter()
                .enumerate()
                .filter_map(move |(j, galaxy)|
                    (*galaxy == '#')
                        .then_some((i, j))
                )
            )
            .collect::<Vec<(usize, usize)>>()
    }

    fn get_universe<T: Display>(inp: T) -> Vec<Vec<char>> {
        inp
            .to_string()
            .lines()
            .map(|line| line
                .chars()
                .collect::<Vec<char>>()
            )
            .collect::<Vec<Vec<char>>>()
    }

    fn get_total_distances<T: Display>(inp: T, expansion_amount: u64) -> u64 {
        let universe = Self::get_universe(inp);
        let empty_rows = universe
            .iter()
            .enumerate()
            .filter_map(|(i, row)|
                (!row.contains(&'#'))
                    .then_some(i)
            )
            .collect::<Vec<usize>>();

        let n_rows = universe.len();
        let n_cols = universe
            .first()
            .unwrap()
            .len();

        let empty_cols = (0..n_cols)
            .filter(|j| (0..n_rows)
                .all(|i| universe[i][*j] != '#')
            )
            .collect::<Vec<usize>>();

        Self::get_galaxies(&universe)
            .into_iter()
            .tuple_combinations()
            .map(|((i1, j1), (i2, j2))| {
                (i1.min(i2)..i1.max(i2))
                    .map(|row|
                        if empty_rows.contains(&row) { expansion_amount }
                        else { 1 }
                    )
                    .sum::<u64>()
                + (j1.min(j2)..j1.max(j2))
                    .map(|col|
                        if empty_cols.contains(&col) { expansion_amount }
                        else { 1 }
                    )
                    .sum::<u64>()
            })
            .sum()
    }

    pub fn part_one<T: Display>(&self, inp: T) -> u64 {
        Self::get_total_distances(inp, 2)
    }

    pub fn part_two<T: Display>(&self, inp: T) -> u64 {
        Self::get_total_distances(inp, 1_000_000)
    }
}

impl Solution for Day11 {
    const NAME: &'static str = "Cosmic Expansion";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 9_543_156);
        assert_eq!(p2, 625_243_292_686);
    }
}

fn main() {
    aoc_2023::run_day(11, &Day11);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}