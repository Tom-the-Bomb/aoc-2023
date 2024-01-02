//! Day 24: Step Counter
//!
//! <https://adventofcode.com/2023/day/24>
use std::{
    str::FromStr,
    fmt::Display,
};
use aoc_2023::Solution;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParseHailstoneError;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone {
    x_pos: f64,
    y_pos: f64,
    z_pos: f64,
    x_vel: f64,
    y_vel: f64,
    z_vel: f64,
}

macro_rules! parse_part {
    ($parts:expr) => {
        $parts
            .next()
            .and_then(|raw| raw
                .trim()
                .parse::<f64>()
                .ok()
            )
            .ok_or(ParseHailstoneError)?
    }
}

#[inline]
fn float_cmp(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

impl FromStr for Hailstone {
    type Err = ParseHailstoneError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = raw
            .split_once('@')
            .ok_or(ParseHailstoneError)?;
        let mut pos = pos.split(',');
        let mut vel = vel.split(',');

        Ok(Self {
            x_pos: parse_part!(pos),
            y_pos: parse_part!(pos),
            z_pos: parse_part!(pos),

            x_vel: parse_part!(vel),
            y_vel: parse_part!(vel),
            z_vel: parse_part!(vel),
        })
    }
}

impl Hailstone {
    #[inline]
    fn m(&self) -> f64 {
        self.y_vel / self.x_vel
    }

    #[inline]
    fn b(&self) -> f64 {
        self.m()
            .mul_add(-self.x_pos, self.y_pos)
    }

    #[inline]
    fn evaluate(&self, x: f64) -> f64 {
        self.m()
            .mul_add(x, self.b())
    }

    #[inline]
    fn in_domain(&self, x: f64, y: f64) -> bool {
        (if self.x_vel > 0.0 {
            x >= self.x_pos
        } else if float_cmp(self.x_vel, 0.0) {
            float_cmp(x, self.x_pos)
        } else {
            x <= self.x_pos
        })
        && (if self.y_vel > 0.0 {
            y >= self.y_pos
        } else if float_cmp(self.x_vel, 0.0) {
            float_cmp(y, self.y_pos)
        } else {
            y <= self.y_pos
        })
    }

    #[inline]
    fn intersection(&self, other: &Self) -> Option<(f64, f64)> {
        let m_diff = self.m() - other.m();
        (m_diff != 0.0)
            .then(|| {
                let x = (other.b() - self.b()) / m_diff;
                (x, self.evaluate(x))
            })
    }
}

pub struct Day24;

impl Day24 {
    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        let hailstones = inp
            .to_string()
            .lines()
            .filter_map(|line| line.parse::<Hailstone>().ok())
            .collect::<Vec<Hailstone>>();

        hailstones
            .iter()
            .enumerate()
            .flat_map(|(i, hs1)| hailstones
                .iter()
                .take(i)
                .filter(|hs2| hs1
                    .intersection(hs2)
                    .map(|(x, y)|
                        hs1.in_domain(x, y)
                        && hs2.in_domain(x, y)
                        && (200_000_000_000_000.0..=400_000_000_000_000.0).contains(&x)
                        && (200_000_000_000_000.0..=400_000_000_000_000.0).contains(&y)
                    )
                    .unwrap_or_default()
                )
            )
            .count()
    }

    pub fn part_two<T: Display>(&self, _inp: T) -> usize {
        0
    }
}

impl Solution for Day24 {
    const NAME: &'static str = "Step Counter";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 14672);
        assert_eq!(p2, 646_810_057_104_753);
    }
}

fn main() {
    aoc_2023::run_day(24, &Day24);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}