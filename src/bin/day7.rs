//! Day 7: Camel Cards
//!
//! <https://adventofcode.com/2023/day/7>
use std::{collections::HashSet, fmt::Display};
use aoc_2023::Solution;

pub struct Day7;

static CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'
];

impl Day7 {
    fn get_hand_strength<T, A>(hand: T, jokers: &Option<A>) -> Vec<isize>
    where
        T: AsRef<str>,
        A: AsRef<[usize]>,
    {
        #[allow(clippy::map_unwrap_or)]
        let jokers = jokers
            .as_ref()
            .map(AsRef::as_ref)
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
        let mut strength = vec![
            match (counter.iter().max(), counter.len()) {
                (Some(5), _) => 6,
                (Some(4), _) => 5,
                (Some(3), 2) => 4,
                (Some(3), 3) => 3,
                (Some(2), 3) => 2,
                (Some(2), 4) => 1,
                _ => 0,
            }
        ];
        #[allow(clippy::cast_possible_wrap)]
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
        if hand == "JJJJJ" {
            Self::get_hand_strength(
                "AAAAA",
                &Some((0..5)
                    .collect::<Vec<usize>>())
            )
        } else {
            let mut buf = [0; 4];

            Self::get_hand_strength(
                hand.replace('J',
                    hand
                        .chars()
                        .filter(|card| *card != 'J')
                        .max_by_key(|card| hand
                            .chars()
                            .filter(|c| c == card)
                            .count()
                        )
                        .unwrap()
                        .encode_utf8(&mut buf)
                ),
                &Some(hand
                    .chars()
                    .enumerate()
                    .filter_map(|(i, card)|
                        (card == 'J').then_some(i)
                    )
                    .collect::<Vec<usize>>(),
                ),
            )
        }
    }

    /// # Panics
    ///
    /// If the hand and bid amount cannot be parsed from a line
    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        let hands = inp
            .to_string();
        let mut hands = hands
            .lines()
            .map(|line| {
                let (hand, bid) = line
                    .split_once(' ')
                    .unwrap();
                (hand, bid.parse::<usize>().unwrap())
            })
            .collect::<Vec<(&str, usize)>>();

        hands
            .sort_by_key(|(hand, _)| Self::get_hand_strength(hand, &None::<&[usize]>));
        hands
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum()
    }

    /// # Panics
    ///
    /// If the hand and bid amount cannot be parsed from a line
    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        let hands = inp
            .to_string();
        let mut hands = hands
            .lines()
            .map(|line| {
                let (hand, bid) = line
                    .split_once(' ')
                    .unwrap();
                (hand, bid.parse::<usize>().unwrap())
            })
            .collect::<Vec<(&str, usize)>>();

        hands
            .sort_by_key(|(hand, _)| Self::get_hand_strength_joker(hand));
        hands
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum()
    }
}

impl Solution for Day7 {
    const NAME: &'static str = "Camel Cards";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 253_313_241);
        assert_eq!(p2, 253_362_743);
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