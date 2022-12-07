import argparse
import datetime
from pathlib import Path

AOC_PYTHON_DIR = Path(__file__).parent

SOLUTION_TEMPLATE = """\
import utils

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    args = parser.parse_args()

    utils.read(day={day}, year={year}, test=args.test)
"""


def default_day_year() -> tuple[int, int]:
    today = datetime.date.today()
    day, year = today.day, today.year
    if today.month != 12:
        year -= 1
    if day > 25:
        day = 25
    return day, year


def main() -> int:
    day, year = default_day_year()
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-d",
        "--day",
        type=int,
        choices=range(1, 26),
        default=day,
        metavar="{1..25}",
    )
    parser.add_argument(
        "-y",
        "--year",
        type=int,
        choices=range(2015, year),
        default=year,
        metavar="{2015..%d}" % year,
    )
    args = parser.parse_args()

    year_dir = AOC_PYTHON_DIR / f"year{args.year}"
    if not year_dir.exists():
        print(f"mkdir: {year_dir}")
        year_dir.mkdir()

        init_file = year_dir / "__init__.py"
        print(f"touch: {init_file}")
        init_file.touch()

    solution_file = year_dir / f"sol{args.day:02}.py"
    if solution_file.exists():
        print(f"Solution file for year {args.year} and day {args.day} exists.")
    else:
        print(f"touch: {solution_file}")
        solution_file.touch()
        solution_file.write_text(SOLUTION_TEMPLATE.format(day=args.day, year=args.year))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
