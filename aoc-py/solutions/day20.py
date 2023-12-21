from __future__ import annotations
"""
Day 20: Pulse Propagation

https://adventofcode.com/2023/day/20
"""
__all__ = ('Day20',)

from typing import ClassVar
from itertools import count
from functools import reduce
from math import lcm

from ..solution import Solution

class RawModule:
    __slots__ = ('target', 'pulse', 'name')

    def __init__(self, target: str, pulse: bool, *, name: str = 'broadcaster') -> None:
        self.name = name
        self.target = target
        self.pulse = pulse

class Module:
    __slots__ = ('name', 'outputs')

    def __init__(self, name: str, outputs: list[str]) -> None:
        self.name = name
        self.outputs = outputs

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} name={self.name!r}>'

class Flipper(Module):
    __slots__ = ('name', 'outputs', 'status')

    def __init__(self, name: str, outputs: list[str]) -> None:
        super().__init__(name, outputs)

        # on or off
        self.status = False

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} name={self.name!r}> status={self.status}'

class Conjunction(Module):
    __slots__ = ('name', 'outputs', 'memory')

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
    
    def _run_modules(self, to_check: list[RawModule], module: Module, source: str, pulse: bool) -> None:
        if isinstance(module, Flipper) and not pulse:
            module.status = not module.status
            pulse = module.status
        elif isinstance(module, Conjunction):
            module.memory[source] = pulse
            pulse = not all(module.memory.values()) # all pulses are high
        else:
            return

        for output in module.outputs:
            to_check.append(RawModule(output, pulse, name=module.name))

    def part_one(self, inp: str) -> int:
        modules, broadcast_targets = self._parse_input(inp)

        n_low = 0
        n_high = 0
        for _ in range(1000):
            n_low += 1
            to_check = [
                RawModule(target, pulse=False)
                for target in broadcast_targets
            ]
            while to_check:
                source = to_check.pop(0)

                if source.pulse:
                    n_high += 1
                else:
                    n_low += 1

                if module := modules.get(source.target):
                    self._run_modules(to_check, module, source.name, source.pulse)
        return n_low * n_high

    def part_two(self, inp: str) -> int:
        modules, broadcast_targets = self._parse_input(inp)

        feed = next(filter(
            lambda name: 'rx' in modules[name].outputs,
            modules,
        ))
        seen = {
            name: False
            for name, module in modules.items()
            if feed in module.outputs
        }
        cycle_lengths = {}

        for n_presses in count(start=1):
            to_check = [
                RawModule(target, pulse=False)
                for target in broadcast_targets
            ]
            while to_check:
                source = to_check.pop(0)

                if module := modules.get(source.target):
                    if module.name == feed and source.pulse:
                        seen[source.name] = True

                        if source not in cycle_lengths:
                            cycle_lengths[source] = n_presses
                        
                        if all(seen.values()):
                            return reduce(lcm, cycle_lengths.values(), 1)
                    self._run_modules(to_check, module, source.name, source.pulse)
        raise ValueError()

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 680278040
        assert p2 == 243548140870057