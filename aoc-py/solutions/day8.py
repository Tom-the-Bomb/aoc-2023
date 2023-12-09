from __future__ import annotations

"""
Day 8: Haunted Wasteland

https://adventofcode.com/2023/day/8
"""
__all__ = ('Day8',)

from typing import ClassVar
from itertools import cycle
from math import lcm

from ..solution import Solution

class Day8(Solution):
    NAME: ClassVar[str] = 'Haunted Wasteland'

    def _parse_node(self, node: str) -> tuple[str, tuple[str, ...]]:
        key, children = node.replace(' ', '').split('=')
        return (
            key,
            tuple(children.strip('()').split(',')),
        )
    
    def part_one(self, inp: str) -> int:
        instructions, nodes = inp.split('\n\n')
        nodes = dict(map(self._parse_node, nodes.splitlines()))

        count = 0
        left, right = nodes['AAA']
        for instruction in cycle(instructions):
            count += 1
            if (key := left if instruction == 'L' else right) == 'ZZZ':
                break
            left, right = nodes[key]
        return count

    def part_two(self, inp: str) -> int:
        instructions, nodes = inp.split('\n\n')
        nodes = dict(map(self._parse_node, nodes.splitlines()))

        a_nodes = [
            value for key, value in nodes.items() if key.endswith('A')
        ]
        counts = []
        for left, right in a_nodes:
            count = 0
            for instruction in cycle(instructions):
                count += 1
                if (key := left if instruction == 'L' else right).endswith('Z'):
                    break
                left, right = nodes[key]
            counts.append(count)
        return lcm(*counts)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 18727
        assert p2 == 18024643846273