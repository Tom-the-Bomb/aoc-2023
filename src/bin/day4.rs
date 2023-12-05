use std::{
    collections::HashSet,
    fmt::Display,
};
use aoc_2023::Solution;

pub struct Day4;

impl Day4 {
    /// returns the amount of the numbers that are winning numbers
    /// 
    /// # Panics
    /// 
    /// If the amount of winning numbers exceeds [`usize::MAX`]
    fn get_winning_amt<T>(card: T) -> u32
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
            .map(str::to_string)
            .collect::<HashSet<String>>()
            .intersection(
                &mine
                    .split_whitespace()
                    .map(str::to_string)
                    .collect::<HashSet<String>>()
            )
            .count()
            .try_into()
            .unwrap()
    }
    
    pub fn part_one<T: Display>(&self, inp: T) -> u32 {
        inp
            .to_string()
            .lines()
            .map(|card| {
                let amt_win = Self::get_winning_amt(card);
                if amt_win > 0 {
                    (2u32).pow(amt_win  - 1)
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
            let win_amt = Self::get_winning_amt(card) as usize;

            for card in i + 1..=win_amt + i {
                copies[card] += copies[i];
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