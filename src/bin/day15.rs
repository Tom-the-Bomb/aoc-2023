//! Day 15: Lens Library
//!
//! <https://adventofcode.com/2023/day/15>
use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day15;

impl Day15 {
    /// Hashing algorithm
    /// turns any string into an 8-bit integer
    ///
    /// `((current_value + ascii_value(character)) * 17) % 256`
    fn hash<T>(string: T) -> u8
    where
        T: AsRef<str>
    {
        string
            .as_ref()
            .as_bytes()
            .iter()
            .fold(0u8, |acc, &next| acc
                .wrapping_add(next)
                .wrapping_mul(17)
            )
    }

    pub fn part_one<T: Display>(&self, inp: T) -> u32 {
        inp
            .to_string()
            .split(',')
            .map(|s| u32::from(Self::hash(s)))
            .sum()
    }

    /// # Panics
    ///
    /// If neither a '=' or '-' exist in an entry in the input
    pub fn part_two<T: Display>(&self, inp: T) -> u32 {
        let inp = inp.to_string();
        let mut boxes = vec![Vec::new(); 256];

        for string in inp
            .split(',')
        {
            let (label, focus) = string
                .split_once('=')
                .unwrap_or_else(||
                    string
                        .split_once('-')
                        .unwrap()
                );
            if let Some(map) =
                boxes.get_mut(Self::hash(label) as usize)
            {
                match (
                    // index of the entry (label, _) if exists
                    map
                        .iter()
                        .position(|(l, _)| *l == label),
                    focus.parse::<u32>()
                ) {
                    // label already exists, focus is a number
                    // update entry
                    (Some(i), Ok(focus)) => {
                        map[i] = (label, focus);
                    },
                    // label already exists, focus is none (needs to be removed; ending with '-')
                    // remove the entry
                    (Some(i), Err(_)) => {
                        map.remove(i);
                    },
                    // label does not exist, focus is a number
                    // create new entry
                    (None, Ok(focus)) => {
                        map.push((label, focus));
                    }
                    // last condition (label doesnt exist, and focus is none (needs to be removed; ending with '-')
                    // We cant remove a non existing entry anyways
                    // therefore this will never happen, so we do nothing
                    _ => (),
                }
            }
        }

        boxes
            .into_iter()
            .zip(1..)
            .flat_map(|(map, i)|
                map
                    .into_iter()
                    .zip(1..)
                    .map(move |((_, focus), j)| i * j * focus)
            )
            .sum()
    }
}

impl Solution for Day15 {
    const NAME: &'static str = "Lens Library";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 508_498);
        assert_eq!(p2, 279_116);
    }
}

fn main() {
    aoc_2023::run_day(15, &Day15);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}