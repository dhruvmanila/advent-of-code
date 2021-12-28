from __future__ import annotations

import math
from collections import defaultdict
from dataclasses import dataclass
from typing import Iterable, TypeAlias

import utils

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
            qty, sym = map(str.strip, s.split())
            return sym, int(qty)

        return cls(
            *symbol_quantity(output),
            dict(symbol_quantity(s) for s in inputs.split(",")),
        )


def parse_reactions(lines: Iterable[str]) -> dict[Symbol, Reaction]:
    return {reaction.output: reaction for reaction in map(Reaction.from_line, lines)}


def minimum_ore(reactions: dict[Symbol, Reaction], fuel: int = 1) -> int:
    need = defaultdict(int)
    need["FUEL"] = fuel

    def next_symbol() -> Symbol | None:
        for symbol, needed in need.items():
            if symbol != "ORE" and needed > 0:
                return symbol
        return None

    while symbol := next_symbol():
        reaction, needed = reactions[symbol], need[symbol]
        multiplier = math.ceil(needed / reaction.quantity)
        for input, quantity in reaction.inputs.items():
            need[input] += quantity * multiplier
        need[symbol] -= reaction.quantity * multiplier

    return need["ORE"]


ORE = 10 ** 12


def maximum_fuel(reactions: dict[Symbol, Reaction]) -> int:
    fuel = 1
    while (needed := minimum_ore(reactions, fuel)) <= ORE:
        fuel = (fuel * ORE // needed) + 1
    return fuel - 1


if __name__ == "__main__":
    data = utils.read(day=14, year=2019)
    reactions = parse_reactions(data.splitlines())

    print(f"Part 1: {minimum_ore(reactions)}")
    print(f"Part 2: {maximum_fuel(reactions)}")
