pub mod day1;

pub use day1::Day1;

use crate::{
    error::{Error, Result},
    solution::Solution,
};

pub fn get_day(day: u8) -> Result<impl Solution> {
    match day {
        1 => Ok(Day1),
        _ => Err(Error::NoSolutionForDay(day)),
    }
}