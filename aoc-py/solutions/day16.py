"""
Day 16: The Floor Will Be Lava

https://adventofcode.com/2023/day/16
"""
__all__ = ('Day16',)

from typing import ClassVar

from ..solution import Solution

class Day16(Solution):
    NAME: ClassVar[str] = 'The Floor Will Be Lava'

    def _get_energized_amount(
        self,
        grid: list[str],
        starting_row: int,
        starting_col: int,
        starting_row_incr: int,
        starting_col_incr: int,
    ) -> int:
        n_rows = len(grid)
        n_cols = len(grid[0])

        energized = set()
        to_check = [(
            (starting_row, starting_col),
            (starting_row_incr, starting_col_incr),
        )]
        while to_check:
            (row, col), (row_incr, col_incr) = to_check.pop(0)

            row += row_incr
            col += col_incr

            if row in range(n_rows) and col in range(n_cols):
                tile = grid[row][col]

                # hitting horizontal splitter from the top/bottom (row increment is not 0)
                if tile == '-' and row_incr != 0:
                    # 2 directions: up and down
                    directions = [(0, -1), (0, 1)]
                # hitting vertical splitter from the left/right (column increment is not 0)
                elif tile == '|' and col_incr != 0:
                    # 2 directions: left and right
                    directions = [(-1, 0), (1, 0)]
                else:
                    match tile:
                        case '/':
                            # (0, 1) -> (-1, 0) [down -> left]
                            # (1, 0) -> (0, -1) [right -> up]
                            # (-1, 0) -> (0, 1) [left -> down]
                            # (0, -1) -> (1, 0) [up -> right]
                            row_incr, col_incr = -col_incr, -row_incr
                        case '\\':
                            # (0, 1) -> (1, 0) [down -> right]
                            # (1, 0) -> (0, 1) [right -> down]
                            # (-1, 0) -> (0, -1) [left -> up]
                            # (0, -1) -> (-1, 0) [up -> left]
                            row_incr, col_incr = col_incr, row_incr
                    directions = [(row_incr, col_incr)]

                for direction in directions:
                    if (entry := ((row, col), direction)) not in energized:
                        energized.add(entry)
                        to_check.append(entry)
        return len({
            # get rid of duplicate traversals
            coords for coords, _ in energized
        })

    def part_one(self, inp: str) -> int:
        return self._get_energized_amount(
            inp.splitlines(),
            starting_row=0,
            # we start at -1 as our loop increments by `rol/col_incr` first
            # so we'd be missing the first column if we use `0` instead
            starting_col=-1,
            starting_row_incr=0,
            starting_col_incr=1,
        )

    def part_two(self, inp: str) -> int:
        grid = inp.splitlines()

        n_rows = len(grid)
        n_cols = len(grid[0])

        return max(
            max(
                max(
                    # start of row
                    self._get_energized_amount(grid, row, -1, 0, 1),
                    # end of row
                    self._get_energized_amount(grid, row, n_cols, 0, -1),
                )
                for row in range(n_rows)
            ),
            max(
                max(
                    # start of column
                    self._get_energized_amount(grid, -1, col, 1, 0),
                    # end of column
                    self._get_energized_amount(grid, n_rows, col, -1, 0),
                )
                for col in range(n_cols)
            ),
        )

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 7798
        assert p2 == 8026