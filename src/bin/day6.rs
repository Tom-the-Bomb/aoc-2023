use std::fmt::Display;
use aoc_2023::Solution;

pub struct Day6;

impl Day6 {
    #[inline]
    fn get_num_beats_bf((time, to_beat): (usize, usize)) -> usize {
        (1..time)
            .filter(|hold_time| hold_time * (time - hold_time) > to_beat)
            .sum()
    }

    fn get_num_beats((time, to_beat): (usize, usize)) -> usize {
        let time = time as f64;
        let to_beat = to_beat as f64;
        let discriminant = time.powf(2.0) - 4.0 * to_beat;

        if discriminant < 0.0 { 0 }
        else {
            let root1 = (time - discriminant.sqrt()) / 2.0;
            let root2 = (time + discriminant.sqrt()) / 2.0;
            (root2.trunc() - root1.ceil() + 1.0) as usize
        }
    }
    
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

    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        Self::part_one_helper(inp, Self::get_num_beats)
    }

    pub fn part_one_bf<T: Display>(&self, inp: T) -> usize {
        Self::part_one_helper(inp, Self::get_num_beats_bf)
    }

    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        Self::part_two_helper(inp, Self::get_num_beats)
    }

    pub fn part_two_bf<T: Display>(&self, inp: T) -> usize {
        Self::part_two_helper(inp, Self::get_num_beats_bf)
    }
}

impl Solution for Day6 {
    const NAME: &'static str = "Trebuchet!?";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 1731600);
        assert_eq!(p2, 40087680);
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