//! Day 6: Wait For It
//!
//! <https://adventofcode.com/2023/day/6>
use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day6;

impl Day6 {
    /// Brute forces getting the amount of cases where we beat the target distance
    /// by looping through all the possible values for the time the button is held,
    /// calculating the distance based on that and checking if it beats the target distance that we need to beat
    #[inline]
    fn get_num_beats_bf((time, to_beat): (usize, usize)) -> usize {
        (1..time)
            .filter(|hold_time| hold_time * (time - hold_time) > to_beat)
            .count()
    }

    /// Mathematical method of getting the amount of cases where we beat the target distance
    ///
    /// The function d(t) which models the distance travelled based on the time the button is held is a quadratic
    /// we simply need to solve the inequality were d(t) > [the time we need to beat]
    /// then we simply need to calculate the amount of integers in between the range of the inequality
    /// AKA the 2 roots of the function d(t) - [the time we need to beat]
    /// that number would be ⌊greater root⌋ - ⌈smaller root⌉ + 1
    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
    )]
    fn get_num_beats((time, to_beat): (usize, usize)) -> usize {
        let time = time as f64;
        let to_beat = to_beat as f64;
        let discriminant = 4.0_f64
            .mul_add(-to_beat, time.powi(2));

        if discriminant < 0.0 { 0 }
        else {
            let root1 = (time - discriminant.sqrt()) / 2.0;
            let root2 = (time + discriminant.sqrt()) / 2.0;
            (root2.trunc() - root1.ceil() + 1.0) as usize
        }
    }

    /// Helper function for Part 1 that executes the brute force and non brute force methods
    fn part_one_helper<T, F>(inp: T, map_func: F) -> usize
    where
        T: Display,
        F: Fn((usize, usize)) -> usize,
    {
        inp
            .to_string()
            .replace('\r', "")
            .split_once('\n')
            .map(|(time, distance)| (
                time.split_once(':')
                    .unwrap()
                    .1
                    .split_whitespace()
                    .filter_map(|n| n.parse::<usize>().ok())
                    .zip(
                        distance.split_once(':')
                            .unwrap()
                            .1
                            .split_whitespace()
                            .filter_map(|n| n.parse::<usize>().ok())
                    )
                    .map(map_func)
                    .product()
                )
            )
            .unwrap()
    }

    /// Helper function for Part 2 that executes the brute force and non brute force methods
    fn part_two_helper<T, F>(inp: T, map_func: F) -> usize
    where
        T: Display,
        F: Fn((usize, usize)) -> usize,
    {
        inp
            .to_string()
            .replace(|c| c == '\r' || c == ' ', "")
            .split_once('\n')
            .map(|(time, distance)|
                map_func((
                    time.split_once(':')
                        .unwrap()
                        .1
                        .parse::<usize>()
                        .unwrap(),
                    distance.split_once(':')
                        .unwrap()
                        .1
                        .parse::<usize>()
                        .unwrap(),
                ))
            )
            .unwrap()
    }

    /// Non brute force part 1
    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        Self::part_one_helper(inp, Self::get_num_beats)
    }

    /// Brute force part 1
    pub fn part_one_bf<T: Display>(&self, inp: T) -> usize {
        Self::part_one_helper(inp, Self::get_num_beats_bf)
    }

    /// Non brute force part 2
    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        Self::part_two_helper(inp, Self::get_num_beats)
    }

    /// Brute force part 2
    pub fn part_two_bf<T: Display>(&self, inp: T) -> usize {
        Self::part_two_helper(inp, Self::get_num_beats_bf)
    }
}

impl Solution for Day6 {
    const NAME: &'static str = "Wait For It";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        assert_eq!(self.part_one_bf(&inp), p1);
        assert_eq!(self.part_two_bf(&inp), p2);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 1_731_600);
        assert_eq!(p2, 40_087_680);
    }
}

fn main() {
    aoc_2023::run_day(6, &Day6);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}