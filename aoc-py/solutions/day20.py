from __future__ import annotations
"""
Day 20: Pulse Propagation

https://adventofcode.com/2023/day/20
"""
__all__ = ('Day20',)

from typing import ClassVar

from ..solution import Solution

class Module:
    def __init__(self, name: str, outputs: list[str]) -> None:
        self.name = name
        self.outputs = outputs

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} name={self.name!r}>'

class Flipper(Module):
    def __init__(self, name: str, outputs: list[str]) -> None:
        super().__init__(name, outputs)

        # on or off
        self.status = False

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} name={self.name!r}> status={self.status}'

class Conjunction(Module):
    def __init__(self, name: str, outputs: list[str]) -> None:
        super().__init__(name, outputs)

        self.memory: dict[str, bool] = {}

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} name={self.name!r}> memory={self.memory}'

class Day20(Solution):
    NAME: ClassVar[str] = 'Pulse Propagation'

    def _parse_input(self, inp: str) -> tuple[dict[str, Module], list[str]]:
        modules: dict[str, Module] = {}
        broadcast_targets: list[str] = []

        for line in inp.splitlines():
            name, targets = line.split('->')
            name = name.strip()
            targets = [target.strip() for target in targets.split(',')]

            if name == 'broadcaster':
                broadcast_targets = targets
            else:
                match name[0], name[1:]:
                    case '%', name:
                        modules[name] = Flipper(name, targets)
                    case '&', name:
                        modules[name] = Conjunction(name, targets)

        for name, module in modules.items():
            for output in module.outputs:
                if (
                    (module := modules.get(output))
                    and isinstance(module, Conjunction)
                ):
                    module.memory[name] = False
        return modules, broadcast_targets

    def part_one(self, inp: str) -> int:
        modules, broadcast_targets = self._parse_input(inp)

        n_low = 0
        n_high = 0
        for _ in range(1000):
            n_low += 1
            to_check: list[tuple[str, str, bool]] = [
                ('broadcaster', target, False)
                for target in broadcast_targets
            ]

            while to_check:
                source, target, pulse = to_check.pop(0)

                if pulse:
                    n_high += 1
                else:
                    n_low += 1

                if module := modules.get(target):
                    if isinstance(module, Flipper) and not pulse:
                        module.status = not module.status
                        pulse = module.status
                    elif isinstance(module, Conjunction):
                        module.memory[source] = pulse
                        pulse = not all(module.memory.values()) # all pulses are high
                    else:
                        continue

                    for output in module.outputs:
                        to_check.append((module.name, output, pulse))
        return n_low * n_high

    def part_two(self, inp: str) -> int:
        ...

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        #print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 680278040
        #assert p2 == 243548140870057