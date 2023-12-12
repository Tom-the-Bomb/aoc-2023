"""
Day 12: Cosmic Expansion

https://adventofcode.com/2023/day/12
"""
__all__ = ('Day12',)

from typing import ClassVar
from itertools import product, groupby
import re

from ..solution import Solution

class Day12(Solution):
    NAME: ClassVar[str] = 'Cosmic Expansion'

    KNOWN = ('.', '#')

    def part_one(self, inp: str) -> int:
        total = 0
        for record in inp.splitlines():
            record, counts = record.split()
            counts = tuple(map(int, counts.split(',')))

            n_unknown = record.count('?')
            record = record.replace('?', '{}')
            for combination in product(
                ('.', '#'),
                repeat=n_unknown,
            ):
                trial = record.format(*combination)
                total += tuple(map(len, re.findall("#+", trial))) == counts
        return total
            
    def part_two(self, inp: str) -> int:
        ...

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        ...