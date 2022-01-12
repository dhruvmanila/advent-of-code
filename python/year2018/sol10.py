from __future__ import annotations

import re
from dataclasses import dataclass
from typing import Iterable, Sequence

import utils

VECTOR_RE = re.compile(
    r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>"
)


@dataclass(frozen=True)
class Vector:
    x: int
    y: int
    dx: int
    dy: int

    def after(self, t: int) -> Vector:
        """Return the Vector formed after moving for t seconds."""
        return Vector(self.x + self.dx * t, self.y + self.dy * t, self.dx, self.dy)


@dataclass(frozen=True)
class Bounds:
    minx: int
    maxx: int
    miny: int
    maxy: int

    def area(self) -> int:
        """Return the area of the bounds."""
        return (self.maxx - self.minx) * (self.maxy - self.miny)


@dataclass(frozen=True)
class Points:
    points: Sequence[Vector]

    def step(self) -> Points:
        return Points([p.after(1) for p in self.points])

    def bounds(self) -> Bounds:
        minx = maxx = self.points[0].x
        miny = maxy = self.points[0].y
        for p in self.points:
            minx = min(minx, p.x)
            maxx = max(maxx, p.x)
            miny = min(miny, p.y)
            maxy = max(maxy, p.y)
        return Bounds(minx, maxx, miny, maxy)

    def __str__(self) -> str:
        result = ""
        points = {(p.y, p.x): "█" for p in self.points}
        b = self.bounds()
        for y in range(b.miny, b.maxy + 1):
            for x in range(b.minx, b.maxx + 1):
                result += points.get((y, x), " ")
            result += "\n"
        return result


def parse_vectors(data: str) -> list[Vector]:
    vectors: list[Vector] = []
    for x, y, dx, dy in VECTOR_RE.findall(data):
        vectors.append(Vector(int(x), int(y), int(dx), int(dy)))
    return vectors


def extract_message_slow(points: Points) -> tuple[Points, int]:
    seconds = 0
    prev_area = points.bounds().area()

    while True:
        new_points = points.step()
        new_area = new_points.bounds().area()
        if new_area > prev_area:
            break
        else:
            points = new_points
            prev_area = new_area
            seconds += 1

    return points, seconds


def extract_message(points: Iterable[Vector], height: int) -> tuple[Points, int]:
    mindy = min(filter(lambda p: p.dy > 0, points), key=lambda p: p.dy).dy
    maxdy = max(filter(lambda p: p.dy < 0, points), key=lambda p: p.dy).dy

    minv = min(filter(lambda p: p.dy == mindy, points), key=lambda p: p.y)
    maxv = max(filter(lambda p: p.dy == maxdy, points), key=lambda p: p.y)

    t = 0
    while abs(maxv.y - minv.y) > height - 1:
        minv = minv.after(1)
        maxv = maxv.after(1)
        t += 1

    return Points([p.after(t) for p in points]), t


if __name__ == "__main__":
    data = utils.read(day=10, year=2018, test=False)
    vectors = parse_vectors(data)
    message, time = extract_message(vectors, 10)

    print(f"10.1:\n{message}", end="")
    print(f"10.2: {time}")
