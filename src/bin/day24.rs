//! Day 24: Never Tell Me The Odds
//!
//! <https://adventofcode.com/2023/day/24>
use std::{
    str::FromStr,
    fmt::Display,
};
use aoc_2023::Solution;

use astro_nalgebra::{
    num_traits::Zero,
    BigFloat,
    ConstCtx,
};
use nalgebra::{Matrix6, Matrix6x1};

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

type BF128 = BigFloat<ConstCtx<128>>;

#[derive(Debug, Clone, PartialEq)]
struct BFHailstone {
    x_pos: BF128,
    y_pos: BF128,
    z_pos: BF128,
    x_vel: BF128,
    y_vel: BF128,
    z_vel: BF128,
}

macro_rules! parse_part {
    ($parts:expr, $target_ty:ty) => {
        $parts
            .next()
            .and_then(|raw| raw
                .trim()
                .parse::<$target_ty>()
                .ok()
            )
            .ok_or(ParseHailstoneError)?
    }
}

macro_rules! impl_from_str {
    ($struct_ty:ty, $field_ty:ty) => {
        impl FromStr for $struct_ty {
            type Err = ParseHailstoneError;

            fn from_str(raw: &str) -> Result<Self, Self::Err> {
                let (pos, vel) = raw
                    .split_once('@')
                    .ok_or(ParseHailstoneError)?;
                let mut pos = pos.split(',');
                let mut vel = vel.split(',');

                Ok(Self {
                    x_pos: parse_part!(pos, $field_ty),
                    y_pos: parse_part!(pos, $field_ty),
                    z_pos: parse_part!(pos, $field_ty),

                    x_vel: parse_part!(vel, $field_ty),
                    y_vel: parse_part!(vel, $field_ty),
                    z_vel: parse_part!(vel, $field_ty),
                })
            }
        }
    }
}

impl_from_str!(Hailstone, f64);
impl_from_str!(BFHailstone, BF128);

#[inline]
fn float_cmp(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

impl Hailstone {
    #[inline]
    #[must_use]
    fn m(&self) -> f64 {
        self.y_vel / self.x_vel
    }

    #[inline]
    #[must_use]
    fn b(&self) -> f64 {
        self.m()
            .mul_add(-self.x_pos, self.y_pos)
    }

    #[inline]
    #[must_use]
    fn evaluate(&self, x: f64) -> f64 {
        self.m()
            .mul_add(x, self.b())
    }

    #[inline]
    #[must_use]
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
    #[must_use]
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
                        && (200_000_000_000_000.0..=400_000_000_000_000.0)
                            .contains(&x)
                        && (200_000_000_000_000.0..=400_000_000_000_000.0)
                            .contains(&y)
                    )
                    .unwrap_or_default()
                )
            )
            .count()
    }

    /// # Panics
    ///
    /// If failed to invert the `a` matrix
    /// or failed to parse input lines
    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        let inp = inp.to_string();
        let mut hailstones = inp
            .lines()
            .take(3)
            .filter_map(|line| line.parse::<BFHailstone>().ok());

        let hs1 = hailstones
            .next()
            .unwrap();
        let hs2 = hailstones
            .next()
            .unwrap();
        let hs3 = hailstones
            .next()
            .unwrap();

        let a = Matrix6::new(
            hs2.y_vel.clone() - hs1.y_vel.clone(),
            hs1.x_vel.clone() - hs2.x_vel.clone(), BF128::zero(),
            hs1.y_pos.clone() - hs2.y_pos.clone(),
            hs2.x_pos.clone() - hs1.x_pos.clone(), BF128::zero(),
            hs3.y_vel.clone() - hs1.y_vel.clone(),
            hs1.x_vel.clone() - hs3.x_vel.clone(), BF128::zero(),
            hs1.y_pos.clone() - hs3.y_pos.clone(),
            hs3.x_pos.clone() - hs1.x_pos.clone(), BF128::zero(),
            hs2.z_vel.clone() - hs1.z_vel.clone(), BF128::zero(),
            hs1.x_vel.clone() - hs2.x_vel.clone(),
            hs1.z_pos.clone() - hs2.z_pos.clone(), BF128::zero(),
            hs2.x_pos.clone() - hs1.x_pos.clone(),
            hs3.z_vel.clone() - hs1.z_vel.clone(), BF128::zero(),
            hs1.x_vel.clone() - hs3.x_vel.clone(),
            hs1.z_pos.clone() - hs3.z_pos.clone(), BF128::zero(),
            hs3.x_pos.clone() - hs1.x_pos.clone(),
            BF128::zero(), hs2.z_vel.clone() - hs1.z_vel.clone(),
            hs1.y_vel.clone() - hs2.y_vel.clone(), BF128::zero(),
            hs1.z_pos.clone() - hs2.z_pos.clone(),
            hs2.y_pos.clone() - hs1.y_pos.clone(),
            BF128::zero(), hs3.z_vel.clone() - hs1.z_vel.clone(),
            hs1.y_vel.clone() - hs3.y_vel.clone(), BF128::zero(),
            hs1.z_pos.clone() - hs3.z_pos.clone(),
            hs3.y_pos.clone() - hs1.y_pos.clone(),
        );

        let b = Matrix6x1::new(
            hs1.y_pos.clone() * hs1.x_vel.clone() - hs2.y_pos.clone() * hs2.x_vel.clone()
            - (hs1.x_pos.clone() * hs1.y_vel.clone() - hs2.x_pos.clone() * hs2.y_vel.clone()),

            hs1.y_pos.clone() * hs1.x_vel.clone() - hs3.y_pos.clone() * hs3.x_vel.clone()
            - (hs1.x_pos.clone() * hs1.y_vel.clone() - hs3.x_pos.clone() * hs3.y_vel.clone()),

            hs1.z_pos.clone() * hs1.x_vel.clone() - hs2.z_pos.clone() * hs2.x_vel
            - (hs1.x_pos.clone() * hs1.z_vel.clone() - hs2.x_pos * hs2.z_vel.clone()),

            hs1.z_pos.clone() * hs1.x_vel - hs3.z_pos.clone() * hs3.x_vel
            - (hs1.x_pos * hs1.z_vel.clone() - hs3.x_pos * hs3.z_vel.clone()),

            hs1.z_pos.clone() * hs1.y_vel.clone() - hs2.z_pos * hs2.y_vel
            - (hs1.y_pos.clone() * hs1.z_vel.clone() - hs2.y_pos * hs2.z_vel),

            hs1.z_pos * hs1.y_vel - hs3.z_pos * hs3.y_vel
            - (hs1.y_pos * hs1.z_vel - hs3.y_pos * hs3.z_vel),
        );

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        a
            .try_inverse()
            .map(|a_inverse| {
                let result = a_inverse * b;

                (result[0]
                    .as_f64()
                + result[1]
                    .as_f64()
                + result[2]
                    .as_f64()
                ) as usize
            })
            .unwrap()
    }
}

impl Solution for Day24 {
    const NAME: &'static str = "Never Tell Me The Odds";

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