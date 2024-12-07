import heapq
import itertools
import re
from dataclasses import dataclass
from enum import Enum, auto
from functools import cache, cached_property
from typing import Iterator, NamedTuple, TypeAlias

Position: TypeAlias = tuple[int, int]

_parse_input = re.compile(
    r"^depth: (?P<depth>\d+)\ntarget: (?P<x>\d+),(?P<y>\d+)$"
).search


def parse_input(data: str) -> tuple[int, Position]:
    match = _parse_input(data)
    assert match is not None, data
    return int(match["depth"]), (int(match["x"]), int(match["y"]))


REGION_ROCKY = 0
REGION_WET = 1
REGION_NARROW = 2


class Tool(Enum):
    CLIMBING_GEAR = auto()
    TORCH = auto()
    NEITHER = auto()


REGION_TO_TOOLS = {
    REGION_ROCKY: {Tool.CLIMBING_GEAR, Tool.TORCH},
    REGION_WET: {Tool.CLIMBING_GEAR, Tool.NEITHER},
    REGION_NARROW: {Tool.TORCH, Tool.NEITHER},
}


@dataclass(frozen=True, slots=True)
class Node:
    """Node on the A* search graph."""

    x: int = 0
    y: int = 0
    tool: Tool = Tool.TORCH
    time: int = 0

    @property
    def pos(self) -> Position:
        """Return the position of this node."""
        return self.x, self.y

    def cost(self, target: Position) -> int:
        """Return the cost of this node, f(n) = g(n) + h(n)

        The cost of this node is the time taken (g) plus estimated cost to
        get to nearest goal (h). Here, we use the manhattan distance to the
        target as the estimated cost.
        """
        return self.time + abs(target[0] - self.x) + abs(target[1] - self.y)


class QueueItem(NamedTuple):
    """Item in the priority queue.

    The priority queue is a min-heap, so the item with the lowest cost is
    popped first. To break ties, we use the count of the item, which is
    incremented each time an item is added to the queue.
    """

    cost: int
    itemcount: int
    node: Node


class Cave:
    def __init__(self, depth: int, target: Position) -> None:
        self.depth = depth
        self.target = target

    def _geologic_index(self, x: int, y: int) -> int:
        """Return the geologic index of the given position."""
        match (x, y):
            case (0, 0) | self.target:
                return 0
            case (_, 0):
                return x * 16807
            case (0, _):
                return y * 48271
            case _:
                return self._erosion_level(x - 1, y) * self._erosion_level(x, y - 1)

    @cache
    def _erosion_level(self, x: int, y: int) -> int:
        """Return the erosion level of the given position."""
        return (self._geologic_index(x, y) + self.depth) % 20183

    def region(self, x: int, y: int) -> int:
        """Return the region type of the given position."""
        return self._erosion_level(x, y) % 3

    @cached_property
    def risk_level(self) -> int:
        """Return the risk level of the cave."""
        total = 0
        for y in range(self.target[1] + 1):
            for x in range(self.target[0] + 1):
                total += self.region(x, y)
        return total

    def _next(self, node: Node) -> Iterator[Node]:
        # Let's change the tool first
        other_tool = next(
            tool
            for tool in REGION_TO_TOOLS[self.region(node.x, node.y)]
            if tool is not node.tool
        )
        yield Node(node.x, node.y, other_tool, node.time + 7)

        # Now, let's move to an adjacent node
        for dx, dy in ((0, 1), (1, 0), (0, -1), (-1, 0)):
            x, y = node.x + dx, node.y + dy
            if x < 0 or y < 0:
                continue
            if node.tool not in REGION_TO_TOOLS[self.region(x, y)]:
                continue
            yield Node(x, y, node.tool, node.time + 1)

    def search(self) -> int:
        """Search and rescue the target, returning the minimum time it took to reach
        the target.

        This uses the A* search algorithm.
        """
        start = Node()
        # Use a counter to break ties in the priority queue
        counter = itertools.count()
        pqueue = [QueueItem(start.cost(self.target), next(counter), start)]
        times = {(start.pos, start.tool): start.time}
        visited = set()

        while pqueue:
            node = heapq.heappop(pqueue).node
            if node.pos == self.target and node.tool is Tool.TORCH:
                return node.time
            for next_node in self._next(node):
                if next_node in visited:
                    continue
                if (
                    times.get((next_node.pos, next_node.tool), float("inf"))
                    <= next_node.time
                ):
                    continue
                times[next_node.pos, next_node.tool] = next_node.time
                visited.add(next_node)
                heapq.heappush(
                    pqueue,
                    QueueItem(next_node.cost(self.target), next(counter), next_node),
                )

        raise Exception("Unable to rescue the target")


def solve(input: str) -> None:
    depth, target = parse_input(input)
    cave = Cave(depth, target)

    print(f"Part 1: {cave.risk_level}")
    print(f"Part 2: {cave.search()}")
