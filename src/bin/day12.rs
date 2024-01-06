//! Day 12: Hot Springs
//!
//! <https://adventofcode.com/2023/day/12>
use std::{
    collections::HashMap,
    fmt::Display
};
use aoc_2023::Solution;

pub struct Day12;

impl Day12 {
    fn get_arrangements<'a, T, C>(
        records: &'a T,
        criteria: &'a C,
        cache: &mut HashMap<(&'a [u8], &'a [usize]), usize>,
    ) -> usize
    where
        T: AsRef<[u8]> + ?Sized,
        C: AsRef<[usize]> + ?Sized,
    {
        let records = records
            .as_ref();
        let criteria = criteria
            .as_ref();

        let criteria_empty = criteria
            .is_empty();
        if records.is_empty() {
            return usize::from(criteria_empty)
        } else if criteria_empty {
            return usize::from(!records.contains(&b'#'))
        }

        if let Some(&val) = cache.get(&(records, criteria)) {
            return val;
        }

        let mut count = 0;
        if let Some(&first_record) = records.first() {
            if let Some(&first_criteria) = criteria.first() {
                if first_record == b'.'
                    || first_record == b'?'
                {
                    count += Self::get_arrangements(
                        records.get(1..)
                            .unwrap_or_default(),
                        criteria,
                        cache,
                    );
                }
                if (first_record == b'#'
                    || first_record == b'?')
                    && records.len() >= first_criteria
                    && !records[..first_criteria].contains(&b'.')
                    && (first_criteria == records.len()
                    || records[first_criteria] != b'#')
                {
                    count += Self::get_arrangements(
                        records.get(first_criteria + 1..)
                            .unwrap_or(&[]),
                        criteria.get(1..)
                            .unwrap_or_default(),
                        cache,
                    );
                }
            }
        }
        cache.insert((records, criteria), count);
        count
    }
}

impl Solution for Day12 {
    const NAME: &'static str = "Hot Springs";

    /// # Panics
    ///
    /// If unable to parse out the criteria and records from each line
    fn part_one<T: Display>(&self, inp: T) -> usize {
        inp
            .to_string()
            .lines()
            .map(|line| {
                let (records, criteria) = line
                    .split_once(' ')
                    .unwrap();
                Self::get_arrangements(
                    &records,
                    &criteria
                        .split(',')
                        .filter_map(|entry| entry.parse::<usize>().ok())
                        .collect::<Vec<usize>>(),
                    &mut HashMap::new(),
                )
            })
            .sum()
    }

    /// # Panics
    ///
    /// If unable to parse out the criteria and records from each line
    fn part_two<T: Display>(&self, inp: T) -> usize {
        inp
            .to_string()
            .lines()
            .map(|line| {
                let (records, criteria) = line
                    .split_once(' ')
                    .unwrap();
                Self::get_arrangements(
                    &[records.as_bytes()]
                        .repeat(5)
                        .join([b'?'].as_slice()),
                    &criteria
                        .split(',')
                        .filter_map(|entry| entry.parse::<usize>().ok())
                        .collect::<Vec<usize>>()
                        .repeat(5),
                    &mut HashMap::new(),
                )
            })
            .sum()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 7007);
        assert_eq!(p2, 3_476_169_006_222);
    }
}

fn main() {
    aoc_2023::run_day(12, &Day12);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}