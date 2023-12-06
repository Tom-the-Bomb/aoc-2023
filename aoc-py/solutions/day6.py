"""
Day 6: Wait For It

https://adventofcode.com/2023/day/6
"""
__all__ = ('Day6',)

from math import prod, sqrt, ceil

from ..solution import Solution

class Day6(Solution):
    NAME = 'Wait For It'

    def _get_num_beats_bf(self, time: int, to_beat: int) -> int:
        """Brute forces the amount of ways in a race of `time` milliseconds
        to beat the record distance of `to_beat`

        Loops throught all possible values that one could hold the button for: `hold_time`
        and checks if the distance travelled is greater than the distance to beat
            = (speed * travel_time)
            where speed = hold_time
                (the amount of time the button is held directly equates to the speed)
            and travel_time = (time - hold_time)
                (The amount of time we have left to move; since we can only move after we stop holding the button)
            therefore our conditional is:
                hold_time * (time - hold_time) > to_beat
                and everytime it holds true, it increases the amount of combinations that beat `to_beat`
        """
        return sum(
            1 for hold_time in range(1, time)
            if hold_time * (time - hold_time) > to_beat
        )

    def _get_num_beats(self, time: int, to_beat: int) -> int:
        """Using Quadratics
    
        let d(x) be the distance travelled
            where x = the time the button is held
        d(x) = x * (t - x)
            where t = the amount of time the race lasts, t > 0
        d(x) = -x^2 + tx

        we need to calculate where d(x) > r
            where r = the record distance that we need to beat
        -x^2 + tx > r
        -x^2 + tx - r > 0

        let r1, r2 be the 2 roots of the equation respectively by the quadratic formula
            in ax^2 + bx + c:
                a = -1
                b = t
                c = -r
            therefore:
                r = -t +- √[t^2 - 4(-1)(-r)] / [2 * (-1)]
            and
                r1 = t - √(t^2 + 4r) / 2
                r2 = t + √(t^2 + 4r) / 2
        therefore:
            r1 < x < r2 is our solution to the inequality
            and total amount of winning possibilities would simply be ⌊r2⌋ - ⌈r1⌉ + 1
            which are the number of integers that satisfy the inequality
        """
        discriminant = time ** 2 - 4 * to_beat
        if discriminant < 0:
            return 0
        root1 = (time - sqrt(discriminant)) / 2
        root2 = (time + sqrt(discriminant)) / 2
        return int(root2) - ceil(root1) + 1

    def part_one(self, inp: str) -> int:
        time, distance = inp.splitlines()
        _, time = time.split(':', maxsplit=1)
        _, distance = distance.split(':', maxsplit=1)

        times = map(int, time.split())
        distances = map(int, distance.split())

        return prod(
            self._get_num_beats(time, to_beat)
            for time, to_beat in zip(times, distances)
        )

    def part_two(self, inp: str) -> int:
        time, distance = inp.replace(' ', '').splitlines()
        _, time = time.split(':', maxsplit=1)
        _, distance = distance.split(':', maxsplit=1)

        return self._get_num_beats(
            int(time),
            int(distance),
        )

    def part_one_bf(self, inp: str) -> int:
        """Brute force solution for part 1"""

        times, distances = inp.splitlines()
        _, times = times.split(':', maxsplit=1)
        _, distances = distances.split(':', maxsplit=1)

        times = map(int, times.split())
        distances = map(int, distances.split())

        return prod(
            self._get_num_beats_bf(time, to_beat)
            for time, to_beat in zip(times, distances)
        )

    def part_two_bf(self, inp: str) -> int:
        """Brute force solution for part 2"""

        time, distance = inp.replace(' ', '').splitlines()
        _, time = time.split(':', maxsplit=1)
        _, distance = distance.split(':', maxsplit=1)

        return self._get_num_beats_bf(
            int(time),
            int(distance),
        )

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 1731600
        assert p2 == 40087680