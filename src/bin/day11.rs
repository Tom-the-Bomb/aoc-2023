//! Day 11: Cosmic Expansion
//!
//! <https://adventofcode.com/2023/day/11>
use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day11;

impl Day11 {
    /// Brute force method
    /// that expands the universe
    /// by +1 row for each empty row and +1 column for each empty colun
    fn expand_one(mut universe: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        for _ in 0..2 {
            let mut new_vec = Vec::with_capacity(universe.len());
            for row in universe {
                if !row.contains(&b'#') {
                    new_vec.push(row.clone());
                }
                new_vec.push(row);
            }
            // matrix transposal
            universe = (0..new_vec[0].len())
                .map(|i| (0..new_vec.len())
                    .map(|j| new_vec[j][i])
                    .collect::<Vec<u8>>()
                )
                .collect::<Vec<Vec<u8>>>();
        }
        universe
    }

    fn get_galaxies(universe: &[Vec<u8>]) -> Vec<(usize, usize)> {
        universe
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row
                .iter()
                .enumerate()
                .filter_map(move |(j, galaxy)|
                    (*galaxy == b'#')
                        .then_some((i, j))
                )
            )
            .collect::<Vec<(usize, usize)>>()
    }

    fn get_universe<T: Display>(inp: T) -> Vec<Vec<u8>> {
        inp
            .to_string()
            .lines()
            .map(|line| line
                .as_bytes()
                .to_vec()
            )
            .collect::<Vec<Vec<u8>>>()
    }

    fn get_total_distances<T: Display>(inp: T, expansion_amount: usize) -> usize {
        let universe = Self::get_universe(inp);
        let empty_rows = &universe
            .iter()
            .enumerate()
            .filter_map(|(i, row)|
                (!row.contains(&b'#'))
                    .then_some(i)
            )
            .collect::<Vec<usize>>();

        let n_rows = universe.len();
        let n_cols = universe
            .first()
            .unwrap()
            .len();

        let empty_cols = &(0..n_cols)
            .filter(|j| (0..n_rows)
                .all(|i| universe[i][*j] != b'#')
            )
            .collect::<Vec<usize>>();

        let galaxies = Self::get_galaxies(&universe);

        galaxies
            .iter()
            .enumerate()
            // galaxy 1
            .flat_map(|(idx, &(i1, j1))|
                galaxies
                    .iter()
                    // effectively equivalent to getting combinations where k=2
                    .take(idx)
                    // galaxy 2
                    .map(move |&(i2, j2)|
                        (i1.min(i2)..i1.max(i2))
                            .map(|row|
                                if empty_rows.contains(&row) { expansion_amount }
                                else { 1 }
                            )
                            .sum::<usize>()
                        + (j1.min(j2)..j1.max(j2))
                            .map(|col|
                                if empty_cols.contains(&col) { expansion_amount }
                                else { 1 }
                            )
                            .sum::<usize>()
                    )
            )
            .sum()
    }

    /// Brute force solution for Part 1
    /// Uses the strategy of actually expanding the universe's matrix
    pub fn part_one_bf<T: Display>(&self, inp: T) -> usize {
        let universe = Self::expand_one(
            Self::get_universe(inp)
        );
        let galaxies = Self::get_galaxies(&universe);

        galaxies
            .iter()
            .enumerate()
            .flat_map(|(idx, &(i1, j1))|
                galaxies
                    .iter()
                    .take(idx)
                    .map(move |&(i2, j2)|
                        i2.abs_diff(i1) + j2.abs_diff(j1)
                    )
            )
            .sum()
    }

    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        Self::get_total_distances(inp, 2)
    }

    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        Self::get_total_distances(inp, 1_000_000)
    }
}

impl Solution for Day11 {
    const NAME: &'static str = "Cosmic Expansion";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        assert_eq!(p1, self.part_one_bf(&inp));

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