"""
Day 15: Lens Library

https://adventofcode.com/2023/day/15
"""
__all__ = ('Day15',)

from typing import ClassVar

from ..solution import Solution

class Day15(Solution):
    NAME: ClassVar[str] = 'Lens Library'

    def hash(self, string: str) -> int:
        current_value = 0
        for char in string:
            current_value += ord(char)
            current_value *= 17
            current_value %= 256
        return current_value

    def part_one(self, inp: str) -> int:
        return sum(map(self.hash, inp.split(',')))

    def part_two(self, inp: str) -> int:
        boxes = [{} for _ in range(256)]

        for string in inp.split(','):
            sep = '-' if '-' in string else '='
            label, focus = string.split(sep)

            key = self.hash(label)
            if focus:
                boxes[key][label] = int(focus)
            else:
                boxes[key].pop(label, None)
        return sum(
            i * j * focus
            for i, box in enumerate(boxes, 1)
            for j, focus in enumerate(box.values(), 1)
        )

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 508498
        assert p2 == 279116