//! Day 19: Aplenty
//!
//! <https://adventofcode.com/2023/day/19>
use std::{
    collections::HashMap,
    fmt::Display,
    ops::Range,
};
use aoc_2023::Solution;

#[derive(Debug, Clone)]
struct Rule<'a> {
    key: &'a str,
    target: &'a str,
    rhs: usize,
    is_gt: bool,
    condition: fn(&usize, &usize) -> bool,
}

impl<'a> Rule<'a> {
    pub fn eval(&self, data: &HashMap<String, usize>) -> bool {
        data.get(self.key)
            .map(|key| (self.condition)(key, &self.rhs))
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
struct Workflow<'a> {
    default: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    pub const fn from_default(default: &'a str) -> Self {
        Self {
            default,
            rules: Vec::new(),
        }
    }

    pub fn with_rules<T>(mut self, rules: T) -> Self
    where
        T: Iterator<Item = &'a str>
    {
        for rule in rules {
            let (condition, target) = rule
                .split_once(':')
                .unwrap();
            let is_gt = condition.contains('>');
            let (key, rhs) = condition.split_once(
                if is_gt { '>' } else { '<' }
            )
                .unwrap();

            self.rules.push(Rule {
                key, target, is_gt,
                rhs: rhs.parse::<usize>()
                    .unwrap(),
                condition: if is_gt { usize::gt } else { usize::lt },
            });
        }
        self
    }
}

pub struct Day19;

impl Day19 {
    #[must_use]
    fn parse_workflows<T>(raw: &T) -> HashMap<String, Workflow<'_>>
    where
        T: AsRef<str>
    {
        raw
            .as_ref()
            .lines()
            .map(|line| {
                let (name, data) = line
                    .split_once('{')
                    .unwrap();
                let mut data = data
                    .trim_end_matches('}')
                    .rsplit(',');
                let default = data
                    .next()
                    .unwrap();
                (
                    name.to_string(),
                    Workflow::from_default(default)
                        .with_rules(data.rev())
                )
            })
            .collect::<HashMap<String, Workflow>>()
    }

    fn parse_parts<T>(raw: &T)
        -> impl Iterator<Item = HashMap<String, usize>> + '_
    where
        T: AsRef<str>,
    {
        raw
            .as_ref()
            .lines()
            .map(|line|
                line
                    .trim_matches(|c| c == '{' || c == '}')
                    .split(',')
                    .map(|entry| {
                        let (part, rating) = entry
                            .split_once('=')
                            .unwrap();
                        (
                            part.to_string(),
                            rating
                                .parse::<usize>()
                                .unwrap()
                        )
                    })
                    .collect::<HashMap<String, usize>>()
            )
    }

    #[must_use]
    fn is_accepted(
        workflows: &HashMap<String, Workflow>,
        group: &HashMap<String, usize>,
        target: &str,
    ) -> bool {
        match target {
            "A" => true,
            "R" => false,
            target => workflows
                .get(target)
                .map(|workflow| {
                    workflow.rules
                        .iter()
                        .find_map(|rule|
                            rule.eval(group)
                                .then(|| Self::is_accepted(workflows, group, rule.target))
                        )
                        .unwrap_or_else(|| Self::is_accepted(workflows, group, workflow.default))
                })
                .unwrap_or_default()
        }
    }

    #[must_use]
    fn count_range(
        workflows: &HashMap<String, Workflow>,
        ranges: &mut HashMap<String, Range<usize>>,
        target: &str,
    ) -> usize {
        match target {
            "A" => ranges
                .values()
                .map(Range::len)
                .product(),
            "R" => 0,
            target => {
                let mut total = 0;
                if let Some(workflow) = workflows
                    .get(target)
                {
                    for rule in &workflow.rules {
                        let range = ranges
                            .get(rule.key)
                            .unwrap();
                        let true_range =
                            if rule.is_gt {
                                range.start.max(rule.rhs + 1)..range.end
                            } else {
                                range.start..range.end.min(rule.rhs)
                            };
                        let false_range =
                            if rule.is_gt {
                                range.start..range.end.min(rule.rhs + 1)
                            } else {
                                range.start.max(rule.rhs)..range.end
                            };

                        if !true_range.is_empty() {
                            let mut clone = ranges.clone();
                            clone.insert(
                                rule.key.to_string(),
                                true_range,
                            );
                            total += Self::count_range(workflows, &mut clone, rule.target);
                        }
                        if false_range.is_empty() {
                            return total;
                        }
                        ranges.insert(
                            rule.key.to_string(),
                            false_range,
                        );
                    }
                    total += Self::count_range(workflows, ranges, workflow.default);
                }
                total
            }
        }
    }

    /// # Panics
    ///
    /// If failed to parse input (malformed)
    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        let inp = inp
            .to_string()
            .replace('\r', "");
        let (workflows, parts) = inp
            .split_once("\n\n")
            .unwrap();
        let workflows = Self::parse_workflows(&workflows);
        let parts = Self::parse_parts(&parts);

        parts
            .filter_map(|group|
                Self::is_accepted(&workflows, &group, "in")
                    .then(|| group.values().sum::<usize>())
            )
            .sum()
    }

    /// # Panics
    ///
    /// If failed to parse input (malformed)
    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        let inp = inp
            .to_string()
            .replace('\r', "");
        let (workflows, _) = inp
            .split_once("\n\n")
            .unwrap();
        let workflows = Self::parse_workflows(&workflows);

        Self::count_range(
            &workflows,
            &mut HashMap::from([
                ("x".to_string(), 1..4001),
                ("m".to_string(), 1..4001),
                ("a".to_string(), 1..4001),
                ("s".to_string(), 1..4001),
            ]),
            "in",
        )
    }
}

impl Solution for Day19 {
    const NAME: &'static str = "Aplenty";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 362_930);
        assert_eq!(p2, 116_365_820_987_729);
    }
}

fn main() {
    aoc_2023::run_day(19, &Day19);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}