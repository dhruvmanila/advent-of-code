import argparse
from datetime import date
from typing import Final

from aoc.solution import SOLUTIONS, SOLUTIONS_WITH_ARGS
from aoc.utils import get_puzzle_input

DECEMBER: Final[int] = 12
"""December month number."""

CHRISTMAS_DAY: Final[int] = 25
"""Christmas day number."""


def cli() -> int:
    """Command-line interface for the Advent of Code solutions in Python."""
    parser = argparse.ArgumentParser(
        prog="aoc",
        add_help=False,
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    add_default_args(parser)
    (args, additional_args) = parser.parse_known_args()
    add_additional_args(args, parser)
    args = parser.parse_args(additional_args, namespace=args)

    solution = SOLUTIONS.get((args.year, args.day))
    solution_with_args = SOLUTIONS_WITH_ARGS.get((args.year, args.day))

    input = get_puzzle_input(year=args.year, day=args.day, sample=args.sample)

    if solution_with_args is not None:
        solution_with_args(input, args)
    elif solution is not None:
        solution(input)
    else:
        parser.error(f"no solution found for year {args.year} and day {args.day}")

    return 0


def add_default_args(parser: argparse.ArgumentParser) -> None:
    """Add the default arguments to the command-line parser."""
    default_date = get_default_date()
    parser.add_argument(
        "-d",
        "--day",
        type=int,
        choices=range(1, 26),
        metavar="DAY",
        default=default_date.day,
        help="day of the puzzle (1-25)",
    )
    parser.add_argument(
        "-y",
        "--year",
        type=int,
        choices=range(2015, default_date.year + 1),
        metavar="YEAR",
        default=default_date.year,
        help=f"year of the puzzle (2015-{default_date.year})",
    )
    parser.add_argument(
        "-s", "--sample", action="store_true", help="use the sample input instead"
    )


def add_additional_args(
    known_args: argparse.Namespace, parser: argparse.ArgumentParser
) -> None:
    """Add additional arguments to the command-line parser based on the known arguments.

    This looks at the year and day of the puzzle to determine what additional arguments
    should be added to the parser.
    """
    from aoc.year2019.day13 import DEFAULT_FRAME_RATE

    group = parser.add_argument_group("additional options")

    match (known_args.year, known_args.day):
        case (2018, 7):
            group.add_argument("--render", action="store_true", help="render the graph")
        case (2018, 8):
            group.add_argument(
                "-p", "--print", action="store_true", help="pretty print the root node"
            )
        case (2018, 15):
            group.add_argument(
                "--render",
                action="store_true",
                help="render the combat in the terminal",
            )
            group.add_argument(
                "--frame-rate",
                type=int,
                default=30,
                help="frame rate of the rendered combat (default: %(default)s)",
            )
        case (2018, 17):
            group.add_argument(
                "-i",
                "--image",
                action="store_true",
                help="generate an image (reservoir.png)",
            )
            group.add_argument("--scale", type=int, default=1, help="image scale")
        case (2018, 19):
            group.add_argument(
                "--fast", action="store_true", help="skip executing the instructions"
            )
        case (2019, 13):
            group.add_argument("--render", action="store_true", help="render the game")
            group.add_argument(
                "--frame-rate",
                type=int,
                default=DEFAULT_FRAME_RATE,
                help="frame rate of the rendered game (default: %(default)s)",
            )

    parser.add_argument(
        "-h", "--help", action="help", help="show this help message and exit"
    )


def get_default_date() -> date:
    """Return the default date for the puzzle as per the current date."""
    today = date.today()
    if today.month != DECEMBER:
        return today.replace(year=today.year - 1, month=DECEMBER, day=CHRISTMAS_DAY)
    elif today.day > CHRISTMAS_DAY:
        return today.replace(day=CHRISTMAS_DAY)
    return today
