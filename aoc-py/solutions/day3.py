"""
Day 3: Gear Ratios

https://adventofcode.com/2023/day/3
"""
__all__ = ('Day3',)

from typing import Callable, ClassVar

from ..solution import Solution

class Day3(Solution):
    NAME: ClassVar[str] = 'Gear Ratios'

    def _symbol_adjacent(
        self,
        nrows: int,
        ncols: int,
        arr: list[str],
        coordinates: list[tuple[int, int]],
        condition: Callable[[str], bool],
    ) -> list[tuple[int, int]]:
        ncoords = len(coordinates) - 1
        symbols = []

        for i, (row, col) in enumerate(coordinates):
            indices = (
                (row - 1, col),
                (row + 1, col),
            )
            if i == 0:
                indices += (
                    (row, col - 1),
                    (row - 1, col - 1),
                    (row + 1, col - 1),
                )
            if i == ncoords:
                indices += (
                    (row, col + 1),
                    (row - 1, col + 1),
                    (row + 1, col + 1),
                )
            for y, x in indices:
                if (
                    y in range(nrows)
                    and x in range(ncols)
                    and condition(arr[y][x])
                ):
                    symbols.append((y, x))
        return symbols

    def part_one(self, inp: str) -> int:
        arr = inp.splitlines()
        nrows = len(arr)
        ncols = len(arr[0])
        total = 0

        curr_indices = []
        curr_num = ''
        for y in range(nrows):
            for x in range(ncols):
                if (n := arr[y][x]).isnumeric():
                    curr_indices.append((y, x))
                    curr_num += n
                else:
                    if curr_indices and self._symbol_adjacent(
                        nrows, ncols, arr, curr_indices,
                        lambda c: not c.isnumeric() and c != '.',
                    ):
                        total += int(curr_num)
                    curr_indices = []
                    curr_num = ''
        return total

    def part_two(self, inp: str) -> int:
        arr = inp.splitlines()
        nrows = len(arr)
        ncols = len(arr[0])
        total = 0

        for row in range(nrows):
            for col in range(ncols):
                if arr[row][col] == '*' and (nums := self._symbol_adjacent(
                    nrows, ncols, arr, [(row, col)],
                    str.isnumeric,
                )):
                    num_map = {}
                    for y, x in nums:
                        curr_num = ''

                        while x > 0 and arr[y][x - 1].isnumeric():
                            x -= 1
                        while x in range(ncols) and (n := arr[y][x]).isnumeric():
                            curr_num += n
                            x += 1
                        num_map[(y, x)] = int(curr_num)
                    if len(num_map) == 2:
                        a, b = num_map.values()
                        total += a * b
        return total

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 532428
        assert p2 == 84051670