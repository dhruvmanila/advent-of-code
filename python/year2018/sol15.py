import curses
import time
from collections.abc import Generator, MutableSequence
from dataclasses import dataclass, field
from enum import Enum
from itertools import count
from operator import attrgetter
from typing import Iterable, Sequence, TypeAlias

import utils

Position: TypeAlias = tuple[int, int]


class ElfDied(Exception):
    """Exception raised when an elf dies."""


def _adjacent(y: int, x: int) -> Generator[Position, None, None]:
    """Generate positions adjacent to the current position in reading order."""
    yield y - 1, x
    yield y, x - 1
    yield y, x + 1
    yield y + 1, x


class UnitType(str, Enum):
    ELF = "E"
    GOBLIN = "G"


@dataclass(order=True)
class Unit:
    y: int
    x: int
    type: UnitType = field(compare=False)
    attackpower: int = field(default=3, compare=False)
    hitpoints: int = field(default=200, compare=False, init=False)

    def adjacent(self) -> Generator[Position, None, None]:
        """Generate positions adjacent to the current position in reading order."""
        return _adjacent(self.y, self.x)

    @property
    def pos(self) -> Position:
        return self.y, self.x

    def __repr__(self) -> str:
        return f"{self.type.name}({self.y}, {self.x}, {self.hitpoints})"


_sentinel_pos: Position = (-1, -1)


@dataclass
class CaveCombat:
    map: Sequence[MutableSequence[str]]
    units: MutableSequence[Unit]
    round: int = 0

    @classmethod
    def from_lines(
        cls, lines: Iterable[str], elf_attack_power: int = 3
    ) -> "CaveCombat":
        map: list[list[str]] = []
        units: list[Unit] = []
        for y, line in enumerate(lines):
            for x, char in enumerate(line):
                match char:
                    case UnitType.ELF:
                        units.append(Unit(y, x, UnitType.ELF, elf_attack_power))
                    case UnitType.GOBLIN:
                        units.append(Unit(y, x, UnitType.GOBLIN))
            map.append(list(line))
        return cls(map, units)

    def do_battle(self, *, no_elf_dies: bool = False) -> int:
        while True:
            outcome = self.turn(no_elf_dies=no_elf_dies)
            if outcome is not None:
                return outcome

    def turn(self, *, no_elf_dies: bool = False) -> int | None:
        for unit in sorted(self.units):
            if unit.hitpoints <= 0:
                continue

            targets = [target for target in self.units if unit.type is not target.type]
            # If there are no targets, the combat ends.
            if not targets:
                return self.round * sum(unit.hitpoints for unit in self.units)

            in_range: set[Position] = set()
            for target in targets:
                for y, x in target.adjacent():
                    if self.map[y][x] == "." or (y, x) == unit.pos:
                        in_range.add((y, x))

            # No target(s) in range, let's move!
            if unit.pos not in in_range:
                # Mapping of position to the previous position in the shortest path.
                # This is used to reconstruct the path after the search.
                visited: dict[Position, Position] = {unit.pos: _sentinel_pos}
                remaining = [unit.pos]

                # This is a modified breadth-first search where instead of going
                # from left to right, we go from right to left. This is to ensure
                # that we always choose the first position in reading order.
                while remaining:
                    new_remaining: list[Position] = []
                    for current_pos in remaining:
                        for next_pos in _adjacent(*current_pos):
                            next_y, next_x = next_pos
                            if next_pos in visited or self.map[next_y][next_x] != ".":
                                continue
                            new_remaining.append(next_pos)
                            visited[next_pos] = current_pos
                    if visited.keys() & in_range:
                        # We found a path to an in-range target. This is the
                        # shortest path as made sure by the reading order.
                        break
                    remaining = new_remaining
                else:
                    # No path to an in-range target was found.
                    continue

                # Reconstruct the path to the first in-range target in reading order
                # and move the unit to the first position in the path.
                chosen_pos = min(visited.keys() & in_range)
                while visited[chosen_pos] != unit.pos:
                    chosen_pos = visited[chosen_pos]

                chosen_y, chosen_x = chosen_pos
                self.map[unit.y][unit.x] = "."

                # This is the current unit, so we can update it in place. As per the
                # instructions:
                #
                #   > The unit does this while considering the *current positions of
                #   > units* and does not do any prediction about where units will be
                #   > later.
                unit.y, unit.x = chosen_y, chosen_x
                self.map[chosen_y][chosen_x] = unit.type.value

            # Target(s) in range, attack!
            if unit.pos in in_range:
                target_pos = {target.pos: target for target in targets}
                target = min(
                    (target_pos[pos] for pos in unit.adjacent() if pos in target_pos),
                    key=attrgetter("hitpoints", "y", "x"),
                )
                target.hitpoints -= unit.attackpower
                if target.hitpoints <= 0:
                    if no_elf_dies and target.type is UnitType.ELF:
                        raise ElfDied()
                    self.map[target.y][target.x] = "."
                    self.units.remove(target)

        self.round += 1
        return None

    def render(self, round: bool = False, units: bool = False) -> str:
        """Render the map, optionally with the round number and/or units.

        This is the same as that in the puzzle description.
        """
        lines: list[str] = []
        if round:
            lines.append(f"After {self.round} round{'s' if self.round > 1 else ''}:")
        for y, line in enumerate(self.map):
            s = "".join(line)
            if units:
                s += "   "
                s += ", ".join(
                    f"{unit.type}({unit.hitpoints})"
                    for unit in self.units
                    if unit.y == y
                )
            lines.append(s)
        return "\n".join(lines)

    def summary(self) -> str:
        pass

    def __str__(self) -> str:
        return self.render()


def compute_elves_win(lines: Iterable[str]) -> int:
    """Compute the outcome of the battle when the elves win."""
    for elf_attack_power in count(4):
        try:
            combat = CaveCombat.from_lines(lines, elf_attack_power)
            return combat.do_battle(no_elf_dies=True)
        except ElfDied:
            pass


def render_full_combat(
    screen: curses.window, lines: Iterable[str], frame_rate: int
) -> int:
    curses.curs_set(0)
    combat = CaveCombat.from_lines(lines)
    while True:
        screen.addstr(0, 0, combat.render(round=True, units=True))
        screen.refresh()
        outcome = combat.turn()
        if outcome is not None:
            break
        time.sleep(1 / frame_rate)
    return outcome


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    parser.add_argument(
        "--render", action="store_true", help="render the combat in the terminal"
    )
    parser.add_argument(
        "--frame-rate",
        type=int,
        default=30,
        help="frame rate of the rendered combat (default: 30)",
    )
    args = parser.parse_args()

    lines = utils.read(day=15, year=2018, test=args.test).splitlines()
    if args.render:
        outcome = curses.wrapper(
            render_full_combat, lines=lines, frame_rate=args.frame_rate
        )
    else:
        cave = CaveCombat.from_lines(lines)
        outcome = cave.do_battle()

    print(f"15.1: {outcome}")
    print(f"15.2: {compute_elves_win(lines)}")
