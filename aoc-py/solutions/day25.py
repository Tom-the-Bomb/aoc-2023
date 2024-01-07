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

        # generate graph
        for line in inp.splitlines():
            left, right = line.split(':')
            for node in right.strip().split():
                graph.add_edge(left, node)
                graph.add_edge(node, left)

        # remove the edges that are of the cut
        graph.remove_edges_from(
            nx.minimum_edge_cut(graph)
        )

        # get 2 partitioned groups after the cut (`a` and `b`)
        a, b = nx.connected_components(graph)
        # multiply sizes of 2 groups together
        return len(a) * len(b)

    def part_two(self, _: str) -> None:
        """No part 2 for day 25!

        Merry Christmas!
        """

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))

        assert p1 == 554064