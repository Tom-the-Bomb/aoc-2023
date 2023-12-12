"""
Day 11: Cosmic Expansion

https://adventofcode.com/2023/day/11
"""
__all__ = ('Day11',)

from typing import ClassVar
from itertools import combinations

from ..solution import Solution

class Day11(Solution):
    NAME: ClassVar[str] = 'Cosmic Expansion'

    def _expand_one(self, universe: list[list[str]]) -> list[list[str]]:
        """Expands the universe

        we need to repeat the expansion process 2 times
        1 time for rows, 1 time for columns, hence the `for _ in range(2):`
            we transpose the matrix after every time
            universe -> expand rows -> transpose -> expand cols -> transpose -> return expanded universe
        """
        for _ in range(2):
            new_list = []
            for row in universe:
                new_list.append(row)
                if '#' not in row:
                    new_list.append(row)
            universe = list(zip(*new_list))
        return universe

    def _get_galaxies(self, universe: list[list[str]]) -> list[tuple[int, int]]:
        """Returns the indices of the galaxies in the universe"""
        return [
            (i, j)
            for i, row in enumerate(universe)
            for j, galaxy in enumerate(row)
            if galaxy == '#'
        ]

    def _get_total_distances(self, inp: str, *, expansion_amount: int = 2) -> int:
        """Calculates (solves the problem) simply by adding `expansion_amount`
        every time an empty row/column is between a pair of galaxies
        to account for the expansion without physically having to expand the universe matrix
        """
        universe = [list(row) for row in inp.splitlines()]
        empty_rows = [
            i for i, row in enumerate(universe)
            if '#' not in row
        ]
        empty_cols = [
            j for j, col in enumerate(list(zip(*universe)))
            if '#' not in col
        ]
        total = 0
        for (i1, j1), (i2, j2) in combinations(
            self._get_galaxies(universe),
            r=2,
        ):
            for row in range(min(i1, i2), max(i1, i2)):
                total += expansion_amount if row in empty_rows else 1
            for col in range(min(j1, j2), max(j1, j2)):
                total += expansion_amount if col in empty_cols else 1
        return total

    def part_one_bf(self, inp: str) -> int:
        """Part 1 using a more brute force strategy or actually expanding the universe"""

        universe = [list(row) for row in inp.splitlines()]
        universe = self._expand_one(universe)
        return sum(
            # Manhattan distance formula
            abs(i2 - i1) + abs(j2 - j1)
            for (i1, j1), (i2, j2) in combinations(
                self._get_galaxies(universe),
                r=2,
            )
        )

    def part_one(self, inp: str) -> int:
        return self._get_total_distances(inp, expansion_amount=2)

    def part_two(self, inp: str) -> int:
        return self._get_total_distances(inp, expansion_amount=1_000_000)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 9543156
        assert p2 == 625243292686