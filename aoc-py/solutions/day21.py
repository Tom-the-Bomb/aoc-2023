"""
Day 1: Trebuchet?!

https://adventofcode.com/2023/day/1
"""
__all__ = ('Day1',)

from typing import ClassVar

from ..solution import Solution

class Day1(Solution):
    NAME: ClassVar[str] = 'Trebuchet?!'
    NUM_MAP: ClassVar[dict[str, str]] = {
        'one': '1',
        'two': '2',
        'three': '3',
        'four': '4',
        'five': '5',
        'six': '6',
        'seven': '7',
        'eight': '8',
        'nine': '9'
    }

    def part_one(self, inp: str) -> int:
        ...

    def part_two(self, inp: str) -> int:
        ...

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 53651
        assert p2 == 53894