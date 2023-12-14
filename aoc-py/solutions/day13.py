"""
Day 13: Point of Incidence

https://adventofcode.com/2023/day/13
"""
__all__ = ('Day13',)

from typing import ClassVar, Optional

from ..solution import Solution

class Day13(Solution):
    NAME: ClassVar[str] = 'Point of Incidence'

    def find_mirror(self, grid: list[str], smudge: int = 0) -> int:
        """Finds and returns the index (1-indexed) of the *horizontal* mirror line"""
        for line in range(1, len(grid)):
            # left side of the reflection line
            # (reversed for comparison; to see if its mirror matches with the right side)
            left = grid[line - 1::-1]
            # right side of the reflection line
            right = grid[line:]
            # truncates the larger side of the mirror
            # to the size of the smaller side so they match
            left_size = len(left)
            right_size = len(right)

            if left_size > right_size:
                left = left[:right_size]
            else:
                right = right[:left_size]

            if (
                # if the smudge is 0 (Part 1: no differences are allowed; the grid must be reflected exactly)
                # we simply check left and side equality and if so then we found the mirror line: `mirror`
                # technically this specific case for when smudge is 0 is completely unecessary
                # but it is more to compare lists directly vs. the below (which would still work)
                smudge == 0
                and left == right
                # if smudge is not 0 that means we have to individual compare all the tiles
                # and check if there is exactly `smudge` number of tile differences
                # between each side of the mirror line
                or sum(t1 != t2
                    for t1, t2 in zip(
                        ''.join(left),
                        ''.join(right),
                    )
                ) == smudge
            ):
                return line
        # if no mirror line found, simply return 0
        return 0

    def _get_summary(self, inp: str, *, smudge: int = 0) -> int:
        return sum(
            # horizontal mirror lines 
            100 * self.find_mirror(
                grid := raw_grid.splitlines(),
                smudge,
            )
            # transpose the grid
            # to get vertical mirror lines
            + self.find_mirror(
                [''.join(row) for row in zip(*grid)],
                smudge,
            )
            for raw_grid in inp.split('\n\n')
        )

    def part_one(self, inp: str) -> int:
        return self._get_summary(inp, smudge=0)

    def part_two(self, inp: str) -> int:
        return self._get_summary(inp, smudge=1)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 35210
        assert p2 == 31974