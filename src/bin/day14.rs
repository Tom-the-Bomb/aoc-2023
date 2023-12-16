//! Day 14: Parabolic Reflector Dish
//!
//! <https://adventofcode.com/2023/day/14>
use std::{collections::VecDeque, fmt::Display};
use aoc_2023::Solution;

pub struct Day14;

type Grid = Vec<Vec<u8>>;

impl Day14 {
    fn transpose(grid: &Grid) -> Grid {
        (0..grid[0].len())
            .map(|i| (0..grid.len())
                .map(|j| grid[j][i])
                .collect::<Vec<u8>>()
            )
            .collect::<Grid>()
    }

    fn reverse_rows(grid: &mut Grid) {
        for row in grid {
            row.reverse();
        }
    }

    fn tilt_lever(grid: &mut Grid) {
        for row in grid {
            for i in 0..row.len() {
                if row[i] != b'O' {
                    continue;
                }
                for t in (1..=i).rev() {
                    if row[t - 1] != b'.' {
                        break;
                    }
                    row.swap(t, t - 1);
                }
            }
        }
    }

    fn tilt_north(grid: &Grid) -> Grid {
        let mut grid = Self::transpose(grid);
        Self::tilt_lever(&mut grid);
        Self::transpose(&grid)
    }

    fn tilt_south(grid: &Grid) -> Grid {
        let mut grid = Self::transpose(grid);
        Self::reverse_rows(&mut grid);
        Self::tilt_lever(&mut grid);
        Self::reverse_rows(&mut grid);
        Self::transpose(&grid)
    }

    fn tilt_east(grid: &mut Grid) {
        Self::reverse_rows(grid);
        Self::tilt_lever(grid);
        Self::reverse_rows(grid);
    }

    fn cycle(grid: &Grid) -> Grid {
        let mut grid = Self::tilt_north(grid);
        Self::tilt_lever(&mut grid);
        grid = Self::tilt_south(&grid);
        Self::tilt_east(&mut grid);
        grid
    }

    fn get_load(grid: &Grid) -> usize {
        let n_rows = grid.len();

        #[allow(clippy::naive_bytecount)]
        grid
            .iter()
            .enumerate()
            .map(|(i, row)|
                row
                    .iter()
                    .filter(|&&tile| tile == b'O')
                    .count()
                * (n_rows - i)
            )
            .sum()
    }

    fn get_grid<T: Display>(inp: T) -> Grid {
        inp
            .to_string()
            .lines()
            .map(|line| line
                .as_bytes()
                .to_vec()
            )
            .collect::<Grid>()
    }

    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        Self::get_load(
            &Self::tilt_north(
                &Self::get_grid(inp)
            )
        )
    }

    /// # Panics
    ///
    /// If the cycles vec is empty
    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        let grid = Self::get_grid(inp);

        let mut cycles = VecDeque::from([grid]);
        let start = loop {
            let next_term = Self::cycle(cycles
                .back()
                .unwrap()
            );

            if let Some(index) = cycles
                .iter()
                .position(|term| term == &next_term)
            {
                break index;
            }
            cycles.push_back(next_term);
        };

        Self::get_load(
            &cycles[
                (1_000_000_000 - start)
                % (cycles.len() - start)
                + start
            ]
        )
    }
}

impl Solution for Day14 {
    const NAME: &'static str = "Parabolic Reflector Dish";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 111_339);
        assert_eq!(p2, 93736);
    }
}

fn main() {
    aoc_2023::run_day(14, &Day14);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}