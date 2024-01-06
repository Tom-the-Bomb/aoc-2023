//! Day 16: The Floor Will Be Lava
//!
//! <https://adventofcode.com/2023/day/16>
use std::{
    collections::{VecDeque, HashSet},
    fmt::Display
};
use aoc_2023::Solution;

pub struct Day16;

impl Day16 {
    fn get_grid<T: Display>(inp: T) -> Vec<Vec<u8>> {
        inp
            .to_string()
            .lines()
            .map(|line| line
                .as_bytes()
                .to_vec()
            )
            .collect::<Vec<Vec<u8>>>()
    }

    fn get_energized_amount(
        grid: &[Vec<u8>],
        starting_row: usize,
        starting_col: usize,
        starting_row_incr: i8,
        starting_col_incr: i8,
    ) -> usize
    {
        let mut energized = HashSet::new();
        let mut to_check = VecDeque::from([(
            (starting_row, starting_col),
            (starting_row_incr, starting_col_incr),
        )]);

        #[allow(clippy::cast_sign_loss)]
        while let Some((
            (mut row, mut col),
            (mut row_incr, mut col_incr)
        )) = to_check.pop_front()
        {
            row = row.wrapping_add(row_incr as usize);
            col = col.wrapping_add(col_incr as usize);

            if let Some(&tile) = grid
                .get(row)
                .and_then(|row| row.get(col))
            {
                let directions =
                    if tile == b'-'
                        && row_incr != 0
                    {
                        vec![(0, -1), (0, 1)]
                    } else if tile == b'|'
                        && col_incr != 0
                    {
                        vec![(-1, 0), (1, 0)]
                    } else {
                        (row_incr, col_incr) = match tile {
                            b'/' => (-col_incr, -row_incr),
                            b'\\' => (col_incr, row_incr),
                            _ => (row_incr, col_incr),
                        };
                        vec![(row_incr, col_incr)]
                    };

                for direction in directions {
                    let entry = ((row, col), direction);
                    if !energized.contains(&entry) {
                        energized.insert(entry);
                        to_check.push_back(entry);
                    }
                }
            }
        }

        energized
            .into_iter()
            .map(|(coords, _)| coords)
            .collect::<HashSet<(usize, usize)>>()
            .len()
    }
}

impl Solution for Day16 {
    const NAME: &'static str = "The Floor Will Be Lava";

    fn part_one<T: Display>(&self, inp: T) -> usize {
        Self::get_energized_amount(
            &Self::get_grid(inp),
            0, usize::MAX, 0, 1,
        )
    }

    /// # Panics
    ///
    /// If the grid is empty
    fn part_two<T: Display>(&self, inp: T) -> usize {
        let grid = Self::get_grid(inp);

        let n_rows = grid.len();
        let n_cols = grid
            .first()
            .unwrap()
            .len();

        (0..n_rows)
            .map(|row|
                Self::get_energized_amount(
                    &grid,
                    row, usize::MAX, 0, 1
                )
                .max(
                    Self::get_energized_amount(
                        &grid,
                        row, n_cols, 0, -1
                    )
                )
            )
            .max()
            .and_then(|max_row|
                (0..n_cols)
                    .map(|col|
                        Self::get_energized_amount(
                            &grid,
                            usize::MAX, col, 1, 0
                        )
                        .max(
                            Self::get_energized_amount(
                                &grid,
                                n_rows, col, -1, 0
                            )
                        )
                    )
                    .max()
                    .map(|max_col| max_col.max(max_row))
            )
            .unwrap()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 7798);
        assert_eq!(p2, 8026);
    }
}

fn main() {
    aoc_2023::run_day(16, &Day16);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}