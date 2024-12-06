import heapq
import itertools
from collections.abc import Iterable, Sequence
from dataclasses import dataclass
from typing import Iterator, NamedTuple

import utils

Position = tuple[int, int]


class NoPathFound(Exception): ...


@dataclass(frozen=True, slots=True)
class Node:
    x: int
    y: int
    height: int
    dist: int = 0

    @property
    def pos(self) -> Position:
        return self.x, self.y

    def cost(self, target: "Node") -> int:
        return self.dist + abs(self.x - target.x) + abs(self.y - target.y)


class QueueItem(NamedTuple):
    cost: int
    itemcount: int
    node: Node


def multisource_astar(
    heightmap: Sequence[Sequence[int]], sources: Iterable[Node], target: Node
) -> int:
    count = itertools.count()
    pqueue: list[QueueItem] = []
    distance: dict[Position, int] = {}

    for source in sources:
        pqueue.append(QueueItem(source.cost(target), next(count), source))
        distance[source.pos] = 0

    def next_nodes(node: Node) -> Iterator[Node]:
        for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            x, y = node.x + dx, node.y + dy
            # Check for bounds
            if 0 <= y < len(heightmap) and 0 <= x < len(heightmap[0]):
                # Check for height
                if heightmap[y][x] - node.height <= 1:
                    yield Node(x, y, heightmap[y][x], node.dist + 1)

    while pqueue:
        _, _, node = heapq.heappop(pqueue)
        if node.pos == target.pos:
            break
        for next_node in next_nodes(node):
            if distance.get(next_node.pos, float("inf")) <= node.dist + 1:
                continue
            distance[next_node.pos] = node.dist + 1
            heapq.heappush(
                pqueue,
                QueueItem(next_node.cost(target), next(count), next_node),
            )

    if target.pos not in distance:
        raise NoPathFound

    return distance[target.pos]


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-s", "--sample", action="store_true", help="use the sample input"
    )
    args = parser.parse_args()

    lines = utils.get_puzzle_input(day=12, year=2022, sample=args.sample).splitlines()

    source: Node | None = None
    target: Node | None = None

    heightmap: list[list[int]] = []
    sources: list[Node] = []
    for y, line in enumerate(lines):
        heights: list[int] = []
        for x, char in enumerate(line):
            if char == "S":
                source = Node(x, y, 0)
                char = "a"
            elif char == "E":
                target = Node(x, y, ord("z") - ord("a"))
                char = "z"
            if char == "a":
                sources.append(Node(x, y, 0))
            heights.append(ord(char) - ord("a"))
        heightmap.append(heights)

    if source is None or target is None:
        raise ValueError("Unable to find source or target")

    print("Part 1:", multisource_astar(heightmap, [source], target))
    print("Part 2:", multisource_astar(heightmap, sources, target))
