from __future__ import annotations

import sys
from collections import Counter
from typing import Iterable

Point = tuple[int, int]


def location_metadata(
    cx: int, cy: int, points: Iterable[Point]
) -> tuple[Point | None, int]:
    """Return a tuple containing metadata for the given point (cx, cy).

    Metadata includes the following two information:
        0. Nearest point from given `points` if it's not nearer to any other point.
        1. Total distance from given point (cx, cy) to all the `points`.
    """
    total_distance = 0
    min_distance = sys.maxsize
    nearest: Point | None = None

    for px, py in points:
        distance = abs(px - cx) + abs(py - cy)
        total_distance += distance
        if distance == min_distance:
            nearest = None
        elif distance < min_distance:
            min_distance = distance
            nearest = (px, py)

    return nearest, total_distance


def compute_area(
    minx: int,
    maxx: int,
    miny: int,
    maxy: int,
    coordinates: Iterable[Point],
    bound: int,
) -> tuple[int, int]:
    safe_area = 0
    area_points: Counter[Point] = Counter()

    # Step 1: For all the points within the grid formed by the min-max pairs, update
    # the count for the nearest coordinate from the given set of coordinates.
    for x in range(minx, maxx):
        for y in range(miny, maxy):
            nearest, total_distance = location_metadata(x, y, coordinates)
            if nearest:
                area_points[nearest] += 1
            if total_distance < bound:
                safe_area += 1

    # Step 2: Remove all the coordinates whose area contains points outside the bound
    # for the top and bottom row just outside the bound.
    for x in range(minx, maxx):
        for y in (miny - 1, maxy + 1):
            nearest, _ = location_metadata(x, y, coordinates)
            if nearest:
                area_points.pop(nearest, None)

    # Step 3: Remove all the coordinates whose area contains points outside the bound
    # for the left and right column just outside the bound.
    for y in range(miny, maxy):
        for x in (minx - 1, maxx + 1):
            nearest, _ = location_metadata(x, y, coordinates)
            if nearest:
                area_points.pop(nearest, None)

    _, dangerous_area = area_points.most_common(1)[0]
    return dangerous_area, safe_area


def parse_data(data: str) -> tuple[int, int, int, int, Iterable[Point]]:
    """Return the bounding box and set of points from the given data.

    Bounding box is in the order: minx, maxx, miny, maxy.
    """
    points: list[Point] = []
    minx, maxx = sys.maxsize, 0
    miny, maxy = sys.maxsize, 0

    for line in data.splitlines():
        x, y = map(int, line.split(","))
        minx, maxx = min(minx, x), max(maxx, x)
        miny, maxy = min(miny, y), max(maxy, y)
        points.append((x, y))

    return minx, maxx, miny, maxy, points


def solve(input: str) -> None:
    dangerous_area, safe_area = compute_area(*parse_data(input), 10000)

    print(f"6.1: {dangerous_area}")
    print(f"6.2: {safe_area}")
