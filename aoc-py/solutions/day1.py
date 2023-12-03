"""
Day 1: Trebuchet?!

https://adventofcode.com/2023/day/1
"""
__all__ = ('Day1',)

from typing import ClassVar
from .. import run_day
from ..solution import Solution

class Day1(Solution):
    NAME = 'Trebuchet?!'
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
        return sum(
            int(
                (digits := [c for c in line if c.isdigit()])[0]
                + digits[-1]
            )
            for line in inp.splitlines()
        )

    def part_two(self, inp: str) -> int:
        for key in self.NUM_MAP:
            inp = inp.replace(key, key + self.NUM_MAP[key] + key)
            # accounts for overlapping words:
            #   i.e. 'twone'
            #   -> 'twone1one' (when key = 'one')
            #   -> '2ne1one'   (when key = 'two')
            #   -> 21 (correct)
            #
            # Without appending `key` to either side:
            #   'twone' -> '2ne' -> 2 (wrong)
        return self.part_one(inp)

    def run(self, inp: str) -> None:
        print('Part 1: ', p1 := self.part_one(inp))
        print('Part 2: ', p2 := self.part_two(inp))

        assert p1 == 53651
        assert p2 == 53894

if __name__ == '__main__':
    run_day(1)