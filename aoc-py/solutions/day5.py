"""
Day 5: If You Give A Seed A Fertilizer

https://adventofcode.com/2023/day/5
"""
__all__ = ('Day5',)

from itertools import chain

from ..solution import Solution

class Day5(Solution):
    NAME = 'If You Give A Seed A Fertilizer'

    def _get_lookup_table(self, mapping: str) -> list[tuple[range, int]]:
        """Returns a list of pairs of:

        the source range
        and the offset between the source and destination values
        """
        table = []
        for line in mapping.splitlines()[1:]:
            dest_start, src_start, range_ = [int(x) for x in line.split()]
            
            table.append((
                range(src_start, src_start + range_),
                dest_start - src_start,
            ))
        return table

    def part_one(self, inp: str) -> int:
        curr_data, *maps = inp.split('\n\n')
        curr_data = [int(item) for item in curr_data.removeprefix('seeds: ').split()]
        for mapping in maps:
            table = self._get_lookup_table(mapping)

            for i, item in enumerate(curr_data):
                curr_data[i] = ([
                    item + diff for map_range, diff in table
                    if item in map_range
                ] or [item])[0]
                
        return min(curr_data)

    def part_two(self, inp: str) -> int:
        inputs, *maps = inp.split('\n\n')
        inputs = inputs.removeprefix('seeds: ').split()
        curr_data = [
            range(a_ := int(a), a_ + int(b))
            for a, b in zip(inputs[::2], inputs[1::2])
        ]

        for mapping in maps:
            table = self._get_lookup_table(mapping)

            temp = []
            while curr_data:
                seed_range = curr_data.pop()

                for map_range, diff in table:
                    intersection = range(
                        max(seed_range.start, map_range.start),
                        min(seed_range.stop, map_range.stop),
                    )
                    if intersection:
                        temp.append(range(intersection.start + diff, intersection.stop + diff))
                        if intersection.start > seed_range.start:
                            curr_data.append(range(map_range.start, intersection.start))
                        if seed_range.stop > intersection.stop:
                            curr_data.append(range(intersection.stop, seed_range.stop))
                        break
                else:
                    temp.append(seed_range)
            curr_data = temp
        return min(range_.start for range_ in curr_data)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 993500720
        assert p2 == 4917124