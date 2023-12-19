//! Day 17: Clumsy Crucible
//!
//! <https://adventofcode.com/2023/day/17>
use std::{
    collections::{BinaryHeap, HashSet},
    cmp::Reverse,
    fmt::Display,
};
use aoc_2023::Solution;

pub struct Day17;

static ALL_DIRECTIONS: [(i8, i8); 4] =
    [(0, -1), (0, 1), (-1, 0), (1, 0)];

impl Day17 {
    /// # Panics
    ///
    /// If the grid is empty for some reason
    fn find_path<T: Display>(inp: T, is_part_two: bool) -> Option<u32> {
        let grid = inp
            .to_string()
            .lines()
            .map(|line| line
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
            )
            .collect::<Vec<Vec<u32>>>();

        let n_rows = grid.len();
        let n_cols = grid
            .first()
            .unwrap()
            .len();
        let mut traversed = HashSet::new();
        let mut to_check = BinaryHeap::from([
            Reverse((0u32, 0u8, (0, 0), (0, 0)))
        ]);
        let max_dir_traversed = if is_part_two { 10 } else { 3 };

        while let Some(Reverse((
            heat, dir_traversed, (row, col), (row_incr, col_incr)
        ))) =
            to_check.pop()
        {
            let set_entry = (dir_traversed, (row, col), (row_incr, col_incr));

            if row == n_rows - 1
                && col == n_cols - 1
                && if is_part_two { dir_traversed >= 4 } else { true }
            {
                return Some(heat);
            }

            if !traversed.contains(&set_entry) {
                let mut directions = Vec::with_capacity(3);

                if if is_part_two {
                    dir_traversed >= 4
                    || row_incr == 0
                    && col_incr == 0
                } else { true }
                {
                    directions.extend(ALL_DIRECTIONS
                        .iter()
                        .filter_map(|&(new_row_incr, new_col_incr)|
                            (
                                (new_row_incr != row_incr
                                    || new_col_incr != col_incr)
                                && (new_row_incr != -row_incr
                                    || new_col_incr != -col_incr)
                            )
                            .then_some(((new_row_incr, new_col_incr), true))
                        )
                    );
                }

                if dir_traversed < max_dir_traversed
                    && (row_incr != 0 || col_incr != 0)
                {
                    directions.push(((row_incr, col_incr), false));
                }

                #[allow(clippy::cast_sign_loss)]
                for ((row_incr, col_incr), changed_directions) in directions {
                    let new_row = row.wrapping_add(row_incr as usize);
                    let new_col = col.wrapping_add(col_incr as usize);

                    if let Some(&new_heat) = grid
                        .get(new_row)
                        .and_then(|row| row.get(new_col))
                    {
                        to_check.push(Reverse((
                            heat + new_heat,
                            if changed_directions { 1 } else { dir_traversed + 1 },
                            (new_row, new_col),
                            (row_incr, col_incr),
                        )));
                    }
                }
                traversed.insert(set_entry);
            }
        }
        None
    }

    /// # Panics
    ///
    /// If no paths to the end are found (not possible)
    pub fn part_one<T: Display>(&self, inp: T) -> u32 {
        Self::find_path(inp, false)
            .expect("No paths found")
    }

    /// # Panics
    ///
    /// If no paths to the end are found (not possible)
    pub fn part_two<T: Display>(&self, inp: T) -> u32 {
        Self::find_path(inp, true)
            .expect("No paths found")
    }
}

impl Solution for Day17 {
    const NAME: &'static str = "Clumsy Crucible";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 724);
        assert_eq!(p2, 877);
    }
}

fn main() {
    aoc_2023::run_day(17, &Day17);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}