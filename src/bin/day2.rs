//! Day 2: Cube Conundrum
//!
//! <https://adventofcode.com/2023/day/2>
use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day2;

impl Solution for Day2 {
    const NAME: &'static str = "Cube Conundrum";

    /// # Panics
    ///
    /// If failed to parse each line's delimiters
    fn part_one<T: Display>(&self, inp: T) -> usize {
        inp.to_string()
            .lines()
            .map(|line| {
                let (game_id, plays) = line
                    .split_once(':')
                    .unwrap();
                let game_id = game_id
                    .trim_start_matches("Game ")
                    .parse::<usize>()
                    .unwrap();

                if plays
                    .split(';')
                    .all(|play| {
                        let mut red = 0;
                        let mut blue = 0;
                        let mut green = 0;

                        for color in play.splitn(3, ',') {
                            let (num, name) = color
                                .trim()
                                .split_once(' ')
                                .unwrap();
                            let num = num
                                .parse::<usize>()
                                .unwrap();
                            match name {
                                "red" => red += num,
                                "green" => green += num,
                                "blue" => blue += num,
                                _ => (),
                            }
                        }
                        red <= 12 && green <= 13 && blue <= 14
                    })
                { game_id } else { 0 }
            })
            .sum()
    }

    /// # Panics
    ///
    /// If failed to parse each line's delimiters
    fn part_two<T: Display>(&self, inp: T) -> usize {
        inp.to_string()
            .lines()
            .map(|line| {
                let (_, plays) = line.split_once(':')
                    .unwrap();
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                for color in plays
                    .replace(',', ";")
                    .split(';')
                {
                    let (num, name) = color
                        .trim()
                        .split_once(' ')
                        .unwrap();
                    let num = num
                        .parse::<usize>()
                        .unwrap();
                    match name {
                        "red" if num > red => red = num,
                        "green" if num > green => green = num,
                        "blue" if num > blue => blue = num,
                        _ => (),
                    }
                }
                red * green * blue
            })
            .sum()
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 2486);
        assert_eq!(p2, 87984);
    }
}

fn main() {
    aoc_2023::run_day(2, &Day2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}