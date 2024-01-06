//! Day 18: Lavaduct Lagoon
//!
//! <https://adventofcode.com/2023/day/18>
use std::{
    iter::once,
    fmt::Display,
};
use aoc_2023::Solution;

pub struct Day18;

impl Day18 {
    /// Shoelace formula to find interior area
    fn shoelace<T>(points: T) -> usize
    where
        T: AsRef<[(isize, isize)]>
    {
        let points = points.as_ref();

        (points
            .iter()
            .zip(
                points
                    .iter()
                    .skip(1)
                    .chain(once(&points[0]))
            )
            .map(|((x1, y1), (x2, y2))|
                x1 * y2 - x2 * y1
            )
            .sum::<isize>() / 2
        )
        .unsigned_abs()
    }

    /// Uses shoelace formula + Pick's theorem to find the total area
    ///
    /// `A + b/2 + 1 = i + b`
    /// where A = shoelace result
    ///       b = perimeter
    ///       i + b = desired result
    ///
    /// # Panics
    ///
    /// If the vector of points is empty
    fn get_area<T>(data: T) -> usize
    where
        T: Iterator<Item = (usize, (isize, isize))>
    {
        let (low, high) = data.size_hint();
        let mut points = Vec::with_capacity(
            high.unwrap_or(low) + 1
        );
        points.push((0, 0));

        let mut perimeter = 0;

        for (dist, (dir_x, dir_y)) in data {
            let &(last_x, last_y) = points
                .last()
                .unwrap();

            #[allow(clippy::cast_possible_wrap)]
            points.push((
                last_x + dir_x * dist as isize,
                last_y + dir_y * dist as isize,
            ));
            perimeter += dist;
        }
        Self::shoelace(&points) + perimeter / 2 + 1
    }
}

impl Solution for Day18 {
    const NAME: &'static str = "Lavaduct Lagoon";

    /// # Panics
    ///
    /// If failed to parse input numbers
    fn part_one<T: Display>(&self, inp: T) -> usize {
        let inp = inp.to_string();
        let data = inp
            .lines()
            .map(|line| {
                let mut parts = line
                    .split_whitespace();
                let direction =
                    match parts.next()
                {
                    Some("U") => (0, 1),
                    Some("D") => (0, -1),
                    Some("L") => (-1, 0),
                    Some("R") => (1, 0),
                    _ => (0, 0),
                };
                let dist = parts
                    .next()
                    .and_then(|dist| dist.parse::<usize>().ok())
                    .unwrap();
                (dist, direction)
            });
        Self::get_area(data)
    }

    /// # Panics
    ///
    /// If failed to parse input numbers
    fn part_two<T: Display>(&self, inp: T) -> usize {
        let inp = inp.to_string();
        let data = inp
            .lines()
            .map(|line| {
                let hexcode = line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .trim_matches(|c| ['(', '#', ')'].contains(&c));
                let (dist, direction) = hexcode
                    .split_at(hexcode.len() - 1);
                (
                    usize::from_str_radix(dist, 16)
                        .unwrap(),
                    [
                        (1, 0),
                        (0, -1),
                        (-1, 0),
                        (0, 1),
                    ][direction
                        .parse::<usize>()
                        .unwrap()
                    ]
                )
            });
        Self::get_area(data)
    }

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 61_865);
        assert_eq!(p2, 40_343_619_199_142);
    }
}

fn main() {
    aoc_2023::run_day(18, &Day18);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}