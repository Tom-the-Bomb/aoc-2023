"""
Day 7: Camel Cards

https://adventofcode.com/2023/day/7
"""
__all__ = ('Day7',)

from typing import ClassVar, Optional

from ..solution import Solution

class Day7(Solution):
    NAME: ClassVar[str] = 'Camel Cards'
    CARDS: ClassVar[list[str]] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']

    def _get_hand_strength(self, hand: str, jokers: Optional[set[int]] = None) -> tuple[int, ...]:
        jokers = jokers or set()
        counter = [hand.count(card) for card in set(hand)]

        match (max(counter), len(counter)):
            case 5, _: # 5 of a kind
                points = 6
            case 4, _: # 4 of a kind
                points = 5
            case 3, 2: # 3 full house
                points = 4
            case 3, 3: # 3 of a kind
                points = 3
            case 2, 3: # 2 pair
                points = 2
            case 2, 4: # one pair
                points = 1
            case _:    # high card
                points = 0
        return (
            points,
            *map(lambda card: -1 if card[0] in jokers else self.CARDS.index(card[1]), enumerate(hand)),
        )

    def _get_hand_strength_joker(self, hand: str) -> tuple[int, ...]:
        if hand == 'JJJJJ':
            return self._get_hand_strength('AAAAA', jokers=set(range(5)))

        return self._get_hand_strength(
            hand.replace(
                'J',
                max(
                    filter(lambda card: card != 'J', hand),
                    key=hand.count,
                ),
            ),
            jokers={i for i, card in enumerate(hand) if card == 'J'},
        )

    def part_one(self, inp: str) -> int:
        hands = sorted(
            [
                ((hand_ := hand.split())[0], int(hand_[1]))
                for hand in inp.splitlines()
            ],
            key=lambda hand: self._get_hand_strength(hand[0])
        )
        return sum(i * bid for i, (_, bid) in enumerate(hands, 1))

    def part_two(self, inp: str) -> int:
        hands = sorted(
            [
                ((hand_ := hand.split())[0], int(hand_[1]))
                for hand in inp.splitlines()
            ],
            key=lambda hand: self._get_hand_strength_joker(hand[0])
        )
        return sum(i * bid for i, (_, bid) in enumerate(hands, 1))

    def run(self, inp: str) -> None:
        print('Part 1:', p1 := self.part_one(inp))
        print('Part 2:', p2 := self.part_two(inp))

        assert p1 == 253313241
        assert p2 == 253362743