from __future__ import annotations
"""
Day 17: Clumsy Crucible

https://adventofcode.com/2023/day/17
"""
__all__ = ('Day17',)

from typing import ClassVar
from heapq import heapify, heappop, heappush

from ..solution import Solution

class Day17(Solution):
    NAME: ClassVar[str] = 'Clumsy Crucible'
    ALL_DIRECTIONS: ClassVar[tuple[tuple[int, int], ...]]= (
        (0, -1), # up
        (0, 1),  # down
        (-1, 0), # left
        (1, 0),  # right
    )

    def find_path(self, inp: str, *, is_part_two: bool) -> int:
        grid = [
            [int(block) for block in row]
            for row in inp.splitlines()
        ]
        n_rows = len(grid)
        n_cols = len(grid[0])

        traversed = set()
        heapify(to_check := [])
        # start at (0, 0) with a total heat of 0, and no direction
        heappush(to_check, (0, 0, (0, 0), (0, 0)))

        # part 1: we can only move max 3 times in one direction
        # part 2: we can only move max 10 times in one direction
        max_dir_traversed = 10 if is_part_two else 3

        while to_check:
            heat, *set_entry = heappop(to_check)
            dir_traversed, (row, col), (row_incr, col_incr) = set_entry

            if (
                # we've reched the end (bottom-right corner)
                row == n_rows - 1
                and col == n_cols - 1
                # part 2: we need to have not turned for at least 4 blocks before we can end
                and (dir_traversed >= 4 if is_part_two else True)
            ):
                return heat

            if (set_entry := tuple(set_entry)) not in traversed:
                directions = []
                if (
                    # part 2: we can only turn after moving at least 4 times in the same direction
                    dir_traversed >= 4
                    # account for being on starting block
                    or row_incr == 0
                    and col_incr == 0
                    if is_part_two else True
                ):
                    directions += [
                        ((new_row_incr, new_col_incr), True)
                        for new_row_incr, new_col_incr in self.ALL_DIRECTIONS
                        if (
                            # ensure we are not turning in the SAME direction
                            (new_row_incr != row_incr or new_col_incr != col_incr)
                            # ensure we are not turning in the direct OPPOSITE direction
                            and (new_row_incr != -row_incr or new_col_incr != -col_incr)
                        )
                    ]

                if (
                    # we can keep going in the same direction!
                    dir_traversed < max_dir_traversed
                    # cannot do this on starting block: no direction
                    and (row_incr != 0 or col_incr != 0)
                ):
                    directions.append(((row_incr, col_incr), False))

                for (row_incr, col_incr), changed_directions in directions:
                    new_row = row + row_incr
                    new_col = col + col_incr

                    if new_row in range(n_rows) and new_col in range(n_cols):
                        heappush(
                            to_check,
                            (
                                # add current heat of block
                                heat + grid[new_row][new_col],
                                # if we've changed directions, reset `dir_traversed` counter to 1 (fresh direction)
                                # else we add to the counter
                                1 if changed_directions else dir_traversed + 1,
                                (new_row, new_col),
                                (row_incr, col_incr),
                            )
                        )
                traversed.add(set_entry)
        raise ValueError('Failed to traverse path')

    def part_one(self, inp: str) -> int:
        return self.find_path(inp, is_part_two=False)

    def part_two(self, inp: str) -> int:
        return self.find_path(inp, is_part_two=True)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 724
        assert p2 == 877