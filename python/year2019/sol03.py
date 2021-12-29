from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass, field
from itertools import product
from typing import Iterable

import utils


@dataclass(frozen=True)
class Point:
    x: int
    y: int

    @property
    def distance(self) -> int:
        """Returns the manhattan distance from the origin."""
        return abs(self.x) + abs(self.y)


@dataclass(frozen=False)
class LineSegment:
    p1: Point
    p2: Point

    minx: int = field(init=False, repr=False)
    maxx: int = field(init=False, repr=False)
    miny: int = field(init=False, repr=False)
    maxy: int = field(init=False, repr=False)

    def __post_init__(self):
        self.minx = min(self.p1.x, self.p2.x)
        self.maxx = max(self.p1.x, self.p2.x)
        self.miny = min(self.p1.y, self.p2.y)
        self.maxy = max(self.p1.y, self.p2.y)

    @property
    def vertical(self) -> bool:
        """Return True if the line segment is vertical, false otherwise."""
        return self.p1.x == self.p2.x

    @property
    def length(self) -> int:
        """Return the length of the line segment."""
        return abs(self.p1.x - self.p2.x) + abs(self.p1.y - self.p2.y)

    def contains(self, point: Point) -> bool:
        """Return True if given point is on the line segment, False otherwise."""
        if self.vertical:
            return point.x == self.p1.x and self.miny <= point.y <= self.maxy
        return point.y == self.p1.y and self.minx <= point.x <= self.maxx

    def intersection(self, other: LineSegment) -> Point | None:
        """Return the intersecting point between self and other if it exists."""
        if self.vertical == other.vertical:
            return None
        elif (
            self.vertical
            and other.minx <= self.p1.x <= other.maxx
            and self.miny <= other.p1.y <= self.maxy
        ):
            return Point(self.p1.x, other.p1.y)
        elif (
            not self.vertical
            and self.minx <= other.p1.x <= self.maxx
            and other.miny <= self.p1.y <= other.maxy
        ):
            return Point(other.p1.x, self.p1.y)
        return None


def segments_from_path(path: Iterable[str]) -> Iterable[LineSegment]:
    segments = []
    start = Point(0, 0)
    for p in path:
        direction, magnitude = p[0], int(p[1:])
        if direction == "U":
            end = Point(start.x, start.y + magnitude)
        elif direction == "R":
            end = Point(start.x + magnitude, start.y)
        elif direction == "D":
            end = Point(start.x, start.y - magnitude)
        else:
            end = Point(start.x - magnitude, start.y)
        segments.append(LineSegment(start, end))
        start = end
    return segments


def minimum_step_count(
    segments_a: Iterable[LineSegment],
    segments_b: Iterable[LineSegment],
    intersection_points: Iterable[Point],
) -> int:
    step_count: dict[Point, int] = defaultdict(int)
    for p in intersection_points:
        found_in_a, found_in_b = False, False
        for a, b in zip(segments_a, segments_b):
            if not found_in_a:
                if a.contains(p):
                    step_count[p] += abs(p.x - a.p1.x) + abs(p.y - a.p1.y)
                    found_in_a = True
                else:
                    step_count[p] += a.length
            if not found_in_b:
                if b.contains(p):
                    step_count[p] += abs(p.x - b.p1.x) + abs(p.y - b.p1.y)
                    found_in_b = True
                else:
                    step_count[p] += b.length
            if found_in_a and found_in_b:
                break
    return step_count[min(step_count, key=lambda p: step_count[p])]


if __name__ == "__main__":
    data = utils.read(day=3, year=2019)
    segments_a, segments_b = (
        segments_from_path(line.split(",")) for line in data.splitlines()
    )

    intersection_points: list[Point] = []
    for a, b in product(segments_a, segments_b):
        if p := a.intersection(b):
            intersection_points.append(p)
    if (origin := Point(0, 0)) in intersection_points:
        intersection_points.remove(origin)
    closest_intersection_point = min(intersection_points, key=lambda p: p.distance)

    print(f"3.1: {closest_intersection_point.distance}")
    print(f"3.2: {minimum_step_count(segments_a, segments_b, intersection_points)}")
