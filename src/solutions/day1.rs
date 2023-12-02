use crate::solution::Solution;
struct Day1;

impl Day1 {
    pub fn part_one(&self) -> u32 {
        inp.lines()
            .map(|line| {
                line.chars()
                    .filter(char::is_numeric)
            })
    }
}

impl Solution for Day1 {
    fn run(&self, inp: String) {

    }
}