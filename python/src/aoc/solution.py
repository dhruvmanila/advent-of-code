import argparse
from collections.abc import Callable, Mapping

from aoc import year2016, year2018, year2019, year2022

type SolutionFn = Callable[[str], None]
"""Signature for the solution functions.

They accept a string as input and do not return anything.
"""

type SolutionFnWithArgs = Callable[[str, argparse.Namespace], None]
"""Signature for the solution functions that accept additional command-line arguments.

They accept a string as input and a namespace of additional arguments, and do not
return anything.
"""

SOLUTIONS: Mapping[tuple[int, int], SolutionFn] = {
    # 2016
    (2016, 1): year2016.day01.solve,
    # 2018
    (2018, 1): year2018.day01.solve,
    (2018, 2): year2018.day02.solve,
    (2018, 3): year2018.day03.solve,
    (2018, 4): year2018.day04.solve,
    (2018, 5): year2018.day05.solve,
    (2018, 6): year2018.day06.solve,
    (2018, 9): year2018.day09.solve,
    (2018, 10): year2018.day10.solve,
    (2018, 11): year2018.day11.solve,
    (2018, 12): year2018.day12.solve,
    (2018, 13): year2018.day13.solve,
    (2018, 14): year2018.day14.solve,
    (2018, 16): year2018.day16.solve,
    (2018, 18): year2018.day18.solve,
    (2018, 20): year2018.day20.solve,
    (2018, 21): year2018.day21.solve,
    (2018, 22): year2018.day22.solve,
    (2018, 23): year2018.day23.solve,
    (2018, 24): year2018.day24.solve,
    (2018, 25): year2018.day25.solve,
    # 2019
    (2019, 1): year2019.day01.solve,
    (2019, 2): year2019.day02.solve,
    (2019, 3): year2019.day03.solve,
    (2019, 4): year2019.day04.solve,
    (2019, 5): year2019.day05.solve,
    (2019, 6): year2019.day06.solve,
    (2019, 7): year2019.day07.solve,
    (2019, 8): year2019.day08.solve,
    (2019, 9): year2019.day09.solve,
    (2019, 10): year2019.day10.solve,
    (2019, 11): year2019.day11.solve,
    (2019, 12): year2019.day12.solve,
    (2019, 14): year2019.day14.solve,
    # 2022
    (2022, 1): year2022.day01.solve,
    (2022, 9): year2022.day09.solve,
    (2022, 10): year2022.day10.solve,
    (2022, 11): year2022.day11.solve,
    (2022, 12): year2022.day12.solve,
    (2022, 13): year2022.day13.solve,
}
"""Mapping of (year, day) to the corresponding solution function."""

SOLUTIONS_WITH_ARGS: Mapping[tuple[int, int], SolutionFnWithArgs] = {
    # 2018
    (2018, 7): year2018.day07.solve,
    (2018, 8): year2018.day08.solve,
    (2018, 15): year2018.day15.solve,
    (2018, 17): year2018.day17.solve,
    (2018, 19): year2018.day19.solve,
    # 2019
    (2019, 13): year2019.day13.solve,
}
"""Mapping of (year, day) to the corresponding solution function that accepts
additional command-line arguments."""
