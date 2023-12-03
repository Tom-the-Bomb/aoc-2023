from sys import argv

from . import run_day, SOLUTIONS

def main(args) -> None:
    day = int(args[0]) if args else None
    if day:
        run_day(day)
    else:
        for day in range(1, len(SOLUTIONS) + 1):
            run_day(day)

if __name__ == '__main__':
    main(argv[1:])