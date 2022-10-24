import re
from collections.abc import Iterable, Sequence
from dataclasses import dataclass
from operator import attrgetter

import utils

Position = tuple[int, int, int]

_parse_line = re.compile(
    r"^pos=<(?P<x>-?\d+),(?P<y>-?\d+),(?P<z>-?\d+)>,\s*r=(?P<r>\d+)$"
).search


class ParserError(BaseException):
    """Exception raised when parsing fails."""


@dataclass(frozen=True, slots=True)
class Bot:
    x: int
    y: int
    z: int
    r: int

    @property
    def pos(self) -> Position:
        return self.x, self.y, self.z

    def distance_from(self, pos: Position) -> int:
        return abs(self.x - pos[0]) + abs(self.y - pos[1]) + abs(self.z - pos[2])

    def in_range(self, pos: Position) -> bool:
        return self.distance_from(pos) <= self.r


def parse_lines(lines: Iterable[str]) -> Sequence[Bot]:
    bots = []
    for idx, line in enumerate(lines):
        match = _parse_line(line)
        if match is None:
            raise ParserError(f"Failed to parse line {idx}: {line!r}")
        bots.append(
            Bot(
                x=int(match["x"]),
                y=int(match["y"]),
                z=int(match["z"]),
                r=int(match["r"]),
            )
        )
    return bots


def in_range_count(bots: Sequence[Bot]) -> int:
    strongest_bot = max(bots, key=attrgetter("r"))
    return sum(strongest_bot.in_range(bot.pos) for bot in bots)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    args = parser.parse_args()

    lines = utils.read(day=23, year=2018, test=args.test).splitlines()
    bots = parse_lines(lines)

    print(f"Part 1: {in_range_count(bots)}")
