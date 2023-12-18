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
        n_coords = len(coordinates) - 1
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
            if i == n_coords:
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

        # indices of the digits of the current number
        curr_indices = []
        # the digits of the current number
        curr_num = ''
        for y in range(nrows):
            for x in range(ncols):
                # we hit a digit
                # append digit to the current number we are tracking
                if (n := arr[y][x]).isnumeric():
                    curr_indices.append((y, x))
                    curr_num += n
                else:
                    # we hit a non digit
                    # if `curr_indices` is not empty,
                    #   this marks the end of the current number we are tracking
                    #   now we need to check if it is a "part number" by checking if there are symbols adjacent to it
                    #
                    # otherwise nothing happens
                    if curr_indices and self._symbol_adjacent(
                        nrows, ncols, arr, curr_indices,
                        lambda c: not c.isnumeric() and c != '.',
                    ):
                        total += int(curr_num)
                    # reset the current number tracking (since we've finished with it)
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
                # we have hit a gear, find all adjacent digits (could lead us to a number)
                if arr[row][col] == '*' and (nums := self._symbol_adjacent(
                    nrows, ncols, arr, [(row, col)],
                    str.isnumeric,
                )):
                    num_map = {}
                    for y, x in nums:
                        curr_num = ''

                        # backtrack to the start of the number (go back until we hit a non-number)
                        while x > 0 and arr[y][x - 1].isnumeric():
                            x -= 1
                        # once we've found the start of the number
                        # keep going forwards to fetch the whole number
                        # appending each digit to `curr_num`
                        while x in range(ncols) and (n := arr[y][x]).isnumeric():
                            curr_num += n
                            x += 1
                        # maps the (row, col) position of the start of the number -> number
                        num_map[(y, x)] = int(curr_num)
                    # if there are only 2 adjacent numbers, we add the product of them to the total
                    if len(num_map) == 2:
                        a, b = num_map.values()
                        total += a * b
        return total

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 532428
        assert p2 == 84051670