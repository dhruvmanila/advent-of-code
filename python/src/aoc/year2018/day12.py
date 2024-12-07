from __future__ import annotations

from collections.abc import Sequence
from dataclasses import dataclass
from typing import Mapping


@dataclass
class Garden:
    # A list of pot numbers which contain a plant.
    plants: Sequence[int]
    notes: Mapping[str, str]

    def __post_init__(self) -> None:
        self.first_pot = min(self.plants)

    @classmethod
    def from_lines(cls, lines: list[str]) -> Garden:
        plants = [
            n
            for n, pot in enumerate(lines[0].removeprefix("initial state: "))
            if pot == "#"
        ]
        notes = {line[:5]: line[-1] for line in lines[2:]}
        return cls(plants, notes)

    def step(self) -> bool:
        changed = True
        next_plants: list[int] = []
        for n in range(min(self.plants) - 3, max(self.plants) + 4):
            group = self[n - 2 : n + 3]
            if (pot := self.notes.get(group)) and pot != ".":
                next_plants.append(n)
        if all(prev - next_ == -1 for prev, next_ in zip(self.plants, next_plants)):
            changed = False
        self.first_pot = min(self.first_pot, min(next_plants))
        self.plants = next_plants
        return changed

    def score(self) -> int:
        return sum(self.plants)

    def __getitem__(self, index: int | slice) -> str:
        if isinstance(index, int):
            return "#" if index in self.plants else "."
        return "".join(
            "#" if n in self.plants else "."
            for n in range(index.start, index.stop, index.step or 1)
        )

    def __str__(self) -> str:
        return "".join(
            "#" if n in self.plants else "."
            for n in range(min(self.plants), max(self.plants) + 1)
        )


def solve(input: str) -> None:
    garden = Garden.from_lines(input.splitlines())
    for generations in range(1, 21):
        garden.step()
    print(f"12.1: {garden.score()}")

    while garden.step():
        generations += 1
    print(
        f"12.2: {garden.score() + (50_00_00_00_000 - generations) * len(garden.plants)}"
    )
