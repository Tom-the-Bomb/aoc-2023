//! Day 25: Snowverload
//!
//! <https://adventofcode.com/2023/day/25>
use std::fmt::Display;
use rustworkx_core::{
    petgraph::graphmap::GraphMap,
    connectivity::stoer_wagner_min_cut as min_cut,
};
use aoc_2023::Solution;

pub struct Day25;

impl Day25 {
    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        let mut graph = GraphMap::new();
        let inp = inp
            .to_string();

        for line in inp.lines() {
            let (left, right) = line
                .split_once(':')
                .unwrap();
            for node in right
                .trim()
                .split_whitespace()
            {
                graph.add_edge(left, node, 1);
                graph.add_edge(node, left, 1);
            }
        }

        if let Ok(Some((_, partition_1))) = min_cut(
            &graph,
            |_| Ok::<usize, usize>(1))
        {
            let size_1 = partition_1.len();
            let size_2 = graph.node_count() - size_1;

            size_1 * size_2
        } else {
            panic!("Could not perform a minimum cut on the graph");
        }
    }

    pub fn part_two<T: Display>(&self, _inp: T) -> usize {
        unimplemented!()
    }
}

impl Solution for Day25 {
    const NAME: &'static str = "Snowverload";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);

        println!("Part 1: {p1}");

        assert_eq!(p1, 554064);
    }
}

fn main() {
    aoc_2023::run_day(25, &Day25);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}