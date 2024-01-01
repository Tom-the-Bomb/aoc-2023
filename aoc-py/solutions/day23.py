"""
Day 23: A Long Walk

https://adventofcode.com/2023/day/23
"""
__all__ = ('Day23',)

from typing import ClassVar
from collections import defaultdict

from ..solution import Solution

class Day23(Solution):
    NAME: ClassVar[str] = 'A Long Walk'

    def _get_neighbors(self, grid: list[str], row: int, col: int) -> dict[str, tuple[int, int]]:
        """Returns the 4-neighborhood tiles of (i, j)

        while ensuring it is not a maze wall ('#')
        and that it does not go out of the grid bounds
        """
        n_rows = len(grid)
        n_cols = len(grid[0])

        return {
            direction: (row, col)
            for direction, (row, col) in (
                ('>', (row, col + 1)),
                ('<', (row, col - 1)),
                ('v', (row + 1, col)),
                ('^', (row - 1, col)),
            )
            if row in range(n_rows)
            and col in range(n_cols)
            and grid[row][col] != '#'
        }

    def _hike(self, inp: str, *, slopes: bool) -> int:
        grid = inp.splitlines()

        start = (0, grid[0].index('.'))

        last_row = len(grid) - 1
        end = (last_row, grid[last_row].index('.'))

        nodes = [start, end]

        for i, row in enumerate(grid):
            for j, tile in enumerate(row):
                if tile != '#' and len(self._get_neighbors(grid, i, j)) >= 3:
                    # crossroad points, where we can make a choice of where to go
                    # they will be the nodes for the constructed graph
                    nodes.append((i, j))

        # graph of nodes (crossroads) + edges between them
        #
        # maps: node -> (map: connected nodes -> distance away from starting node)
        # only stores directly adjacent nodes
        graph = defaultdict(dict)

        for starting_node in nodes:
            to_check = [(starting_node, int())]
            seen = {starting_node}

            while to_check:
                # distance = edge length
                node, distance = to_check.pop()

                if distance > 0 and node in nodes:
                    # we've reached a crossroad/node, we can add it to the graph
                    graph[starting_node][node] = distance
                else:
                    # hit a regular tile
                    # keep on traversing, "floodfilling" through the neighbors of each tile
                    row, col = node
                    connected_nodes = self._get_neighbors(grid, row, col)

                    for node in (
                        [slope]
                        # we've reached a slope tile: '<', '>', '^', 'v'
                        # obtain the single direction (neighbor in that direction)
                        # that we are allowed to go to according to the slope
                        if slopes and (slope := connected_nodes.get(grid[row][col]))
                        # '.' tile, regular tile, check all 4 neighbors
                        else connected_nodes.values()
                    ):
                        if node not in seen:
                            # add tile as part of edge, increment distance
                            to_check.append((node, distance + 1))
                            seen.add(node)

        seen = set()

        def _dfs(node: tuple[int, int]) -> int:
            """Brutes force the length of the longest path

            depth-first-search traversal through all path possibilities using generated graph
            """
            if node == end:
                return 0
            # `seen` set to avoid cycles.
            seen.add(node)

            max_length = max([
                # recursively pathfinds, and adds up all the distances of the edges
                _dfs(next_node) + graph[node][next_node]
                for next_node in graph[node]
                if next_node not in seen
            ] + [0]) # `0` if no path found, avoids max() arg is an empty sequence error

            seen.remove(node)
            return max_length

        return _dfs(start)

    def part_one(self, inp: str) -> int:
        return self._hike(inp, slopes=True)

    def part_two(self, inp: str) -> int:
        return self._hike(inp, slopes=False)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 2182
        assert p2 == 6670