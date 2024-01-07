use std::fmt::Display;

pub trait Solution {
    const NAME: &'static str;

    fn part_one<T: Display>(&self, inp: T) -> usize;

    fn part_two<T: Display>(&self, inp: T) -> usize;

    fn run(&self, inp: String);

    #[inline]
    #[must_use]
    fn name(&self) -> &'static str {
        Self::NAME
    }
}