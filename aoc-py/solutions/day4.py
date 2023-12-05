"""
Day 4: Scratchcards

https://adventofcode.com/2023/day/4
"""
__all__ = ('Day4',)

from ..solution import Solution

class Day4(Solution):
    NAME = 'Scratchcards'

    def _get_winning_amt(self, card: str) -> int:
        _, nums = card.split(':', maxsplit=1)
        winning, mine = nums.split('|', maxsplit=1)
        winning, mine = set(winning.split()), set(mine.split())

        return len(winning.intersection(mine))

    def part_one(self, inp: str) -> int:
        total = 0
        for card in inp.splitlines():
            amt_win = self._get_winning_amt(card)
            total += 2 ** (amt_win - 1) if amt_win > 0 else 0
        return total

    def part_two(self, inp: str) -> int:
        copies = [1] * len(lines := inp.splitlines())

        for i, (card, n_copies) in enumerate(zip(lines, copies)):
            win_amt = self._get_winning_amt(card)

            for card in range(i + 1, win_amt + i + 1):
                try:
                    copies[card] += n_copies
                except IndexError:
                    pass
        return sum(copies)

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 26914
        assert p2 == 13080971