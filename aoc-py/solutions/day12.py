"""
Day 12: Hot Springs

https://adventofcode.com/2023/day/12
"""
__all__ = ('Day12',)

from typing import ClassVar
from functools import cache
from itertools import product, groupby

from ..solution import Solution

class Day12(Solution):
    NAME: ClassVar[str] = 'Hot Springs'

    @cache
    def _get_arrangements(
        self,
        records: str,
        criteria: tuple[int, ...],
    ) -> int:
        """Recursive solution with memoization cache"""
        if not records:
            # if there are no records
            #   - and the criteria also states there should be 0 broken,
            #     then there is 1 possible arrangement
            #   - if there are no records but the criteria says there should be,
            #     there are NO possible arrangements
            return 0 if criteria else 1
        elif not criteria:
            # if the criteria requires 0 broken springs
            #   - but there are broken springs,
            #     there are NO possible arrangements
            #   - if there are no broken springs
            #     this fits the criteria, so there is 1 possible arrangment
            return 0 if '#' in records else 1

        count = 0
        first_record = records[0]
        first_criteria = criteria[0]

        # case 1: handle/treat unknown spring to be operational
        if first_record in {'.', '?'}:
            # if the first spring is a operational or unknown
            # we can disregard it (since we are just counting damaged springs (by the criteria))
            # simply slice it off and check the # of arrangements for the springs after that
            count += self._get_arrangements(records[1:], criteria)
        # case 2: handle/treat unknown spring to be damaged
        if (
            first_record in {'#', '?'}
            # ensures that there are even enough springs left in the records to satisfy the first criteria
            and len(records) >= first_criteria
            # ensures for a collection of damaged springs to count towards a single criteria entry:
            # they must all be consecutive, meaning there should be no (non-damaged / operational / '.') springs
            # in between / up to the next `first_criteria` amount of springs
            and '.' not in records[:first_criteria]
            # ensures that the immediate spring right after `first_critera` amount of damaged springs
            # is NOT damaged (this is what ends our block of damaged springs)
            # and ensures that our block of damaged springs is has exactly `first_critera` amount of damaged springs
            # (the problem states that all adjacent damaged springs just count as one block)
            #   - Ex: "#### is always 4 and never 2,2" hence for 2,2 to occur, right after ## there must be a '.' (or '?') (not '#')
            and (
                # either we are already at the end of the records
                first_criteria == len(records)
                # otherwise we check if right after `first_critera` amount of springs the next one is NOT damanged
                or records[first_criteria] != '#'
            )
        ):
            # if the above condition is true, we have got ourselves a block of damaged springs that satisfies the criteria
            # we can then contiue to check,
            count += self._get_arrangements(
                # without the block of damaged (that was just verified)
                records[first_criteria + 1:],
                # without the first critera (that was just satisfied)
                criteria[1:],
            )
        return count

    def part_one_bf(self, inp: str) -> int:
        """Brute force solution by trying all the possible combinations
        and checking them against the critera for each record
        """
        total = 0
        for record in inp.splitlines():
            record, counts = record.split()
            counts = tuple(map(int, counts.split(',')))

            n_unknown = record.count('?')
            record = record.replace('?', '{}')
            for combination in product(
                ('.', '#'),
                repeat=n_unknown,
            ):
                trial = record.format(*combination)
                total += tuple(len(tuple(b)) for a, b in groupby(trial) if a == '#') == counts
        return total

    def part_one(self, inp: str) -> int:
        return sum(
            self._get_arrangements(
                (parts := line.split())[0],
                tuple(map(int, parts[1].split(',')))
            )
            for line in inp.splitlines()
        )

    def part_two(self, inp: str) -> int:
        return sum(
            self._get_arrangements(
                '?'.join([(parts := line.split())[0]] * 5),
                tuple(map(int, parts[1].split(','))) * 5
            )
            for line in inp.splitlines()
        )

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 7007
        assert p2 == 3476169006222