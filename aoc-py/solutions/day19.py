from __future__ import annotations
"""
Day 19: Lavaduct Lagoon

https://adventofcode.com/2023/day/19
"""
__all__ = ('Day19',)
from typing import ClassVar, Callable
from operator import gt, lt
from functools import partial

from ..solution import Solution

class Rule:
    def __init__(
        self,
        key: str,
        target: str,
        condition: Callable[[int], bool],
        *,
        is_gt: bool,
    ) -> None:
        self.key = key
        self.target = target
        self.condition = condition
        self.is_gt = is_gt

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} key={self.key!r} target={self.target!r} condition={self.condition}>'

    def eval(self, data: dict[str, int]) -> bool:
        return self.condition(
            data[self.key],
        )

class Workflow:
    def __init__(self, name: str, default: str) -> None:
        self.name = name
        self.default = default

        self.rules: list[Rule] = []

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} rules={self.rules}>'

    def with_rules(self, rules: list[str]) -> Workflow:
        for rule in rules:
            condition, target = rule.split(':')
            is_gt = '>' in condition
            key, n = condition.split('>' if is_gt else '<')

            self.rules.append(Rule(
                key, target,
                partial(
                    lt if is_gt else gt,
                    int(n),
                ),
                is_gt=is_gt,
            ))
        return self

class Day19(Solution):
    NAME: ClassVar[str] = 'Lavaduct Lagoon'

    def _parse_workflows(self, raw: str) -> dict[str, Workflow]:
        workflows = {}

        for line in raw.splitlines():
            name, data = line.split('{')
            data = data.removesuffix('}').split(',')
            default = data.pop()

            workflows[name] = Workflow(name, default).with_rules(data)
        return workflows
    
    def _parse_parts(self, raw: str) -> list[dict[str, int]]:
        return [
            {(entry := _entry.split('='))[0]: int(entry[1]) for _entry in line
                .strip(r'{}')
                .split(',')
            }
            for line in raw.splitlines()
        ]
    
    def _accept(
        self,
        workflows: dict[str, Workflow],
        group: dict[str, int],
        target: str = 'in'
    ) -> bool:
        if target == 'A':
            return True
        if target == 'R':
            return False
        
        workflow = workflows[target]

        for rule in workflow.rules:
            if rule.eval(group):
                return self._accept(workflows, group, rule.target)
        return self._accept(workflows, group, workflow.default)

    def part_one(self, inp: str) -> int:
        workflows, parts = inp.split('\n\n')
        workflows = self._parse_workflows(workflows)
        parts = self._parse_parts(parts)

        return sum(
            sum(group.values())
            for group in parts
            if self._accept(workflows, group)
        )

    def part_two(self, inp: str) -> int:
        workflows, _ = inp.split('\n\n')
        workflows = self._parse_workflows(workflows)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        #print('Part 2:', p2 := self.part_two(inp))

        #assert p1 == 61965
        #assert p2 == 40343619199142