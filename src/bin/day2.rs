#![allow(clippy::missing_panics_doc)]

use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day2;

impl Day2 {
    pub fn part_one<T: Display>(&self, inp: T) -> u32 {
        inp.to_string()
            .lines()
            .map(|line| {
                let (game_id, plays) = line
                    .split_once(':')
                    .unwrap();
                let game_id = game_id
                    .trim_start_matches("Game ")
                    .parse::<u32>()
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
                                .parse::<u32>()
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

    pub fn part_two<T: Display>(&self, inp: T) -> u32 {
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
                        .parse::<u32>()
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
}

impl Solution for Day2 {
    const NAME: &'static str = "Cube Conundrum";

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