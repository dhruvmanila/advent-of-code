from __future__ import annotations

import re
from collections import defaultdict
from dataclasses import dataclass, field
from typing import Iterable

import utils

CLAIM_RE = re.compile(r"^#(\d+)\s*@\s*(\d+),(\d+):\s*(\d+)x(\d+)$")

Point = tuple[int, int]


@dataclass
class Rectangle:
    id: int
    left: int
    top: int
    width: int
    height: int

    right: int = field(init=False)
    bottom: int = field(init=False)

    def __post_init__(self):
        self.right = self.left + self.width
        self.bottom = self.top + self.height

    @classmethod
    def from_line(cls, line: str) -> Rectangle:
        match = CLAIM_RE.fullmatch(line)
        assert match is not None, line
        return cls(*map(int, match.groups()))

    def overlaps(self, other: Rectangle) -> bool:
        """Return True if self overlaps other, False otherwise."""
        return (
            self.left < other.right
            and self.right > other.left
            and self.top < other.bottom
            and self.bottom > other.top
        )


def sum_overlap(rectangles: Iterable[Rectangle]) -> int:
    grid: dict[Point, int] = defaultdict(int)
    for r in rectangles:
        for x in range(r.left, r.right):
            for y in range(r.top, r.bottom):
                grid[(x, y)] += 1
    total = 0
    for count in grid.values():
        if count > 1:
            total += 1
    return total


def no_overlap_id(rectangles: Iterable[Rectangle]) -> int:
    for r1 in rectangles:
        for r2 in rectangles:
            if r1 != r2 and r1.overlaps(r2):
                break
        else:
            return r1.id

    raise AssertionError("Unsolvable!")


if __name__ == "__main__":
    data = utils.read(day=3, year=2018)
    rectangles = list(map(Rectangle.from_line, data.splitlines()))

    print(f"3.1: {sum_overlap(rectangles)}")
    print(f"3.2: {no_overlap_id(rectangles)}")
