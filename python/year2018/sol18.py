from __future__ import annotations

import itertools
from collections import Counter
from enum import Enum
from typing import Mapping, Sequence

import utils


class Acre(str, Enum):
    OPEN = "."
    TREE = "|"
    LUMBERYARD = "#"


class Forest:
    def __init__(
        self, rows: int, cols: int, grid: Mapping[tuple[int, int], Acre]
    ) -> None:
        self.rows = rows
        self.cols = cols
        self.grid = grid

    @classmethod
    def from_map(cls, forestmap: str) -> Forest:
        grid = {}
        lines = forestmap.splitlines()
        rows, cols = len(lines), len(lines[0])
        for y, line in enumerate(lines):
            for x, c in enumerate(line):
                grid[(y, x)] = Acre(c)
        return cls(rows, cols, grid)

    @property
    def resource_value(self) -> int:
        count = Counter(self.grid.values())
        return count[Acre.TREE] * count[Acre.LUMBERYARD]

    def neighbors(self, y: int, x: int) -> Sequence[Acre]:
        neighbors = []
        for dy in range(-1, 2):
            for dx in range(-1, 2):
                if dy == dx == 0:
                    continue
                if acre := self.grid.get((y + dy, x + dx)):
                    neighbors.append(acre)
        return neighbors

    def run(self):
        new_grid = {}
        for (y, x), acre in self.grid.items():
            count = Counter(self.neighbors(y, x))
            match acre:
                case Acre.OPEN if count[Acre.TREE] >= 3:
                    new_grid[(y, x)] = Acre.TREE
                case Acre.TREE if count[Acre.LUMBERYARD] >= 3:
                    new_grid[(y, x)] = Acre.LUMBERYARD
                case Acre.LUMBERYARD if count[Acre.LUMBERYARD] == 0 or count[
                    Acre.TREE
                ] == 0:
                    new_grid[(y, x)] = Acre.OPEN
                case _:
                    new_grid[(y, x)] = acre
        self.grid = new_grid

    def __str__(self) -> str:
        s = []
        for y in range(self.rows):
            row = []
            for x in range(self.cols):
                row.append(self.grid[(y, x)])
            s.append("".join(row))
        return "\n".join(s)


if __name__ == "__main__":
    data = utils.read(day=18, year=2018, test=False)
    forest = Forest.from_map(data)

    prev = 0
    seen = {}
    for minute in itertools.count(1):
        forest.run()
        value = forest.resource_value
        if minute == 10:
            print(f"18.1: {value}")
        cycle = minute - seen.get(value, 0)
        if cycle == prev:
            if 1_000_000_000 % cycle == minute % cycle:
                print(f"18.2: {value}")
                break
        seen[value] = minute
        prev = cycle
