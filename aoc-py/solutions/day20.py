from __future__ import annotations
"""
Day 20: Pulse Propagation

https://adventofcode.com/2023/day/20
"""
__all__ = ('Day20',)

from typing import ClassVar
from itertools import count
from math import lcm

from ..solution import Solution

class Destination:
    """Represents a destination when processing the button presses and signals"""
    __slots__ = ('target', 'pulse', 'name')

    def __init__(self, target: str, pulse: bool, *, name: str = 'broadcaster') -> None:
        self.name = name
        self.target = target
        self.pulse = pulse

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} name={self.name!r} target={self.target!r} pulse={self.pulse}>'

class Module:
    """Base Module Class"""
    __slots__ = ('name', 'outputs')

    def __init__(self, name: str, outputs: list[str]) -> None:
        self.name = name
        self.outputs = outputs

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} name={self.name!r}>'

class Flipper(Module):
    """Flipper type module, a module with a prefix of '%'

    Stores it 'on' or 'off' status in `self.status`
    """
    __slots__ = ('name', 'outputs', 'status')

    def __init__(self, name: str, outputs: list[str]) -> None:
        super().__init__(name, outputs)

        # on or off
        self.status = False

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} name={self.name!r}> status={self.status}>'

class Conjunction(Module):
    """Conjunction type module, a module with a prefix of '&'

    Stores the most recent pulses received from each of their connected input modules
    inside `self.memory` as a mapping of `module_name -> pulse`
    """
    __slots__ = ('name', 'outputs', 'memory')

    def __init__(self, name: str, outputs: list[str]) -> None:
        super().__init__(name, outputs)

        self.memory: dict[str, bool] = {}

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} name={self.name!r}> memory={self.memory}>'

class Day20(Solution):
    NAME: ClassVar[str] = 'Pulse Propagation'

    def _parse_input(self, inp: str) -> tuple[dict[str, Module], list[str]]:
        """Parses the input

        returns a mapping of modules and a list of our starting targets
        """
        modules: dict[str, Module] = {}
        broadcast_targets: list[str] = []

        for line in inp.splitlines():
            name, targets = line.split('->')
            name = name.strip()
            targets = [target.strip() for target in targets.split(',')]

            if name == 'broadcaster':
                # unique broadcaster module, our starting point
                broadcast_targets = targets
            else:
                match name[0], name[1:]:
                    case '%', name:
                        modules[name] = Flipper(name, targets)
                    case '&', name:
                        modules[name] = Conjunction(name, targets)

        # grab all modules: treat as input modules
        for name, module in modules.items():
            # grab each input module's outputs
            # check if it's a conjunction module (remember's its input modules)
            for output in module.outputs:
                if (
                    (module := modules.get(output))
                    and isinstance(module, Conjunction)
                ):
                    # fills all conjunction modules with their connected input modules
                    # defaulting to false to begin
                    module.memory[name] = False
        return modules, broadcast_targets

    def _run_modules(self, destinations: list[Destination], module: Module, source: str, pulse: bool) -> None:
        """Runs a single iteration of all the modules' pulse sending process"""

        if isinstance(module, Flipper) and not pulse:
            module.status = not module.status
            pulse_to_send = module.status
        elif isinstance(module, Conjunction):
            module.memory[source] = pulse
            # Sends low pulse if all remembered pulses are high otherwise sends a high pulse
            pulse_to_send = not all(module.memory.values())
        else:
            return

        for output in module.outputs:
            destinations.append(
                Destination(output, pulse_to_send, name=module.name)
            )

    def part_one(self, inp: str) -> int:
        modules, broadcast_targets = self._parse_input(inp)

        n_low = 0
        n_high = 0
        for _ in range(1000):
            n_low += 1
            destinations = [
                Destination(target, pulse=False)
                for target in broadcast_targets
            ]
            while destinations:
                source = destinations.pop(0)

                if source.pulse:
                    n_high += 1
                else:
                    n_low += 1

                if module := modules.get(source.target):
                    self._run_modules(destinations, module, source.name, source.pulse)
        return n_low * n_high

    def part_two(self, inp: str) -> int:
        modules, broadcast_targets = self._parse_input(inp)

        # finds the module that has an output into 'rx'
        rx_feeder = next(filter(
            lambda name: 'rx' in modules[name].outputs,
            modules,
        ))
        # backtrack and find all modules that have an output that is the feeder into 'rx'
        seen = {
            name: False
            for name, module in modules.items()
            if rx_feeder in module.outputs
        }
        # contains all modules in which the module outputs to the module that feeds into 'rx'
        # maps to the amount of button presses each one takes to complete
        press_amounts = {}

        for n_presses in count(start=1):
            destinations = [
                Destination(target, pulse=False)
                for target in broadcast_targets
            ]
            while destinations:
                source = destinations.pop(0)

                if module := modules.get(source.target):
                    if module.name == rx_feeder and source.pulse:
                        seen[name := source.name] = True

                        if name not in press_amounts:
                            press_amounts[name] = n_presses

                        if all(seen.values()):
                            # we have checked all the modules that feed into the module
                            # we can then return the result
                            # lowest common multiple of all buttonn press amounts of the modules that feed into the feeder of 'rx'
                            return lcm(*press_amounts.values())
                    self._run_modules(destinations, module, source.name, source.pulse)
        raise ValueError('No solution found')

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 680278040
        assert p2 == 243548140870057