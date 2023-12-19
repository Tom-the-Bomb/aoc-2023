from __future__ import annotations
"""
Day 18: Lavaduct Lagoon

https://adventofcode.com/2023/day/18
"""
__all__ = ('Day18',)

from typing import (
    Iterator,
    ClassVar,
    TypeAlias,
    TYPE_CHECKING,
)

from ..solution import Solution

if TYPE_CHECKING:
    Points: TypeAlias = list[tuple[int, int]]
    Entry: TypeAlias = tuple[int, tuple[int, int]]

class Day18(Solution):
    NAME: ClassVar[str] = 'Lavaduct Lagoon'

    def _parse_p1(self, line: str) -> Entry:
        """Parses each line from Part 1:
        - ignores the hexadecimal number

        (i.e.) 'R 6 (#70c710)' -> (6, (1, 0))
        """
        direction, dist, *_ = line.split()
        return (
            int(dist),
            {
                'U': (0, 1),
                'D': (0, -1),
                'L': (-1, 0),
                'R': (1, 0),
            }.get(direction, (0, 0))
        )
    
    def _parse_p2(self, line: str) -> Entry:
        """Parses each line from Part 2:
        - ignores the provided direction and distance

        the direction and distance are now parsed from the hexadecimal
        (i.e.) 'R 6 (#70c710)'
            direction = last digit (in this case 0)
                where 0 -> R | 1 -> D | 2 -> L | 3 -> U
            distance = first 5 digits (in this case 70c71)
                (converted to decimal (base 10) from hex (base 16))
            -> (461937, (1, 0))
        """
        *_, hexcode = line.split()
        hexcode = hexcode.strip('(#)')
        dist, direction = hexcode[:-1], hexcode[-1]
        return (
            int(dist, 16),
            [
                (1, 0),
                (0, -1),
                (-1, 0),
                (0, 1),
            ][int(direction)]
        )

    def _shoelace(self, points: Points) -> int:
        """Gets the interior enclosed area of a polygon
        given the points in clockwise order

        Since the way our data is structured, the coordinates represent the tiles themselves and not points
        the shoelace formula will return only the cartesian internal area

        this value is not of use to us at all directly as it works for cartesian polygons
        (it has no meaning in this application)
        but it is only useful to input into `Pick's Theorem` as `A`
        
        <https://en.wikipedia.org/wiki/Shoelace_formula>
        """
        return abs(
            sum(
                x1 * y2 - x2 * y1
                for (x1, y1), (x2, y2) in zip(
                    points,
                    [*points[1:], points[0]]
                )
            ) // 2
        )

    def _get_area(self, data: Iterator[Entry]) -> int:
        """Maps out the points (in cartesian) of the lagoon polygon starting at (0, 0)
        and following each instruction given the distance and direction to travel in

        Uses the Shoelace Formula + Picks's Theorem

        <https://en.wikipedia.org/wiki/Pick%27s_theorem>
        """
        # points of the polygon (lagoon) in clockwise order starting at the origin
        points: Points = [(0, 0)]
        # the total perimeter of the shape
        # (amount of tiles dug/traversed)
        perimeter = 0
        for dist, (dir_x, dir_y) in data:
            last_x, last_y = points[-1]

            points.append((
                last_x + dir_x * dist,
                last_y + dir_y * dist,
            ))
            perimeter += dist

        # Pick's theorem
        # ==============
        # A = i + b/2 - 1
        # where A = the area of the polygon
        #       i = # of interior lattice points
        #       b = # of boundary lattice points
        # rearrange:
        #   A - b/2 + 1 = i
        # + b both sides
        #   A + b/2 + 1 = i + b
        # what we want is i + b actually and not A as our result
        #   A = what we get from applying the `shoelace formula``
        #   b = `perimeter` (which is the same as # of interior boundary lattice points in this case)
        #   i + b = internal lattice + boundary lattice = the desired output for us
        # 
        return self._shoelace(points) + perimeter // 2 + 1

    def part_one(self, inp: str) -> int:
        return self._get_area(
            map(self._parse_p1, inp.splitlines())
        )

    def part_two(self, inp: str) -> int:
        return self._get_area(
            map(self._parse_p2, inp.splitlines())
        )

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 61865
        assert p2 == 40343619199142