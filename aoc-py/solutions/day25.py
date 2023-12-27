"""
Day 25: Snowverload

https://adventofcode.com/2023/day/25
"""
__all__ = ('Day25',)

from typing import ClassVar

import networkx as nx

from ..solution import Solution

class Day25(Solution):
    NAME: ClassVar[str] = 'Snowverload'

    def part_one(self, inp: str) -> int:
        graph = nx.Graph()

        for line in inp.splitlines():
            left, right = line.split(':')
            for node in right.strip().split():
                graph.add_edge(left, node)
                graph.add_edge(node, left)

        graph.remove_edges_from(
            nx.minimum_edge_cut(graph)
        )

        a, b = nx.connected_components(graph)
        return len(a) * len(b)

    def part_two(self, _: str) -> None:
        """No part 2 for day 25!

        Merry Christmas!
        """

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 554064
        assert p2 is None