from collections import defaultdict
from collections.abc import Iterable
from dataclasses import dataclass
from itertools import combinations


@dataclass(frozen=True, slots=True)
class Point:
    x: int
    y: int
    z: int
    t: int

    def is_near(self, other: "Point") -> bool:
        return (
            abs(self.x - other.x)
            + abs(self.y - other.y)
            + abs(self.z - other.z)
            + abs(self.t - other.t)
        ) <= 3

    @classmethod
    def from_line(cls, line: str) -> "Point":
        return cls(*[int(n) for n in line.split(",")])


def count_constallations(points: Iterable[Point]) -> int:
    graph: dict[Point, set[Point]] = defaultdict(set)
    for p1, p2 in combinations(points, 2):
        if p1.is_near(p2):
            graph[p1].add(p2)
            graph[p2].add(p1)

    constellations = 0
    # Initial nodes are all points as a single point is a constellation by itself.
    nodes = set(points)
    while nodes:
        constellations += 1
        queue = [nodes.pop()]
        while queue:
            node = queue.pop()
            for next_node in graph[node]:
                if next_node not in nodes:
                    continue
                nodes.remove(next_node)
                queue.append(next_node)
    return constellations


def solve(input: str) -> None:
    points = [Point.from_line(line) for line in input.splitlines()]

    print(f"1.1: {count_constallations(points)}")
