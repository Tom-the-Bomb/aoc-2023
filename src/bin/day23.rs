//! Day 23: A Long Walk
//!
//! <https://adventofcode.com/2023/day/23>
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};
use aoc_2023::Solution;

pub struct Day23;

type Coord = (usize, usize);

impl Day23 {
    fn get_neighbors(
        grid: &[Vec<u8>],
        row: usize,
        col: usize,
    ) -> HashMap<u8, Coord> {
        [
            (b'>', (row, col + 1)),
            (b'<', (row, col.wrapping_sub(1))),
            (b'v', (row + 1, col)),
            (b'^', (row.wrapping_sub(1), col)),
        ]
        .into_iter()
        .filter(|&(_, (i, j))| grid
            .get(i)
            .and_then(|row| row.get(j))
            .map(|&tile| tile != b'#')
            .unwrap_or_default()
        )
        .collect::<HashMap<u8, Coord>>()
    }

    fn dfs(
        node: Coord,
        end: Coord,
        graph: &HashMap<Coord, HashMap<Coord, usize>>,
        seen: &mut HashSet<Coord>,
    ) -> usize {
        if node == end { 0 }
        else {
            seen.insert(node);

            #[allow(clippy::filter_map_bool_then)]
            let max_length = graph
                .get(&node)
                .map_or(0, |connected_nodes|
                    connected_nodes
                        .keys()
                        .filter_map(|next_node|
                            (!seen.contains(next_node))
                                .then(|| Self::dfs(*next_node, end, graph, seen) + graph[&node][&next_node])
                        )
                        .max()
                        .unwrap_or(0)
                );

            seen.remove(&node);
            max_length
        }
    }

    fn hike<T: Display>(inp: T, slopes: bool) -> usize {
        let grid = inp
            .to_string()
            .lines()
            .map(|line| line
                .as_bytes()
                .to_vec()
            )
            .collect::<Vec<Vec<u8>>>();

        let start = (0, grid[0]
            .iter()
            .position(|&c| c == b'.')
            .unwrap()
        );
        let end = {
            let last_row = grid.len() - 1;
            (last_row, grid[last_row]
                .iter()
                .position(|&c| c == b'.')
                .unwrap()
            )
        };

        let mut nodes = vec![start, end];
        let grid_ref = &grid;

        nodes.extend(
            grid.iter()
                .enumerate()
                .flat_map(|(i, row)| row
                    .iter()
                    .enumerate()
                    .filter_map(move |(j, &tile)|
                        (tile != b'#' && Self::get_neighbors(grid_ref, i, j)
                            .len() >= 3
                        )
                        .then_some((i, j))
                    )
                )
        );

        let mut graph = HashMap::new();

        for starting_node in &nodes {
            let mut to_check = VecDeque::from([
                (*starting_node, 0)
            ]);
            let mut seen = HashSet::new();

            while let Some((node @ (row, col), distance)) = to_check.pop_front() {
                if distance > 0 && nodes.contains(&node) {
                    graph
                        .entry(*starting_node)
                        .or_insert_with(HashMap::new)
                        .insert(node, distance);
                } else {
                    let connected_nodes = Self::get_neighbors(grid_ref, row, col);
                    let next_nodes =
                        if let Some(&slope) = grid
                            .get(row)
                            .and_then(|row| row.get(col))
                            .and_then(|node| connected_nodes.get(node))
                            .filter(|_| slopes)
                        { vec![slope] }
                        else { connected_nodes
                            .into_values()
                            .collect::<Vec<Coord>>()
                        };

                    for node in next_nodes {
                        if !seen.contains(&node) {
                            to_check.push_back((node, distance + 1));
                            seen.insert(node);
                        }
                    }
                }
            }
        }
        Self::dfs(start, end, &graph, &mut HashSet::new())
    }

    pub fn part_one<T: Display>(&self, inp: T) -> usize {
        Self::hike(inp, true)
    }

    pub fn part_two<T: Display>(&self, inp: T) -> usize {
        Self::hike(inp, false)
    }
}

impl Solution for Day23 {
    const NAME: &'static str = "A Long Walk";

    fn run(&self, inp: String) {
        let p1 = self.part_one(&inp);
        let p2 = self.part_two(&inp);

        println!("Part 1: {p1}");
        println!("Part 2: {p2}");

        assert_eq!(p1, 2182);
        assert_eq!(p2, 6670);
    }
}

fn main() {
    aoc_2023::run_day(23, &Day23);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() { main(); }
}