pub mod day1;

pub use day1::Day1;

use crate::{Error, Result, Solution};

pub fn get_day(day: u8) -> Result<impl Solution> {
    Ok(match day {
        1 => Day1,
        _ => return Err(Error::NoSolutionForDay(day)),
    })
}