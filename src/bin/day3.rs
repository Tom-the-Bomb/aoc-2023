use std::{collections::HashMap, fmt::Display};
use aoc_2023::Solution;

pub struct Day3;

impl Day3 {
    fn symbol_adjacent<F>(
        &self,
        arr: &Vec<Vec<char>>,
        coordinates: &Vec<(usize, usize)>,
        condition: F,
    ) -> Vec<(usize, usize)>
    where
        F: Fn(char) -> bool,
    {
        let ncoords = coordinates.len() - 1;

        coordinates
            .iter()
            .enumerate()
            .flat_map(|(i, (row, col))| {
                let mut indices = vec![
                    (*row - 1, *col),
                    (*row + 1, *col),
                ];
                if i == 0 {
                    indices.extend([
                        (*row, *col - 1),
                        (*row - 1, *col - 1),
                        (*row + 1, *col - 1),
                    ]);
                }
                if i == ncoords {
                    indices.extend([
                        (*row, *col + 1),
                        (*row - 1, *col + 1),
                        (*row + 1, *col + 1),
                    ]);
                }
                indices
                    .into_iter()
                    .filter_map(|(y, x)|
                        arr.get(y)
                            .and_then(|row| row
                                .get(x)
                                .and_then(|c| condition(*c)
                                    .then_some((y, x))
                                )
                            )
                    )
            })
            .collect()
    }

    pub fn part_one<T: Display>(&self, inp: T) -> u32 {
        let arr = inp
            .to_string()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let mut total = 0;

        let mut curr_indices = Vec::new();
        let mut curr_num = String::new();
        for (y, row) in arr
            .iter()
            .enumerate()
        {
            for (x, chr) in row
                .iter()
                .enumerate()
            {
                if chr.is_numeric() {
                    curr_indices.push((y, x));
                    curr_num.push(*chr);
                } else {
                    if !curr_indices.is_empty()
                        && !self.symbol_adjacent(
                            &arr, &curr_indices, char::is_numeric
                        ).is_empty()
                    {
                        total += curr_num.parse::<u32>()
                            .unwrap();
                    }
                    curr_indices.clear();
                    curr_num.clear();
                }
            }
        }
        total
    }

    pub fn part_two<T: Display>(&self, inp: T) -> u32 {
        let arr = inp
            .to_string()
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();
        let mut total = 0;
        let ncols = arr.first()
            .unwrap()
            .len();

        for (y_, row) in arr
            .iter()
            .enumerate()
        {
            for (x_, chr) in row
                .iter()
                .enumerate()
            {
                if *chr == '*' {
                    let nums = self.symbol_adjacent(
                        &arr, &vec![(y_, x_)],
                        char::is_numeric
                    );
                    if !nums.is_empty() {
                        let mut num_map = HashMap::new();
                        for (y, mut x) in nums {
                            let mut curr_num = String::new();

                            while x > 0 && arr[y][x - 1].is_numeric() {
                                x -= 1;
                            }
                            while (0..ncols).contains(&x)
                                && arr[y][x].is_numeric()
                            {
                                curr_num.push(arr[y][x]);
                                x += 1;
                            }
                            num_map.insert((y, x), curr_num);
                        }
                        if num_map.len() == 2 {
                            let mut values = num_map.values();
                            total += values.next()
                                .unwrap()
                                .parse::<u32>()
                                .unwrap()
                                * values.next()
                                .unwrap()
                                .parse::<u32>()
                                .unwrap();
                        }
                    }
                }
            }
        }
        total
    }
}

impl Solution for Day3 {
    const NAME: &'static str = "Gear Rati8os";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 532428) ;
        assert_eq!(p2, 84051670);
    }
}

fn main() {
    aoc_2023::run_day(3, &Day3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}