from enum import Enum
from typing import Iterable

import utils


class Direction(int, Enum):
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3


def destination_distance(instructions: Iterable[str]) -> tuple[int, int]:
    x, y = 0, 0
    direction = Direction.NORTH
    visited: set[tuple[int, int]] = {(x, y)}
    visited_twice = -1
    for instruction in instructions:
        direction = (direction + (1 if instruction[0] == "R" else -1)) % 4
        for _ in range(int(instruction[1:])):
            match direction:
                case Direction.NORTH:
                    y += 1
                case Direction.EAST:
                    x += 1
                case Direction.SOUTH:
                    y -= 1
                case Direction.WEST:
                    x -= 1
            if visited_twice == -1:
                if (x, y) in visited:
                    visited_twice = abs(x) + abs(y)
                else:
                    visited.add((x, y))
    return abs(x) + abs(y), visited_twice


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    args = parser.parse_args()

    data = utils.read(year=2016, day=1, test=args.test)
    blocks, visited_twice = destination_distance(data.split(", "))

    print(f"1.1: {blocks}")
    print(f"1.2: {visited_twice}")
