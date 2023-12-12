//! Day 10: Pipe Maze
//!
//! <https://adventofcode.com/2023/day/10>
use std::{
    collections::{VecDeque, HashSet},
    fmt::Display,
};
use aoc_2023::Solution;

static GO_LEFT_PIPES: [char; 3] =
    ['-', 'J', '7'];
static GO_RIGHT_PIPES: [char; 3] =
    ['-', 'L', 'F'];
static GO_UP_PIPES: [char; 3] =
    ['|', 'J', 'L'];
static GO_DOWN_PIPES: [char; 3] =
    ['|', '7', 'F'];

pub struct Day10;

impl Day10 {
    fn get_grid<T: Display>(inp: T) -> Vec<Vec<char>> {
        inp
            .to_string()
            .lines()
            .map(|row| row
                .chars()
                .collect::<Vec<char>>()
            )
            .collect::<Vec<Vec<char>>>()
    }

    fn get_starting_pos(grid: &[Vec<char>]) -> (usize, usize) {
        for (i, row) in grid
            .iter()
            .enumerate()
        {
            if let Some(j) = row
                .iter()
                .position(|c| *c == 'S')
            {
                return (i, j)
            }
        }
        panic!("No 'S' character found in grid")
    }

    fn get_loop(grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
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
                        || curr_tile == 'S')
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

    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        Self::get_loop(
            &Self::get_grid(inp)
        )
        .len() / 2
    }

    pub fn part_two<T: Display>(&self, inp: T) -> u32 {
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
}

impl Solution for Day10 {
    const NAME: &'static str = "Pipe Maze";

    fn run(&self, inp: String) {
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