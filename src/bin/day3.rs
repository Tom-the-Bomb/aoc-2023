//! Day 3: Gear Ratios
//!
//! <https://adventofcode.com/2023/day/3>
use std::{
    collections::HashMap,
    fmt::Display,
};
use aoc_2023::Solution;

pub struct Day3;

impl Day3 {
    fn symbol_adjacent<F, A>(
        arr: &[Vec<char>],
        coordinates: A,
        condition: F,
    ) -> Vec<(usize, usize)>
    where
        F: Fn(char) -> bool,
        A: AsRef<[(usize, usize)]>,
    {
        let coordinates = coordinates.as_ref();
        let n_coords = coordinates.len() - 1;

        coordinates
            .iter()
            .enumerate()
            .flat_map(|(i, (row, col))| {
                let mut indices = vec![
                    (row.wrapping_sub(1), *col),
                    (*row + 1, *col),
                ];
                if i == 0 {
                    indices.extend([
                        (*row, col.wrapping_sub(1)),
                        (row.wrapping_sub(1), col.wrapping_sub(1)),
                        (*row + 1, col.wrapping_sub(1)),
                    ]);
                }
                if i == n_coords {
                    indices.extend([
                        (*row, *col + 1),
                        (row.wrapping_sub(1), *col + 1),
                        (*row + 1, *col + 1),
                    ]);
                }
                indices
                    .into_iter()
                    .filter_map(|(y, x)|
                        arr.get(y)
                            .and_then(|row| row
                                .get(x)
                                .and_then(|c| condition(*c)
                                    .then_some((y, x))
                                )
                            )
                    )
            })
            .collect()
    }

    /// # Panics
    ///
    /// If a number string in the input somehow is unable to be parsed into [`u32`]
    pub fn part_one<T: Display>(&self, inp: T) -> u32 {
        let arr = inp
            .to_string()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let mut total = 0;

        let mut curr_indices = Vec::new();
        let mut curr_num = String::new();
        for (y, row) in arr
            .iter()
            .enumerate()
        {
            for (x, chr) in row
                .iter()
                .enumerate()
            {
                if chr.is_numeric() {
                    curr_indices.push((y, x));
                    curr_num.push(*chr);
                } else {
                    if !curr_indices.is_empty()
                        && !Self::symbol_adjacent(
                            &arr, &curr_indices,
                            |c| !c.is_numeric() && c != '.'
                        ).is_empty()
                    {
                        total += curr_num.parse::<u32>()
                            .unwrap();
                    }
                    curr_indices.clear();
                    curr_num.clear();
                }
            }
        }
        total
    }

    /// # Panics
    ///
    /// if the numbers failed to be parsed into [`u32`]
    pub fn part_two<T: Display>(&self, inp: T) -> u32 {
        let arr = inp
            .to_string()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let mut total = 0;

        for (y, row) in arr
            .iter()
            .enumerate()
        {
            for (x, chr) in row
                .iter()
                .enumerate()
            {
                if *chr == '*' {
                    let nums = Self::symbol_adjacent(
                        &arr, [(y, x)],
                        char::is_numeric
                    );
                    if !nums.is_empty() {
                        let mut num_map = HashMap::new();
                        for (y, mut x) in nums {
                            let mut curr_num = String::new();

                            while arr.get(y)
                                .and_then(|row| row.get(x.wrapping_sub(1))
                                    .filter(|c| c.is_numeric())
                                )
                                .is_some()
                            {
                                x -= 1;
                            }
                            while let Some(chr) = arr.get(y)
                                .and_then(|row| row.get(x)
                                    .filter(|c| c.is_numeric())
                                )
                            {
                                curr_num.push(*chr);
                                x += 1;
                            }
                            num_map.insert((y, x), curr_num);
                        }
                        if num_map.len() == 2 {
                            let mut values = num_map.values();
                            total += values.next()
                                .unwrap()
                                .parse::<u32>()
                                .unwrap()
                                * values.next()
                                .unwrap()
                                .parse::<u32>()
                                .unwrap();
                        }
                    }
                }
            }
        }
        total
    }
}

impl Solution for Day3 {
    const NAME: &'static str = "Gear Ratios";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 532_428) ;
        assert_eq!(p2, 84_051_670);
    }
}

fn main() {
    aoc_2023::run_day(3, &Day3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}