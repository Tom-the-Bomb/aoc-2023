from __future__ import annotations
"""
Day 22: Sand Slabs

https://adventofcode.com/2023/day/22
"""
__all__ = ('Day22',)

from typing import ClassVar
from collections import defaultdict

from ..solution import Solution

class Point:
    """Represents a coordinate/point in 3D space (x, y, z)

    A brick contains a pair of these, representing 2 ends of the brick
    """
    __slots__ = ('x', 'y', 'z')

    def __init__(self, x: int, y: int, z: int) -> None:
        self.x = x
        self.y = y
        self.z = z

    @classmethod
    def from_str(cls, raw: str) -> Point:
        """Parses from given input"""
        x, y, z = raw.split(',')
        return cls(int(x), int(y), int(z))

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} ({self.x}, {self.y}, {self.z})>'

    def __eq__(self, other: Point) -> bool:
        return self.x == other.x and self.y == other.y and self.z == other.z

    def __ne__(self, other: Point) -> bool:
        return not self == other

class Brick:
    """Represents a brick

    Stores information about its provided 2 coordinate points
    """
    __slots__ = ('bottom', 'top')

    def __init__(self, bottom: Point, top: Point) -> None:
        self.bottom = bottom
        self.top = top

    @classmethod
    def from_str(cls, raw: str) -> Brick:
        """Parses from given input"""
        bottom, top = raw.split('~')
        return cls(
            Point.from_str(bottom),
            Point.from_str(top),
        )

    @property
    def height(self) -> int:
        """Returns the height / thickness of the brick itself, difference in z-values"""
        return self.top.z - self.bottom.z

    def __repr__(self) -> str:
        return f'<{self.__class__.__name__} bottom={self.bottom!r} top={self.top!r}>'

    def overlaps_xy(self, other: Brick) -> bool:
        """Checks whether or not the bricks overlap when looking from a birds-eye view

        gives us insight into whether or not there even is potential for them to support each other
        by telling us whether or not they are above each other
        """
        return (
            max(self.bottom.x, other.bottom.x) <= min(self.top.x, other.top.x)
            and max(self.bottom.y, other.bottom.y) <= min(self.top.y, other.top.y)
        )

    def __eq__(self, other: Brick) -> bool:
        return self.bottom == other.bottom and self.top == other.top

    def __ne__(self, other: Brick) -> bool:
        return not self == other

    def __ge__(self, other: Brick) -> bool:
        return self.bottom.z >= other.bottom.z

    def __gt__(self, other: Brick) -> bool:
        return self.bottom.z > other.bottom.z

    def __le__(self, other: Brick) -> bool:
        return self.bottom.z <= other.bottom.z

    def __lt__(self, other: Brick) -> bool:
        return self.bottom.z < other.bottom.z

class Day22(Solution):
    NAME: ClassVar[str] = 'Sand Slabs'

    def _get_support_mappings(self, inp: str) -> tuple[list[Brick], dict[int, set[int]], dict[int, set[int]]]:
        bricks = [Brick.from_str(line) for line in inp.splitlines()]
        # sort from lowest-z to highest-z (height)
        bricks.sort()

        # drop bricks
        for i, brick in enumerate(bricks):
            z = 1
            # all bricks under `brick`
            for lower_brick in bricks[:i]:
                # `brick` overlaps with `lower_brick` on the x-y plane,
                # since `lower_brick` is under `brick`, and they overlap
                # (overlap = `lower_brick` can support `brick`)
                if brick.overlaps_xy(lower_brick):
                    # this indicates that `brick` cannot fall any lower than `lower_brick`'s top height + 1
                    #
                    # repeat this process for all bricks under `brick` and
                    # get the maximum height it can fall down to of all the bricks
                    # giving us where `brick` will fall down to ultimately
                    z = max(z, lower_brick.top.z + 1)
            # `z` represents what the brick's BOTTOM will drop down to
            #
            # set z-level of the top of the brick to `z` + the height offset
            brick.top.z = brick.height + z
            # drop the z-level of the brick's bottom to `z`
            brick.bottom.z = z
        bricks.sort()

        # 2 way mapping:
        # maps: brick -> what other bricks that brick supports
        supports = defaultdict(set)
        # maps: brick -> what supports that brick
        supported_by = defaultdict(set)

        for a, brick in enumerate(bricks):
            # all bricks beneath brick `a`
            for b, lower_brick in enumerate(bricks[:a]):
                if (
                    # bricks overlap on the x-y plane: potential for support
                    lower_brick.overlaps_xy(brick)
                    # the brick's bottom's z-value is exactly equal to the z-value + 1 of the top of the brick below it
                    # indicating that `lower_brick` is touching/supporting `brick`
                    #   `b` brick is supporting `a`
                    and brick.bottom.z == lower_brick.top.z + 1
                ):
                    supports[b].add(a)
                    supported_by[a].add(b)
        return bricks, supports, supported_by

    def part_one(self, inp: str) -> int:
        bricks, supports, supported_by = self._get_support_mappings(inp)

        return sum(
            1
            # for all bricks (`b`) check:
            for b in range(len(bricks))
            # there are more than 1 brick supporting `a`
            # indicating there is another brick OTHER than `b` to support `a`
            # therefore, we can safely disintegrate `b` as `a` will still be supported
            #
            # for all bricks (`a`) that `b` supports
            if all(len(supported_by[a]) > 1 for a in supports[b])
        )

    def part_two(self, inp: str) -> int:
        bricks, supports, supported_by = self._get_support_mappings(inp)
        total = 0

        for b in range(len(bricks)):
            # all bricks `a` that are SOLELY supported by `b`
            # which will also be all the bricks that will fall once `b` disintegrates
            #
            # we go through all bricks `b` supports and include it (`a`)
            # if `a` is supported by only 1 brick, which has to be `b`
            to_check = [
                a for a in supports[b] if len(supported_by[a]) == 1
            ]
            # bricks that are going to fall
            falling = set(to_check + [b])

            while to_check:
                to_fall = to_check.pop(0)

                # go through all bricks that are supported by `to_fall`
                # that is also NOT already falling
                for a in supports[to_fall].difference(falling):
                    # if everything that supports `a` is ALL falling
                    if supported_by[a].issubset(falling):
                        to_check.append(a)
                        falling.add(a)
            # we need to substract `1` as we included `b` the original disintegrated brick
            total += len(falling) - 1
        return total

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 459
        assert p2 == 75784