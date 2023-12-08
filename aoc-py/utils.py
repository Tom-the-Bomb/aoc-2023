
from time import perf_counter

from .solutions import SOLUTIONS

def get_input(day: int) -> str:
    with open(f'./inputs/day{day}.txt') as f:
        return f.read()

def run_day(day: int) -> None:
    try:
        solution = SOLUTIONS[day - 1]()
    except IndexError:
        print(f'Solution does not exist yet for day {day}')
    else:
        text = f' Day [{day}] Solution - {solution.NAME} '
        line = '+-----+' + '-' * len(text) + '+'
        print(f'\n{line}\n| PY3 |{text}|\n{line}')

        start = perf_counter()
        solution.run(get_input(day))
        end = perf_counter()

        print(line := f'Execution time: {(end - start) * 1000:.2f}ms')
        print('=' * len(line))