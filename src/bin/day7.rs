//! Day 7: Camel Cards
//!
//! <https://adventofcode.com/2023/day/7>
use std::{collections::HashSet, fmt::Display};
use aoc_2023::Solution;

pub struct Day7;

static CARDS: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

impl Day7 {
    fn get_hand_strength<T, A>(hand: T, jokers: Option<A>) -> Vec<isize>
    where
        T: AsRef<str>,
        A: AsRef<[usize]>,
    {
        let jokers = jokers
            .as_ref()
            .map(|a| a.as_ref())
            .unwrap_or(&[]);
        let hand = hand
            .as_ref();
        let counter = hand
            .chars()
            .collect::<HashSet<char>>()
            .into_iter()
            .map(|card| hand
                .chars()
                .filter(|c| *c == card)
                .count()
            )
            .collect::<Vec<usize>>();
        let points = match (counter.iter().max(), counter.len()) {
            (Some(5), _) => 6,
            (Some(4), _) => 5,
            (Some(3), 2) => 4,
            (Some(3), 3) => 3,
            (Some(2), 3) => 2,
            (Some(2), 4) => 1,
            _ => 0,
        };
        let mut strength = vec![points];
        strength.extend(hand
            .chars()
            .enumerate()
            .map(|(i, card)|
                if jokers.contains(&i) { -1 }
                else {
                    CARDS
                        .iter()
                        .position(|r| *r ==  card)
                        .unwrap_or_default()
                        as isize
                }
            )
        );
        strength
    }

    fn get_hand_strength_joker<T>(hand: T) -> Vec<isize>
    where
        T: AsRef<str>
    {
        let hand = hand
            .as_ref();
        todo!()
    }

    /// Non brute force part 1
    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        todo!()
    }

    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        todo!()
    }
}

impl Solution for Day7 {
    const NAME: &'static str = "Camel Cards";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 1_731_600);
        assert_eq!(p2, 40_087_680);
    }
}

fn main() {
    aoc_2023::run_day(7, &Day7);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}