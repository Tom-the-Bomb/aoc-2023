//! Day 22: Sand Slabs
//!
//! <https://adventofcode.com/2023/day/22>
use std::{
    collections::{HashMap, HashSet, VecDeque},
    cmp::Ordering,
    fmt::Display,
    str::FromStr,
};
use aoc_2023::Solution;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsePointError;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseBrickError {
    ParsePointError(ParsePointError),
    MissingTilde,
}

impl From<ParsePointError> for ParseBrickError {
    fn from(err: ParsePointError) -> Self {
        Self::ParsePointError(err)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let mut parts = raw.split(',');
        Ok(Self {
            x: parts.next()
                .and_then(|x| x.parse::<usize>().ok())
                .ok_or(ParsePointError)?,
            y: parts.next()
                .and_then(|y| y.parse::<usize>().ok())
                .ok_or(ParsePointError)?,
            z: parts.next()
                .and_then(|z| z.parse::<usize>().ok())
                .ok_or(ParsePointError)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Brick {
    bottom: Point,
    top: Point,
}

impl FromStr for Brick {
    type Err = ParseBrickError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let (bottom, top) = raw
            .split_once('~')
            .ok_or(ParseBrickError::MissingTilde)?;
        Ok(Self {
            bottom: Point::from_str(bottom)?,
            top: Point::from_str(top)?,
        })
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bottom.z.cmp(&other.bottom.z)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Brick {
    #[inline]
    const fn height(&self) -> usize {
        self.top.z - self.bottom.z
    }

    #[inline]
    fn overlaps_xy(&self, other: &Self) -> bool {
        self.bottom.x.max(other.bottom.x) <= self.top.x.min(other.top.x)
        && self.bottom.y.max(other.bottom.y) <= self.top.y.min(other.top.y)
    }
}

pub struct Day22;

impl Day22 {
    #[allow(clippy::type_complexity)]
    fn get_support_mappings<T: Display>(inp: T) -> (
        Vec<Brick>,
        HashMap<usize, HashSet<usize>>,
        HashMap<usize, HashSet<usize>>,
    ) {
        let mut bricks = inp
            .to_string()
            .lines()
            .filter_map(|line| line.parse::<Brick>().ok())
            .collect::<Vec<Brick>>();
        bricks.sort();

        for i in 0..bricks.len() {
            let brick = &bricks[i];
            let mut z = 1;

            for lower_brick in bricks
                .iter()
                .take(i)
            {
                if brick.overlaps_xy(lower_brick) {
                    z = z.max(lower_brick.top.z + 1);
                }
            }
            bricks[i].top.z = brick.height() + z;
            bricks[i].bottom.z = z;
        }
        bricks.sort();

        let mut supports = HashMap::new();
        let mut supported_by = HashMap::new();

        for (a, brick) in bricks
            .iter()
            .enumerate()
        {
            for (b, lower_brick) in bricks
                .iter()
                .take(a)
                .enumerate()
            {
                if lower_brick.overlaps_xy(brick)
                    && brick.bottom.z == lower_brick.top.z + 1
                {
                    supports
                        .entry(b)
                        .or_insert_with(HashSet::new)
                        .insert(a);
                    supported_by
                        .entry(a)
                        .or_insert_with(HashSet::new)
                        .insert(b);
                }
            }
        }
        (bricks, supports, supported_by)
    }

    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        let (bricks, supports, supported_by) = Self::get_support_mappings(inp);

        (0..bricks.len())
            .filter(|b| supports
                .get(b)
                .map_or(true, |b_supports|
                    b_supports
                        .iter()
                        .all(|a| supported_by
                            .get(a)
                            .map(|supports_a| supports_a.len() > 1)
                            .unwrap_or_default()
                        )
                )
            )
            .count()
    }

    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        let (bricks, supports, supported_by) = Self::get_support_mappings(inp);
        let mut total = 0;

        for b in 0..bricks.len() {
            if let Some(b_supports) = supports.get(&b) {

                let mut to_check = b_supports
                    .iter()
                    .filter(|a| supported_by
                        .get(a)
                        .map(|supports_a| supports_a.len() == 1)
                        .unwrap_or_default()
                    )
                    .copied()
                    .collect::<VecDeque<usize>>();

                let mut falling = to_check
                    .iter()
                    .copied()
                    .collect::<HashSet<usize>>();

                while let Some(to_fall) = to_check.pop_front() {
                    if let Some(to_fall_supports) = supports.get(&to_fall) {
                        for a in to_fall_supports {
                            if !falling.contains(a) && supported_by[&a].is_subset(&falling) {
                                to_check.push_back(*a);
                                falling.insert(*a);
                            }
                        }
                    }
                }
                total += falling.len();
            }
        }
        total
    }
}

impl Solution for Day22 {
    const NAME: &'static str = "Sand Slabs";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 459);
        assert_eq!(p2, 75784);
    }
}

fn main() {
    aoc_2023::run_day(22, &Day22);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}