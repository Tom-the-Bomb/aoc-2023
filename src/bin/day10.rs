//! Day 10: Pipe Maze
//!
//! <https://adventofcode.com/2023/day/10>
use std::{
    collections::{
        VecDeque,
        HashSet,
        HashMap,
    },
    fmt::Display,
};
use aoc_2023::Solution;

static GO_LEFT_PIPES: [u8; 3] =
    [b'-', b'J', b'7'];
static GO_RIGHT_PIPES: [u8; 3] =
    [b'-', b'L', b'F'];
static GO_UP_PIPES: [u8; 3] =
    [b'|', b'J', b'L'];
static GO_DOWN_PIPES: [u8; 3] =
    [b'|', b'7', b'F'];

pub struct Day10;

impl Day10 {
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

    fn get_loop(grid: &[Vec<u8>]) -> HashSet<(usize, usize)> {
        let starting_coords = Self::get_starting_pos(grid);

        let mut nodes = HashSet::from([starting_coords]);
        let mut to_check = VecDeque::from([starting_coords]);

        while let Some((curr_row, curr_col)) = to_check.pop_front() {
            let curr_tile = grid[curr_row][curr_col];

            let neighbors = [
                (curr_row.wrapping_sub(1), curr_col,
                    GO_UP_PIPES,
                    GO_DOWN_PIPES,
                ),
                (curr_row + 1, curr_col,
                    GO_DOWN_PIPES,
                    GO_UP_PIPES,
                ),
                (curr_row, curr_col.wrapping_sub(1),
                    GO_LEFT_PIPES,
                    GO_RIGHT_PIPES,
                ),
                (curr_row, curr_col + 1,
                    GO_RIGHT_PIPES,
                    GO_LEFT_PIPES,
                ),
            ];
            for (next_row, next_col, pipes, co_pipes) in neighbors {
                if let Some(next_tile) = grid
                    .get(next_row)
                    .and_then(|row| row.get(next_col))
                {
                    let next_coord = (next_row, next_col);
                    if (pipes.contains(&curr_tile)
                        || curr_tile == b'S')
                        && co_pipes.contains(next_tile)
                        && !nodes.contains(&next_coord)
                    {
                        nodes.insert(next_coord);
                        to_check.push_back(next_coord);
                    }
                }
            }
        }
        nodes
    }

    /// Reformats the grid using unicode characters to help better visualize the pipes
    ///
    /// All pipes that are not part of the loop are replaced with a "."
    pub fn display_grid<T: Display>(&self, inp: T) -> String {
        let mapping = HashMap::from([
            (b'|', '│'),
            (b'-', '─'),
            (b'J', '┘'),
            (b'7', '┐'),
            (b'L', '└'),
            (b'F', '┌'),
        ]);
        let grid = Self::get_grid(inp);
        let nodes = Self::get_loop(&grid);

        let formatted = grid
            .iter()
            .enumerate()
            .map(|(i, row)| format!(
                "{i:>3}| {}",
                row
                    .iter()
                    .enumerate()
                    .map(|(j, tile)|
                        if nodes.contains(&(i, j)) {
                            *mapping.get(tile)
                                .unwrap_or(&(*tile as char))
                        }
                        else { '.' }
                        .to_string()
                    )
                    .collect::<String>()
                )
            )
            .collect::<Vec<String>>()
            .join("\n");

        format!("\n{formatted}\n")
    }
}

impl Solution for Day10 {
    const NAME: &'static str = "Pipe Maze";

    fn part_one<T: Display>(&self, inp: T) -> usize {
        Self::get_loop(
            &Self::get_grid(inp)
        )
        .len() / 2
    }

    fn part_two<T: Display>(&self, inp: T) -> usize {
        let grid = Self::get_grid(inp);
        let nodes = Self::get_loop(&grid);

        let mut area = 0;
        for (i, row) in grid
            .into_iter()
            .enumerate()
        {
            let mut downwards = 0;
            let mut upwards = 0;

            for (j, tile) in row
                .into_iter()
                .enumerate()
            {
                if nodes.contains(&(i, j)) {
                    if GO_DOWN_PIPES.contains(&tile) {
                        downwards += 1;
                    }
                    if GO_UP_PIPES.contains(&tile) {
                        upwards += 1;
                    }
                } else if downwards % 2 == 1 && upwards % 2 == 1 {
                    area += 1;
                }
            }
        }
        area
    }

    fn run(&self, inp: String) {
        println!("{}", self.display_grid(&inp));

        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 7063);
        assert_eq!(p2, 589);
    }
}

fn main() {
    aoc_2023::run_day(10, &Day10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}