from typing import ClassVar
from abc import ABC, abstractmethod

class Solution(ABC):
    """Contains the solution(s) for a single day"""
    NAME: ClassVar[str]

    @abstractmethod
    def run(self, inp: str) -> None:
        """Executes that day's code"""