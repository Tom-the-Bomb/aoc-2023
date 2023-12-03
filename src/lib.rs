use std::fs::read_to_string;

pub use solution::*;

pub mod solution;

#[inline]
pub fn get_input(day: u8) -> String {
    read_to_string(format!("./inputs/day{day}.txt"))
        .expect(format!("Failed to read input for Day {day}").as_str())
}

pub fn run_day<D: Solution>(day: u8, cls: D) {
    let text = format!(" Day [{day}] Solution - {} ", cls.name());
    let line = format!(
        "+------+{}+",
        "-".repeat(text.chars().count())
    );
    println!("\n{line}\n| RUST |{text}|\n{line}");
    cls.run(get_input(day));
}