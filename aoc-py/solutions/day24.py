from __future__ import annotations
"""
Day 24: Never Tell Me The Odds

https://adventofcode.com/2024/day/24
"""
__all__ = ('Day24',)

from typing import ClassVar, Optional

import z3
import numpy as np
from sympy import Symbol, solve

from ..solution import Solution

class Hailstone:
    """Represents a hailstone, storing it's velocity and starting-position values"""
    def __init__(
        self,
        x_pos: int,
        y_pos: int,
        z_pos: int,
        x_vel: int,
        y_vel: int,
        z_vel: int,
    ) -> None:
        self.x_pos = x_pos
        self.y_pos = y_pos
        self.z_pos = z_pos

        self.x_vel = x_vel
        self.y_vel = y_vel
        self.z_vel = z_vel

        # convert to y = mx + b
        self.m = self.y_vel / self.x_vel
        # given y = mx + b
        # y - mx = b
        self.b = self.y_pos - self.m * self.x_pos

    @classmethod
    def from_str(cls, raw: str) -> Hailstone:
        pos, vel = raw.split('@')
        return cls(
            *map(int, pos.split(',')),
            *map(int, vel.split(',')),
        )

    def evaluate(self, x: float) -> float:
        """Evaluates f(x) = mx + b"""
        return self.m * x + self.b

    def in_domain(self, x: float, y: float) -> bool:
        """Ensures the hailstones do not intersect in the past (only when t >= 0)

        We represent the hailstones as a line of `mx + b`
        but in reality they are rays, starting at (x_pos, y_pox)

        This method checks whether or not a coordinate (x, y) is on said ray
        """
        return (
            # x-values check based on the sign of `x_vel`, which indicates which side of the `y_pos` we should be on
            x >= self.x_pos if self.x_vel > 0
            else x == self.x_pos if self.x_vel == 0
            else x <= self.x_pos
        ) and (
            # y-values check based on the sign of `y_vel`, which indicates which side of the `y_pos` we should be on
            y >= self.y_pos if self.y_vel > 0
            else y == self.y_pos if self.y_vel == 0
            else y <= self.y_pos
        )

    def intersection(self, other: Hailstone) -> Optional[tuple[float, float]]:
        try:
            # given 2 lines:
            #   f(x) = mx + b
            #   g(x) = nx + c
            # intersection: f(x) = g(x):
            #   mx + b = nx + c
            #   mx - nx = c - b
            #   x(m - n) = c - b
            #   x = (c - b) / (m - n)
            x = (other.b - self.b) / (self.m - other.m)
            return (x, self.evaluate(x))
        except ZeroDivisionError:
            # slopes are equal -> parallel lines -> no intersection
            return

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} [y = {self.m}x + {self.b}]>'

