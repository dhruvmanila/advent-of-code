import re
from collections.abc import Iterable, Sequence
from dataclasses import dataclass
from operator import attrgetter
from typing import Any

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


def solve_using_solver(bots: Sequence[Bot]) -> int:
    from z3 import If, Int, Optimize, Sum

    def zabs(expr: Any) -> If:
        """Create a z3 absolute value expression."""
        return If(expr >= 0, expr, -expr)

    x, y, z = Int("x"), Int("y"), Int("z")
    distance_from_zero = Int("distance_from_zero")
    in_range_count = Int("in_range_count")

    o = Optimize()
    in_range_vars = []
    for i, bot in enumerate(bots):
        in_range = Int(f"in_range_{i}")
        o.add(
            in_range
            == If(
                zabs(x - bot.x) + zabs(y - bot.y) + zabs(z - bot.z) <= bot.r,
                1,
                0,
            )
        )
        in_range_vars.append(in_range)

    o.add(in_range_count == Sum(in_range_vars))
    o.add(distance_from_zero == zabs(x) + zabs(y) + zabs(z))

    o.maximize(in_range_count)
    o.minimize(distance_from_zero)

    o.check()
    model = o.model()

    return model[distance_from_zero].as_long()


def solve(input: str) -> None:
    bots = parse_lines(input.splitlines())

    print(f"Part 1: {in_range_count(bots)}")
    print(f"Part 2: {solve_using_solver(bots)}")
