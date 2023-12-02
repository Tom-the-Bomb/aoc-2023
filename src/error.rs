use std::fmt;

#[derive(Debug)]
pub enum Error {
    NoSolutionForDay(u8)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                Self::NoSolutionForDay(day) =>
                    format!("Solution does not exist yet for day {day}"),
            }
            .as_str()
        )
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;