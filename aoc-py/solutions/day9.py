from __future__ import annotations

"""
Day 9: Mirage Maintenance

https://adventofcode.com/2023/day/9
"""
__all__ = ('Day9',)

from typing import ClassVar

from ..solution import Solution

class Day9(Solution):
    NAME: ClassVar[str] = 'Mirage Maintenance'

    def _get_differences(self, sequence: list[int]) -> list[list[int]]:
        differences = [sequence]

        while sum(sequence) != 0:
            sequence = [
                sequence[i + 1] - sequence[i] for i in range(len(sequence) - 1)
            ]
            differences.insert(0, sequence)
        return differences

    def part_one(self, inp: str) -> int:
        total = 0
        for sequence in inp.splitlines():
            sequence = [int(t) for t in sequence.split()]

            differences = self._get_differences(sequence)
            for i, difference in enumerate(differences):
                new_diff = 0 if i == 0 else differences[i - 1][-1]

                differences[i].append(difference[-1] + new_diff)
            total += differences[-1][-1]
        return total

    def part_two(self, inp: str) -> int:
        total = 0
        for sequence in inp.splitlines():
            sequence = [int(t) for t in sequence.split()]

            differences = self._get_differences(sequence)
            for i, difference in enumerate(differences):
                new_diff = 0 if i == 0 else differences[i - 1][0]

                differences[i].insert(0, difference[0] - new_diff)
            total += differences[-1][0]
        return total

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 1647269739
        assert p2 == 864