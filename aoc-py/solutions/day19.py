from __future__ import annotations
"""
Day 19: Aplenty

https://adventofcode.com/2023/day/19
"""
__all__ = ('Day19',)

from typing import ClassVar
from math import prod
from operator import gt, lt
from functools import partial

from ..solution import Solution

class Rule:
    """Represents a rule in a workflow
    contains the key, target and a callable that is the condition
    that returns a boolean value from the evaluated inequality

    i.e. 'x>2662:A'
        key = 'x'
        target = 'A'
        condition = (a, b=2662) -> a > b
    """
    def __init__(
        self,
        key: str,
        target: str,
        condition: partial[bool],
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
        """Evaluates the condition, by fetching the rating value that corresponds to `self.key`
        from the given data mapping `data`
        """
        return self.condition(
            data[self.key],
        )

class Workflow:
    """Represents a workflow

    Contains a `list[Rule]` of all the rules
    and an attribute `default` that contains the value of the default target
    """
    def __init__(self, name: str, default: str) -> None:
        self.name = name
        self.default = default

        self.rules: list[Rule] = []

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} rules={self.rules}>'

    def with_rules(self, rules: list[str]) -> Workflow:
        """Parses each individual rule in the workflow

        i.e. 'x>2662:A'
            key = 'x'
            target = 'A'
            n = 2662
        """
        for rule in rules:
            condition, target = rule.split(':')
            is_gt = '>' in condition
            key, rhs = condition.split('>' if is_gt else '<')

            self.rules.append(Rule(
                key, target,
                partial(
                    # partial puts the argument `n`first before the custom ones
                    # hence we need to reverse it `x>2` -> `(a, b=n) -> n < a`
                    lt if is_gt else gt,
                    int(rhs),
                ),
                is_gt=is_gt,
            ))
        return self

class Day19(Solution):
    NAME: ClassVar[str] = 'Aplenty'

    def _parse_workflows(self, raw: str) -> dict[str, Workflow]:
        workflows = {}

        for line in raw.splitlines():
            name, data = line.split('{')
            data = data.removesuffix('}').split(',')
            default = data.pop()

            workflows[name] = Workflow(name, default).with_rules(data)
        return workflows

    def _parse_parts(self, raw: str) -> list[dict[str, int]]:
        """Parses the 2nd block of the input, which are the `part ratings`
        into a dictionary mapping the name to the rating

        Each line, i.e. "{x=787,m=2655,a=1222,s=2876}" is one 'group' and a list of the groups are returned
        the above example will get parsed into: `{'x': 787, 'm': 2655, 'a': 1222, 's': 2876}
        """
        return [
            {(entry := _entry.split('='))[0]: int(entry[1]) for _entry in line
                .strip(r'{}')
                .split(',')
            }
            for line in raw.splitlines()
        ]

    def _is_accepted(
        self,
        workflows: dict[str, Workflow],
        group: dict[str, int],
        target: str = 'in',
    ) -> bool:
        """Recursively runs a workflow, computing each rule and calling this method again
        with the new target until we have reached a `rejection` or `acceptance`,
        in which then we return `bool` indicating whether or not it was accepted

        Used in Part 1
        """
        if target == 'A':
            return True
        if target == 'R':
            return False

        workflow = workflows[target]

        for rule in workflow.rules:
            if rule.eval(group):
                return self._is_accepted(workflows, group, rule.target)
        # evaluate fallback value after all the rules are evaluated
        return self._is_accepted(workflows, group, workflow.default)

    def _count_range(
        self,
        workflows: dict[str, Workflow],
        ranges: dict[str, range],
        target: str = 'in',
    ) -> int:
        """Counts the number of possible ratings that satisfy the workflows within [1, 4000]

        Used in Part 2
        """
        if target == 'A':
            return prod(len(interval) for interval in ranges.values())
        if target == 'R':
            return 0

        total = 0
        workflow = workflows[target]

        for rule in workflow.rules:
            interval = ranges[rule.key]
            # rhs in the inequality condition
            rhs = rule.condition.args[0]

            true_range = range(
                # if the condition is `key > rhs`
                # for all `key` in between [a, b) evaluates to `true`
                # when `key` is between [a, b) if `a > rhs` else (rhs, b)
                max(rhs + 1, interval.start),
                interval.stop,
            ) if rule.is_gt else range(
                # if the condition is `key < rhs`
                # for all `key` in between [a, b) evaluates to `true`
                # when `key` is between [a, b) if `b < rhs` else [a, rhs)
                interval.start,
                min(rhs, interval.stop),
            )
            # complementing the above, the inverse is when the conditions are false:
            false_range = range(
                # if the condition is `key > rhs`
                # for all `key` in between [a, b) evaluates to `false`
                # when `key` is between [a, b) if `b < rhs` else [a, rhs]
                #
                # * the same as the `true` condition for `key < rhs` but inclusive on the end
                # since `a < b` is false when `b >= a` (not) `b > a`
                interval.start,
                min(rhs + 1, interval.stop)
            ) if rule.is_gt else range(
                # if the condition is `key > rhs`
                # for all `key` in between [a, b) evaluates to `false`
                # when `key` is between [a, b) if `a > rhs` else [rhs, b)
                #
                # * the same as the `true` condition for `key > rhs` but inclusive on the start
                max(rhs, interval.start),
                interval.stop
            )

            if true_range:
                copy = ranges.copy()
                copy[rule.key] = true_range
                total += self._count_range(workflows, copy, rule.target)
            if false_range:
                ranges[rule.key] = false_range
            else:
                break
        else:
            total += self._count_range(workflows, ranges, workflow.default)
        return total

    def part_one(self, inp: str) -> int:
        workflows, parts = inp.split('\n\n')
        workflows = self._parse_workflows(workflows)
        parts = self._parse_parts(parts)

        # returns sum of the ratings of all accepted parts
        return sum(
            # values() -> ratings
            sum(group.values())
            for group in parts
            if self._is_accepted(workflows, group)
        )

    def part_two(self, inp: str) -> int:
        workflows, _ = inp.split('\n\n')
        workflows = self._parse_workflows(workflows)

        return self._count_range(
            workflows,
            {
                'x': range(1, 4001),
                'm': range(1, 4001),
                'a': range(1, 4001),
                's': range(1, 4001),
            }
        )

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 362930
        assert p2 == 116365820987729