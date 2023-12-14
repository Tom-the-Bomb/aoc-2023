"""
Day 13: Point of Incidence

https://adventofcode.com/2023/day/13
"""
__all__ = ('Day13',)

from typing import ClassVar

from ..solution import Solution

class Day13(Solution):
    NAME: ClassVar[str] = 'Point of Incidence'

    def _find_mirror(self, grid: list[str], smudge: int = 0) -> int:
        """Finds and returns the index (1-indexed) of the *horizontal* mirror line"""

        for line in range(1, len(grid)):
            # the rows "above" (before) the the reflection line
            # (reversed for comparison; to see if its mirror matches with the right side)
            #
            # equivalent to `grid[:line][::-1]`
            left = grid[line - 1::-1]
            # the rows "below" (after) the reflection line
            right = grid[line:]

            left_size = len(left)
            right_size = len(right)
            # truncates the larger side of the mirror
            # to the size of the smaller side so they match
            if left_size > right_size:
                left = left[:right_size]
            else:
                right = right[:left_size]

            if (
                # if the smudge is 0 (Part 1: no differences are allowed; the grid must be reflected exactly)
                # we simply check left and side equality and if true then we have found the mirror line: `mirror`
                #
                # technically this specific case for when smudge is 0 is completely unecessary
                # but it is more to compare lists directly vs. the below (which would still work)
                smudge == 0
                and left == right
                # if smudge is not 0 that means we have to individual compare all the tiles
                # and check if there is exactly `smudge` number of tile differences
                # between each side of the mirror line
                #
                # [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
                # where smudge = desired hamming distance
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
            # horizontal mirror lines (x100)
            100 * self._find_mirror(
                grid := raw_grid.splitlines(),
                smudge,
            )
            # transpose the grid
            # to get vertical mirror lines
            + self._find_mirror(
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