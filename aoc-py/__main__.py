from sys import argv
from time import perf_counter

from . import run_day, SOLUTIONS

def main(args) -> None:
    day = int(args[0]) if args else None
    if day:
        run_day(day)
    else:
        start = perf_counter()
        for day in range(1, len(SOLUTIONS) + 1):
            run_day(day)
        end = perf_counter()

        print(line := f'\n[Total Execution time: {(end - start) * 1000:,.2f}ms]')
        print('=' * len(line))

if __name__ == '__main__':
    main(argv[1:])