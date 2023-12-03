"""
Python 3 Advent of Code 2023 Solutions
"""
__all__ = (
    'SOLUTIONS',
    'get_input',
    'run_day',
)

from .solutions import SOLUTIONS

def get_input(day: int) -> str:
    with open(f'./inputs/day{day}.txt') as f:
        return f.read()

def run_day(day: int) -> None:
    try:
        solution = SOLUTIONS[day - 1]()
        text = f' Day [{day}] Solution - {solution.NAME} '
        line = '+-----+' + '-' * len(text) + '+'

        print(f'\n{line}\n| PY3 |{text}|\n{line}')
        solution.run(get_input(day))
    except IndexError:
        print(f'Solution does not exist yet for day {day}')