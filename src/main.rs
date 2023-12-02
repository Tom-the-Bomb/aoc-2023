use std::{
    env,
    fs::read_to_string,
};
use solutions::get_day;
use solution::Solution;

pub mod solutions;
pub mod solution;
pub mod error;

#[inline]
fn get_input(day: u8) -> String {
    read_to_string(format!("./inputs/day{day}.txt"))
        .expect(format!("Failed to read input for Day {day}").as_str())
}

fn run_day(day: u8) {
    match get_day(day) {
        Ok(sol) => {
            let text = format!("| 🦀 Day [{day}] Solution - {} |", sol.name());
            let line = format!(
                "+{}+",
                "-".repeat(text.chars().count() - 1)
            );
            println!("\n{line}\n{text}\n{line}");
            sol.run(get_input(day));
        },
        Err(e) => println!("{e}"),
    }
}

fn main() {
    if let Some(day) = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .map(|x| x.parse::<u8>())
        .and_then(Result::ok)
    {
        run_day(day);
    } else {
        let mut day = 1;
        while let Ok(_) = get_day(day) {
            run_day(day);
            day += 1;
        }
    }
}