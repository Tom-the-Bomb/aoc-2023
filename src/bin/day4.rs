//! Day 4: Scratchcards
//!
//! <https://adventofcode.com/2023/day/4>
use std::{
    collections::HashSet,
    fmt::Display,
};
use aoc_2023::Solution;

pub struct Day4;

impl Day4 {
    /// returns the amount of the numbers that are winning numbers on a card
    fn get_winning_amt<T>(card: T) -> usize
    where
        T: AsRef<str>,
    {
        let string = card.as_ref();
        let (_, nums) = string
            .split_once(':')
            .unwrap();
        let (winning, mine) = nums
            .split_once('|')
            .unwrap();

        winning
            .split_whitespace()
            .collect::<HashSet<&str>>()
            .intersection(
                &mine
                    .split_whitespace()
                    .collect::<HashSet<&str>>()
            )
            .count()
    }

    /// # Panics
    ///
    /// If the amount of winning numbers exceeds [`u32::MAX`]
    pub fn part_one<T: Display>(&self, inp: T) -> u32 {
        inp
            .to_string()
            .lines()
            .map(|card| {
                let amt_win = Self::get_winning_amt(card);
                if amt_win > 0 {
                    (2_u32).pow(
                        u32::try_from(amt_win)
                            .unwrap() - 1
                    )
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn part_two<T: Display>(&self, inp: T) -> u32 {
        let inp = inp.to_string();
        let mut copies = vec![1; inp.lines().count()];

        for (i, card) in inp
            .lines()
            .enumerate()
        {
            let win_amt = Self::get_winning_amt(card);

            for card in i + 1..=win_amt + i {
                let n_copies = *copies.get(i)
                    .unwrap_or(&0);
                if let Some(copy) = copies.get_mut(card) {
                    *copy += n_copies;
                }
            }
        }
        copies
            .into_iter()
            .sum()
    }
}

impl Solution for Day4 {
    const NAME: &'static str = "Scratchcards";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 26914);
        assert_eq!(p2, 13_080_971);
    }
}

fn main() {
    aoc_2023::run_day(4, &Day4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}