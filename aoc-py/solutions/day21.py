"""
Day 21: Step Counter

https://adventofcode.com/2023/day/21
"""
__all__ = ('Day21',)

from typing import ClassVar

from ..solution import Solution

class Day21(Solution):
    NAME: ClassVar[str] = 'Step Counter'

    def _get_starting_pos(self, grid: list[str]) -> tuple[int, int]:
        """Finds and returns the coordinates (indices) of the starting position, the character 'S'"""
        for i, row in enumerate(grid):
            for j, char in enumerate(row):
                if char == 'S':
                    return i, j
        raise ValueError("No 'S' character found in grid")

    def _traverse(
        self,
        grid: list[str],
        start: tuple[int, int],
        *,
        steps: int
    ) -> int:
        initial_wrap = (int(), int())

        to_check = [(start, initial_wrap, steps)]
        traversed = {(start, initial_wrap)}
        n_reached = 0

        n_rows = len(grid)
        n_cols = len(grid[0])

        while to_check:
            (row, col), (n_row_wraps, n_col_wraps), steps_left = to_check.pop(0)

            if steps_left % 2 == 0:
                # even # of steps left -> we are in a cycle of
                # going back and forth +1 -> -1 ... in the same position, so we end here
                n_reached += 1

            if steps_left > 0:
                directions = (
                    (row, col + 1),
                    (row, col - 1),
                    (row + 1, col),
                    (row - 1, col),
                )

                for new_row, new_col in directions:
                    # gets the amount of times we wrap around the grid's repetition cycles
                    # also the "n-th cycle" of the grid content
                    #
                    # i.e. when we are in the initial, given grid:
                    #   `n_row_wraps` and `n_col_wraps` are both `0`
                    #
                    # quotient of dividing the current `index` by the # of rows/cols
                    # needs to be stored in `traversed` to allow the set to distinguish
                    # between coordinates that are the same, but are on different cycles/wraps of the grid
                    #
                    # the modulo of the index and the # of rows/cols will give us the index we can use on the gri
                    new_row_wraps, new_row = divmod(new_row, n_rows)
                    new_col_wraps, new_col = divmod(new_col, n_cols)
                    new_row_wraps += n_row_wraps
                    new_col_wraps += n_col_wraps

                    if (
                        new_row in range(n_rows)
                        and new_col in range(n_cols)
                        and grid[new_row][new_col] != '#'
                        and (pos := ((new_row, new_col), (new_row_wraps, new_col_wraps))) not in traversed
                    ):
                        to_check.append(pos + (steps_left - 1,))
                        traversed.add(pos)
        return n_reached

    def part_one(self, inp: str) -> int:
        grid = inp.splitlines()
        start = self._get_starting_pos(grid)

        return self._traverse(grid, start, steps=64)

    def part_two(self, inp: str) -> int:
        grid = inp.splitlines()

        n_rows = len(grid)
        n = 26501365 // n_rows
        start = self._get_starting_pos(grid)
        start_x, _ = start

        t1 = self._traverse(grid, start, steps=start_x)
        t2 = self._traverse(grid, start, steps=start_x + n_rows)
        t3 = self._traverse(grid, start, steps=start_x + n_rows + n_rows)

        return (
            (n ** 2 - n)
            * ((t1 + t3) // 2 - t2)
            + n * (t2 - t1)
            + t1
        )

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 3743
        assert p2 == 618261433219147