class Day24(Solution):
    NAME: ClassVar[str] = 'Never Tell Me The Odds'

    def part_one(self, inp: str) -> int:
        hailstones = [
            Hailstone.from_str(line) for line in inp.splitlines()
        ]
        total = 0
        for i, hs1 in enumerate(hailstones):
            for hs2 in hailstones[:i]:
                if point := hs1.intersection(hs2):
                    x, y = point
                    if (
                        hs1.in_domain(x, y)
                        and hs2.in_domain(x, y)
                        and 200000000000000 <= x <= 400000000000000
                        and 200000000000000 <= y <= 400000000000000
                    ):
                        total += 1
        return total

    def part_two_sympy(self, inp: str) -> int:
        """Part 2 solved algebraically using sympy"""
        hailstones = [
            Hailstone.from_str(line) for line in inp.splitlines()
        ]
        # Declare symbols:
        # They are all integers, and positions have to also be non-negative
        #
        # rock x, y, z starting positions
        x_pos = Symbol('x_pos', integer=True, nonnegative=True)
        y_pos = Symbol('y_pos', integer=True, nonnegative=True)
        z_pos = Symbol('z_pos', integer=True, nonnegative=True)
        # rock x, y, z velocities
        x_vel = Symbol('x_vel', integer=True)
        y_vel = Symbol('y_vel', integer=True)
        z_vel = Symbol('z_vel', integer=True)

        equations = []

        for n_eqs, hailstone in enumerate(hailstones, 1):
            equations.append(
                (x_pos - hailstone.x_pos) * (hailstone.y_vel - y_vel)    # type: ignore (cannot recognize arithmetic between `int` and `Symbol`)
                - (y_pos - hailstone.y_pos) * (hailstone.x_vel - x_vel)) # type: ignore
            equations.append(
                (y_pos - hailstone.y_pos) * (hailstone.z_vel - z_vel)   # type: ignore
                - (z_pos - hailstone.z_pos) * (hailstone.y_vel - y_vel) # type: ignore
            )
            # we have 3 or more equations -> try solve & solution exists
            if n_eqs >= 3 and (solutions := solve(equations)):
                solution = solutions[0]
                # solution = add up values of (x, y, z) for the rock's position
                return int(solution[x_pos] + solution[y_pos] + solution[z_pos])
        raise ValueError('No solution found for the system of equations')

    def part_two_linalg(self, inp: str) -> int:
        """Part 2 solved using linear algebra"""
        hs1, hs2, hs3, *_ = [Hailstone.from_str(line) for line in inp.splitlines()]

        # solving for `x` in Ax + b where (A = a)
        #
        # coeffcient matrix (A) of 6x6
        # 6 variables & 6 equations generated from positions and velocities
        # of 3 hailstone vectors that are linearly independent
        a = np.array(
            [
                [hs2.y_vel - hs1.y_vel, hs1.x_vel - hs2.x_vel, 0, hs1.y_pos - hs2.y_pos, hs2.x_pos - hs1.x_pos, 0],
                [hs3.y_vel - hs1.y_vel, hs1.x_vel - hs3.x_vel, 0, hs1.y_pos - hs3.y_pos, hs3.x_pos - hs1.x_pos, 0],
                [hs2.z_vel - hs1.z_vel, 0, hs1.x_vel - hs2.x_vel, hs1.z_pos - hs2.z_pos, 0, hs2.x_pos - hs1.x_pos],
                [hs3.z_vel - hs1.z_vel, 0, hs1.x_vel - hs3.x_vel, hs1.z_pos - hs3.z_pos, 0, hs3.x_pos - hs1.x_pos],
                [0, hs2.z_vel - hs1.z_vel, hs1.y_vel - hs2.y_vel, 0, hs1.z_pos - hs2.z_pos, hs2.y_pos - hs1.y_pos],
                [0, hs3.z_vel - hs1.z_vel, hs1.y_vel - hs3.y_vel, 0, hs1.z_pos - hs3.z_pos, hs3.y_pos - hs1.y_pos],
            ]
        )

        b = np.array([
            hs1.y_pos * hs1.x_vel - hs2.y_pos * hs2.x_vel - (hs1.x_pos * hs1.y_vel - hs2.x_pos * hs2.y_vel),
            hs1.y_pos * hs1.x_vel - hs3.y_pos * hs3.x_vel - (hs1.x_pos * hs1.y_vel - hs3.x_pos * hs3.y_vel),
            hs1.z_pos * hs1.x_vel - hs2.z_pos * hs2.x_vel - (hs1.x_pos * hs1.z_vel - hs2.x_pos * hs2.z_vel),
            hs1.z_pos * hs1.x_vel - hs3.z_pos * hs3.x_vel - (hs1.x_pos * hs1.z_vel - hs3.x_pos * hs3.z_vel),
            hs1.z_pos * hs1.y_vel - hs2.z_pos * hs2.y_vel - (hs1.y_pos * hs1.z_vel - hs2.y_pos * hs2.z_vel),
            hs1.z_pos * hs1.y_vel - hs3.z_pos * hs3.y_vel - (hs1.y_pos * hs1.z_vel - hs3.y_pos * hs3.z_vel),
        ])

        return round(
            sum(np.linalg.solve(a, b)[:3]) # type: ignore (`solve` function is not known)
        )

    def part_two(self, inp: str) -> int:
        """Part 2 solved using z3-solver"""

        # rock position and velocity variables
        x_pos, y_pos, z_pos, x_vel, y_vel, z_vel = z3.Reals(
            'x_pos, y_pos, z_pos, x_vel, y_vel, z_vel'
        )
        solver = z3.Solver()

        for i, line in enumerate(inp.splitlines()):
            hailstone = Hailstone.from_str(line)

            time = z3.Real(f't_{i}')

            solver.add(time >= 0)
            solver.add(
                # rock `x` position at `time`
                x_pos + time * x_vel
                # hailstone `y` position at `time`
                == hailstone.x_pos + time * hailstone.x_vel # type: ignore (addition of a `z3` variable to an `int` is not known)
            )
            solver.add(
                # rock `y` position at `time`
                y_pos + time * y_vel
                # hailstone `y` position at `time`
                == hailstone.y_pos + time * hailstone.y_vel # type: ignore
            )
            solver.add(
                # rock `z` position at `time`
                z_pos + time * z_vel
                # hailstone `z` position at `time`
                == hailstone.z_pos + time * hailstone.z_vel # type: ignore
            )
        assert solver.check() == z3.sat
        model = solver.model()

        return (
            model[x_pos].as_long()   # type: ignore (`as_long` method is not known)
            + model[y_pos].as_long() # type: ignore
            + model[z_pos].as_long() # type: ignore
        )

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p2 == self.part_two_sympy(inp)
        assert p2 == self.part_two_linalg(inp)

        assert p1 == 14672
        assert p2 == 646810057104753