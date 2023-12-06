//! Day 5: If You Give A Seed A Fertilizer
//!
//! <https://adventofcode.com/2023/day/5>
#![feature(iter_array_chunks)]

use std::{ops::Range, fmt::Display};
use aoc_2023::Solution;

pub struct Day5;

impl Day5 {
    /// Returns a list of pairs of:
    /// - the source range
    /// - the offset between the source and destination values
    ///
    /// # Panics
    ///
    /// If the almanac's mapping data fail to be parsed into [`i64`]
    fn get_lookup_table<T>(map: T) -> Vec<(Range<i64>, i64)>
    where
        T: AsRef<str>
    {
        map
            .as_ref()
            .lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.split_whitespace()
                    .filter_map(|part| part.parse::<i64>().ok());
                let dest_start = parts
                    .next()
                    .unwrap();
                let src_start = parts
                    .next()
                    .unwrap();
                let range = parts
                    .next()
                    .unwrap();
                (src_start..src_start + range, dest_start - src_start)
            })
            .collect::<Vec<_>>()
    }

    /// # Panics
    ///
    /// If somehow there are no lines in the data after the first
    pub fn part_one<T: Display>(&self, inp: T) -> i64 {
        let inp = inp
            .to_string()
            .replace('\r', "");
        let mut maps = inp
            .split("\n\n");
        let mut curr_data = maps
            .next()
            .map(|string| string
                .trim_start_matches("seeds:")
                .split_whitespace()
                .filter_map(|item| item.parse::<i64>().ok())
                .collect::<Vec<i64>>()
            )
            .unwrap();
        for map in maps {
            let table = Self::get_lookup_table(map);

            for item in &mut curr_data {
                *item = table.iter()
                    .find_map(|(map_range, diff)| map_range
                        .contains(item)
                        .then_some(*item + *diff)
                    )
                    .unwrap_or(*item);
            }
        }
        curr_data
            .into_iter()
            .min()
            .unwrap()
    }

    /// # Panics
    ///
    /// If somehow there are no lines in the data after the first
    pub fn part_two<T: Display>(&self, inp: T) -> i64 {
        let inp = inp
            .to_string()
            .replace('\r', "");
        let mut maps = inp
            .split("\n\n");
        let mut curr_data = maps
            .next()
            .map(|string|
                string
                    .trim_start_matches("seeds:")
                    .split_whitespace()
                    .filter_map(|item| item.parse::<i64>().ok())
                    .array_chunks::<2>()
                    .collect::<Vec<[i64; 2]>>()
            )
            .unwrap();

        for map in maps {
            let table = Self::get_lookup_table(map);
            let mut temp = Vec::new();

            'a: while let Some([a, mut b]) = curr_data.pop() {
                b += a;
                for (map_range, diff) in &table {
                    let isect_a = a.max(map_range.start);
                    let isect_b = b.min(map_range.end);

                    if isect_a < isect_b {
                        temp.push([isect_a + diff, isect_b - isect_a]);
                        if isect_a > a {
                            curr_data.push([map_range.start, isect_a - map_range.start]);
                        } else if b > isect_b {
                            curr_data.push([isect_b, b - isect_b]);
                        }
                        continue 'a;
                    }
                }
                temp.push([a, b - a]);
            }
            curr_data = temp;
        }
        curr_data
            .into_iter()
            .min()
            .unwrap()[0]
    }
}

impl Solution for Day5 {
    const NAME: &'static str = "If You Give A Seed A Fertilizer";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 993_500_720);
        assert_eq!(p2, 4_917_124);
    }
}

fn main() {
    aoc_2023::run_day(5, &Day5);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}