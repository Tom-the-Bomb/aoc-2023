//! Day 21: Step Counter
//!
//! <https://adventofcode.com/2023/day/21>
use std::{
    collections::{VecDeque, HashSet},
    fmt::Display,
};
use aoc_2023::Solution;

pub struct Day21;

impl Day21 {
    fn get_starting_pos(grid: &[Vec<u8>]) -> (usize, usize) {
        for (i, row) in grid
            .iter()
            .enumerate()
        {
            if let Some(j) = row
                .iter()
                .position(|c| *c == b'S')
            {
                return (i, j)
            }
        }
        panic!("No 'S' character found in grid")
    }

    #[inline]
    fn get_grid<T: Display>(inp: T) -> Vec<Vec<u8>> {
        inp
            .to_string()
            .lines()
            .map(|row| row
                .as_bytes()
                .to_vec()
            )
            .collect::<Vec<Vec<u8>>>()
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    fn traverse(
        grid: &[Vec<u8>],
        (start_row, start_col): (usize, usize),
        steps: usize,
    ) -> usize {
        let start = (start_row as isize, start_col as isize);

        let mut to_check = VecDeque::from([
            (start, (0, 0), steps),
        ]);
        let mut traversed = HashSet::from([
            (start, (0, 0)),
        ]);
        let mut n_reached = 0;

        let n_rows = grid.len() as isize;
        let n_cols = grid
            .first()
            .unwrap()
            .len() as isize;

        while let Some(((row, col), (n_row_wraps, n_col_wraps), steps_left)) = to_check.pop_front() {
            if steps_left % 2 == 0 {
                n_reached += 1;
            }

            if steps_left > 0 {
                for (new_row, new_col) in [
                    (row, col + 1),
                    (row, col - 1),
                    (row + 1, col),
                    (row - 1, col),
                ] {
                    let (new_row_wraps, new_row) = (
                        new_row.div_euclid(n_rows) + n_row_wraps,
                        new_row.rem_euclid(n_rows)
                    );
                    let (new_col_wraps, new_col) = (
                        new_col.div_euclid(n_cols) + n_col_wraps,
                        new_col.rem_euclid(n_cols)
                    );

                    if let Some(&tile) = grid
                        .get(new_row as usize)
                        .and_then(|row| row.get(new_col as usize))
                    {
                        let pos = (
                            (new_row, new_col),
                            (new_row_wraps, new_col_wraps)
                        );
                        if tile != b'#' && !traversed.contains(&pos) {
                            to_check.push_back((
                                (new_row, new_col),
                                (new_row_wraps, new_col_wraps),
                                steps_left - 1,
                            ));
                            traversed.insert(pos);
                        }
                    }
                }
            }
        }
        n_reached
    }

    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        let grid = Self::get_grid(inp);
        Self::traverse(
            &grid,
            Self::get_starting_pos(&grid),
            64,
        )
    }

    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        let grid = Self::get_grid(inp);
        let start = Self::get_starting_pos(&grid);

        let n_rows = grid.len();
        let n = 26_501_365 / n_rows;

        let t1 = Self::traverse(&grid, start, start.0);
        let t2 = Self::traverse(&grid, start, start.0 + n_rows);
        let t3 = Self::traverse(&grid, start, start.0 + n_rows + n_rows);

        (n.pow(2) - n)
            * ((t1 + t3) / 2 - t2)
            + n * (t2 - t1)
            + t1
    }
}

impl Solution for Day21 {
    const NAME: &'static str = "Step Counter";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 3743);
        assert_eq!(p2, 618_261_433_219_147);
    }
}

fn main() {
    aoc_2023::run_day(21, &Day21);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}