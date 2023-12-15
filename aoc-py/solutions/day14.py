from __future__ import annotations
"""
Day 14: Parabolic Reflector Dish

https://adventofcode.com/2023/day/14
"""
__all__ = ('Day14',)

from typing import ClassVar, TypeAlias, TYPE_CHECKING

from ..solution import Solution

if TYPE_CHECKING:
    Grid: TypeAlias = list[list[str]]

class Day14(Solution):
    NAME: ClassVar[str] = 'Parabolic Reflector Dish'

    def _transpose(self, grid: Grid) -> Grid:
        """Transposes the grid"""
        return [list(row) for row in zip(*grid)]

    def _reverse_rows(self, grid: Grid) -> Grid:
        """Reverses each row in the grid"""
        for row in grid:
            row.reverse()
        return grid

    def _tilt_lever(self, grid: Grid) -> Grid:
        """Pushes all rounded rocks to the left side (west) of the matrix"""
        for row in grid:
            for i, element in enumerate(row):
                # goes from left to right for each row and finds all rounded rocks
                if element == 'O':
                    # for each rounded rock, traverse backwards in the row
                    # and if the tile before it is empty we can swap and move the rock to the left
                    # we keep doing so until we hit a non empty tile
                    for t in range(i, 0, -1):
                        if row[t - 1] == '.':
                            # is empty, swap
                            row[t - 1], row[t] = row[t], row[t - 1]
                        else:
                            # non empty tile, we are done pushing it
                            break
        return grid

    def _tilt_north(self, grid: Grid) -> Grid:
        """Tilts the rocks north (up) (transpose first)"""
        grid = self._transpose(grid)
        grid = self._tilt_lever(grid)
        return self._transpose(grid)

    def _tilt_south(self, grid: Grid) -> Grid:
        """Tilts the rocks south (down)
        (same as north; but reverse the columns
        (which are rows after transposal))
        """
        grid = self._transpose(grid)
        grid = self._reverse_rows(grid)
        grid = self._tilt_lever(grid)
        grid = self._reverse_rows(grid)
        return self._transpose(grid)

    def _tilt_east(self, grid: Grid) -> Grid:
        """Tilts the rocks east (right)
        Same as the default (left) but reverses the rows
        """
        grid = self._reverse_rows(grid)
        grid = self._tilt_lever(grid)
        return self._reverse_rows(grid)

    def _cycle(self, grid: Grid) -> Grid:
        grid = self._tilt_north(grid)
        # west = left (no need for pre manipulation)
        grid = self._tilt_lever(grid)
        grid = self._tilt_south(grid)
        return self._tilt_east(grid)

    def _get_load(self, grid: Grid) -> int:
        """Calculates the load
        by counting the rounded boulders per row
        """
        n_rows = len(grid)
        return sum(
            row.count('O') * (n_rows - i)
            for i, row in enumerate(grid)
        )

    def part_one(self, inp: str) -> int:
        grid = [list(row) for row in inp.splitlines()]
        grid = self._tilt_north(grid)
        return self._get_load(grid)

    def part_two(self, inp: str) -> int:
        grid = [list(row) for row in inp.splitlines()]

        # `cycles` will contain the extra non-repeating terms
        # + 1 period
        cycles = [grid]
        # contains the index of the element that first begins the repeating sequence
        # also equal to the length of the extra non-repeating prefixing chunk
        start = 0
        while True:
            # generates next term
            next_term = self._cycle(cycles[-1])
            # this marks the end of our first cycle
            # since we have already seen `next_term` before when it was the start of the cycle
            if next_term in cycles:
                # set `start` to the index of the start of the first cycle
                start = cycles.index(next_term)
                break
            else:
                cycles.append(next_term)

        for _ in range(
            # the total length of the repeating chunk
            (1_000_000_000 - start)
            # modulo by the period of the sequence
            # (gets the remainder at the end that does not complete another full cycle)
            % (len(cycles) - start)
            # add back the non-repeating chunk's length
            + start
        ):
            grid = self._cycle(grid)
        return self._get_load(grid)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 111339
        assert p2 == 93736