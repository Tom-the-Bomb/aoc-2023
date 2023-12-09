
__all__ = ('Solution',)

from typing import ClassVar
from abc import ABC, abstractmethod

class Solution(ABC):
    """Contains the solution(s) for a single day"""
    NAME: ClassVar[str]

    @abstractmethod
    def part_one(self, inp: str) -> int:
        """Part 1 Solution"""

    @abstractmethod
    def part_two(self, inp: str) -> int:
        """Part 2 Solution"""

    @abstractmethod
    def run(self, inp: str) -> None:
        """Executes that day's code"""