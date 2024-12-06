from collections.abc import Mapping, Sequence

import utils

Position = tuple[int, int]


DIRECTION_DELTA: Mapping[str, Position] = {
    "U": (0, 1),
    "D": (0, -1),
    "L": (-1, 0),
    "R": (1, 0),
}


def signum(n: int) -> int:
    if n == 0:
        return 0
    return 1 if n > 0 else -1


def is_touching(head: Position, tail: Position) -> bool:
    for dx in range(-1, 2):
        for dy in range(-1, 2):
            if (tail[0] + dx, tail[1] + dy) == head:
                return True
    return False


def simulate_motions(motions: Sequence[str]) -> tuple[int, int]:
    knots = [(0, 0)] * 10
    visited1, visited2 = {knots[0]}, {knots[0]}

    for motion in motions:
        direction, steps = motion.split()
        dx, dy = DIRECTION_DELTA[direction]
        for _ in range(int(steps)):
            knots[0] = (knots[0][0] + dx, knots[0][1] + dy)
            for idx, (head, tail) in enumerate(zip(knots, knots[1:]), start=1):
                if is_touching(head, tail):
                    continue
                knots[idx] = (
                    tail[0] + signum(head[0] - tail[0]),
                    tail[1] + signum(head[1] - tail[1]),
                )
            visited1.add(knots[1])
            visited2.add(knots[-1])

    return len(visited1), len(visited2)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-s", "--sample", action="store_true", help="use the sample input"
    )
    args = parser.parse_args()

    motions = utils.get_puzzle_input(day=9, year=2022, sample=args.sample).splitlines()
    visited1, visited2 = simulate_motions(motions)

    print(f"9.1: {visited1}")
    print(f"9.2: {visited2}")
