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
        return [list(row) for row in zip(*grid)]
    
    def _reverse_rows(self, grid: Grid) -> Grid:
        return [row[::-1] for row in grid]

    def _tilt_lever(self, grid: Grid) -> Grid:
        for col in grid:
            for i, element in enumerate(col):
                if element == 'O':
                    for t in range(i, 0, -1):
                        if col[t - 1] == '.':
                            col[t - 1], col[t] = col[t], col[t - 1]
                        else:
                            break
        return grid
    
    def _tilt_north(self, grid: Grid) -> Grid:
        grid = self._transpose(grid)
        grid = self._tilt_lever(grid)
        return self._transpose(grid)
    
    def _tilt_south(self, grid: Grid) -> Grid:
        grid = self._transpose(grid)
        grid = self._reverse_rows(grid)
        grid = self._tilt_lever(grid)
        grid = self._reverse_rows(grid)
        return self._transpose(grid)
    
    def _tilt_west(self, grid: Grid) -> Grid:
        return self._tilt_lever(grid)
    
    def _tilt_east(self, grid: Grid) -> Grid:
        grid = self._reverse_rows(grid)
        grid = self._tilt_lever(grid)
        return self._reverse_rows(grid)
    
    def _cycle(self, grid: Grid) -> Grid:
        grid = self._tilt_north(grid)
        grid = self._tilt_west(grid)
        grid = self._tilt_south(grid)
        return self._tilt_east(grid)
    
    def _get_load(self, grid: Grid) -> int:
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
        
        cycles = [grid]
        start = 0
        while True:
            next_term = self._cycle(cycles[-1])
            if next_term in cycles:
                start = cycles.index(next_term)
                break
            else:
                cycles.append(next_term)
        period = len(cycles) - start
        for _ in range((1_000_000_000 - start) % period + start):
            grid = self._cycle(grid)
        return self._get_load(grid)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        #assert p1 == 35210
        #assert p2 == 31974