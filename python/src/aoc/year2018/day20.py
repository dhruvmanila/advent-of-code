from collections import defaultdict
from collections.abc import Mapping, Set

Position = tuple[int, int]


DIRECTION_DELTA: Mapping[str, Position] = {
    "N": (0, 1),
    "S": (0, -1),
    "E": (1, 0),
    "W": (-1, 0),
}


class ProjectMap:
    def __init__(self, graph: Mapping[Position, Set[Position]]) -> None:
        self.graph = graph

    @classmethod
    def from_regex(cls, regex: str) -> "ProjectMap":
        if regex[0] != "^":
            raise ValueError(f"Invalid start token: {regex[0]!r} (expected '^')")
        elif regex[-1] != "$":
            raise ValueError(f"Invalid end token: {regex[-1]!r} (expected '$')")

        current = (0, 0)
        graph: dict[Position, set[Position]] = defaultdict(set, {current: set()})
        stack: list[Position] = []

        # Skip the anchor tokens i.e., '^' and '$'
        for token in regex[1:-1]:
            match token:
                case "(":
                    stack.append(current)
                case "|":
                    current = stack[-1]
                case ")":
                    stack.pop()
                case "N" | "S" | "E" | "W":
                    dx, dy = DIRECTION_DELTA[token]
                    nextpos = (current[0] + dx, current[1] + dy)
                    # This is a undirected graph, so create the edge from both positions.
                    graph[current].add(nextpos)
                    graph[nextpos].add(current)
                    current = nextpos
                case _:
                    raise ValueError(f"Invalid token: {token!r}")

        return cls(graph)

    def furthest_room(self) -> tuple[int, int]:
        """Return the furthest room and the number of rooms at least 1000 doors away."""
        start = (0, 0)
        queue = [start]
        distances: dict[Position, int] = {start: 0}

        while queue:
            current = queue.pop()
            distance = distances[current] + 1
            for next_node in self.graph[current]:
                if next_node in distances:
                    continue
                queue.append(next_node)
                distances[next_node] = distance

        return max(distances.values()), sum(1 for d in distances.values() if d >= 1000)

    def __str__(self) -> str:
        minx, miny = min(self.graph)
        maxx, maxy = max(self.graph)
        width, height = abs(maxx - minx) + 1, abs(maxy - miny) + 1

        lines = [["#"] * (width * 2 + 1) for _ in range(height * 2 + 1)]
        for (x, y), neighbors in self.graph.items():
            lx = (x - minx) * 2 + 1
            ly = (maxy - y) * 2 + 1
            lines[ly][lx] = "X" if (x, y) == (0, 0) else "."
            for nx, ny in neighbors:
                door = "-" if nx == x else "|"
                lines[ly + (y - ny)][lx + (nx - x)] = door

        return "\n".join("".join(line) for line in lines)


def solve(input: str) -> None:
    project_map = ProjectMap.from_regex(input)
    furthest_room, far_rooms = project_map.furthest_room()

    print(f"Part 1: {furthest_room}")
    print(f"Part 2: {far_rooms}")
