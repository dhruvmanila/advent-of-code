from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable, TypeAlias

Symbol: TypeAlias = str


@dataclass
class Reaction:
    output: Symbol
    quantity: int
    inputs: dict[Symbol, int]

    @classmethod
    def from_line(cls, line: str) -> Reaction:
        inputs, output = map(str.strip, line.split("=>"))

        def symbol_quantity(s: str) -> tuple[Symbol, int]:
            q, sym = map(str.strip, s.split())
            return sym, int(q)

        return cls(
            *symbol_quantity(output),
            dict(symbol_quantity(s) for s in inputs.split(","))
        )


def parse_reactions(lines: Iterable[str]) -> dict[Symbol, Reaction]:
    return {reaction.output: reaction for reaction in map(Reaction.from_line, lines)}


def minimum_ore(reactions: dict[Symbol, Reaction], fuel: int = 1) -> int:
    pass


if __name__ == "__main__":
    import aocd

    lines = aocd.get_data(day=14, year=2019).splitlines()
    reactions = parse_reactions(lines)
    print(reactions["FUEL"])
