"""
Day 10: Pipe Maze

https://adventofcode.com/2023/day/10
"""
__all__ = ('Day10',)

from typing import ClassVar

from ..solution import Solution

class Day10(Solution):
    NAME: ClassVar[str] = 'Pipe Maze'

    GO_LEFT_PIPES: ClassVar[set[str]] = {'-', 'J', '7'}
    GO_RIGHT_PIPES: ClassVar[set[str]] = {'-', 'L', 'F'}
    GO_UP_PIPES: ClassVar[set[str]] = {'|', 'J', 'L'}
    GO_DOWN_PIPES: ClassVar[set[str]] = {'|', '7', 'F'}

    DISPLAY_CHAR_MAPPING: ClassVar[dict[str, str]] = {
        '|': '│',
        '-': '─',
        'J': '┘',
        '7': '┐',
        'L': '└',
        'F': '┌',
    }

    def _get_starting_pos(self, grid: list[str]) -> tuple[int, int]:
        for i, row in enumerate(grid):
            for j, char in enumerate(row):
                if char == 'S':
                    return i, j
        raise ValueError("No 'S' character found in grid")

    def _get_loop(self, grid: list[str]) -> set[tuple[int, int]]:
        starting_coords = self._get_starting_pos(grid)

        n_rows = len(grid)
        n_cols = len(grid[0])

        loop = {starting_coords}
        to_check = [starting_coords]
        while to_check:
            curr_row, curr_col = to_check.pop(0)
            curr_tile = grid[curr_row][curr_col]

            neighbors = (
                (curr_row - 1, curr_col, # check 1 tile up
                    self.GO_UP_PIPES,
                    self.GO_DOWN_PIPES
                ),
                (curr_row + 1, curr_col, # check 1 tile down
                    self.GO_DOWN_PIPES,
                    self.GO_UP_PIPES
                ),
                (curr_row, curr_col - 1, # check 1 tile left
                    self.GO_LEFT_PIPES,
                    self.GO_RIGHT_PIPES
                ),
                (curr_row, curr_col + 1, # check 1 tile right
                    self.GO_RIGHT_PIPES,
                    self.GO_LEFT_PIPES
                ),
            )
            for next_row, next_col, pipes, co_pipes in neighbors:
                if (
                    # ensure the row of the next tile doesn't overflow
                    next_row in range(n_rows)
                    # ensure the column of the next tile doesn't overflow
                    and next_col in range(n_cols)
                    # ensure the current tile is able to go in the direction (`pipes`)
                    and curr_tile in pipes | {'S'}
                    # ensure the next tile is able to accept the direction (`co_pipes`)
                    # `co_pipes` are the opposite direction of `pipes`
                    # i.e. a pipe going up needs the next tile to have a pipe that goes down
                    and grid[next_row][next_col] in co_pipes
                    # ensure the tile has not been traversed yet i.e. we are going backwards in the loop
                    and (next_coord := (next_row, next_col)) not in loop
                ):
                    to_check.append(next_coord)
                    loop.add(next_coord)
        return loop

    def part_one(self, inp: str) -> int:
        grid = inp.splitlines()
        loop = self._get_loop(grid)
        # the halfway mark of the loop is the furthest point from the start
        return len(loop) // 2

    def part_two(self, inp: str) -> int:
        """Solution to Part 2 using Ray Casting

        <https://en.wikipedia.org/wiki/Point_in_polygon>
        """
        grid = inp.splitlines()
        loop = self._get_loop(grid)

        area = 0
        for i, row in enumerate(grid):
            downwards = 0
            upwards = 0
            for j, char in enumerate(row):
                if (i, j) in loop:
                    if char in self.GO_DOWN_PIPES: # | ┌ ┐, downward facing pipes
                        downwards += 1
                    if char in self.GO_UP_PIPES:   # | └ ┘, upward facing pipes
                        upwards += 1
                elif downwards % 2 == 1 and upwards % 2 == 1:
                    # we hit a possible candidate for a tile that can be in or out the loop
                    # since we have been keeping track of the amount of "walls" we have hit
                    # we check if the amount of downward pointing walls and upward pointing walls are all even
                    # if so we can say that this tile is inside the loop
                    area += 1
        return area

    def display_grid(self, inp: str) -> str:
        """Reformats the grid using unicode characters to help better visualize the pipes

        All pipes that are not part of the loop are replaced with a "."
        """
        grid = inp.splitlines()
        loop = self._get_loop(grid)
        grid = [list(row) for row in grid]

        for i, row in enumerate(grid):
            for j, char in enumerate(row):
                if (i, j) in loop:
                    grid[i][j] = self.DISPLAY_CHAR_MAPPING.get(char, char)
                else:
                    grid[i][j] = '.'

        return '\n' + '\n'.join(
            f"{i:>3}| {''.join(row)}"
            for i, row in enumerate(grid, 1)
        ) + '\n'

    def run(self, inp: str) -> None:
        print(self.display_grid(inp))

        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 7063
        assert p2 == 589