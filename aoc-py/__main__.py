from sys import argv

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

def main(args) -> None:
    day = int(args[0]) if args else None
    if day:
        run_day(day)
    else:
        for day in range(1, len(SOLUTIONS) + 1):
            run_day(day)

if __name__ == '__main__':
    main(argv[1:])