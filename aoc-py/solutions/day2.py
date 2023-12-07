"""
Day 1: Cube Conundrum

https://adventofcode.com/2023/day/2
"""
__all__ = ('Day2',)

from typing import ClassVar
from math import prod

from ..solution import Solution

class Day2(Solution):
    NAME: ClassVar[str] = 'Cube Conundrum'

    def part_one(self, inp: str) -> int:
        total = 0
        for line in inp.splitlines():
            game, plays = line.split(':', maxsplit=1)
            game = int(game.removeprefix('Game '))

            for play in plays.split(';'):
                colors = {
                    (parts := color.split(maxsplit=1))[1]: int(parts[0])
                    for color in play.split(',', maxsplit=2)
                }
                if (
                    colors.get('red', 0) > 12
                    or colors.get('green', 0) > 13
                    or colors.get('blue', 0) > 14
                ): break
            else:
                total += game
        return total

    def part_two(self, inp: str) -> int:
        total = 0
        for line in inp.splitlines():
            _, plays = line.split(':', maxsplit=1)

            mapping = {'red': 0, 'green': 0, 'blue': 0}
            for color in plays.replace(',', ';').split(';'):
                num, name = color.split(maxsplit=1)
                num = int(num)
                if num > mapping.get(name, 0):
                    mapping[name] = num
            total += prod(mapping.values())
        return total

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 2486
        assert p2 == 87984