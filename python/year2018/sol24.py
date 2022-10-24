import copy
import itertools
import re
from collections.abc import Iterable, Sequence
from dataclasses import dataclass, field, replace
from enum import Enum
from operator import attrgetter
from typing import Optional

import utils

_parse_group = re.compile(
    r"^(?P<units>\d+) units each with "
    r"(?P<hitpoints>\d+) hit points "
    r"(?:\((?P<attributes>[^)]+)\) )?"
    r"with an attack that does (?P<attackdamage>\d+) (?P<attacktype>\w+) damage "
    r"at initiative (?P<initiative>\d+)$"
).search


class ParserError(BaseException):
    """Exception raised when parsing fails."""


class StalemateError(BaseException):
    """Exception raised when a stalemate is detected."""


class ArmyType(Enum):
    IMMUNE_SYSTEM = "Immune System"
    INFECTION = "Infection"


@dataclass(frozen=False, unsafe_hash=True)
class Group:
    units: int = field(hash=False)
    hitpoints: int
    attackdamage: int
    attacktype: str
    initiative: int
    weaknesses: frozenset[str]
    immunities: frozenset[str]

    @classmethod
    def from_line(cls, line: str) -> "Group":
        """Parse the given line and return a new Group.

        Raises:
            ParserError: If the line cannot be parsed.
        """
        match = _parse_group(line)
        if match is None:
            raise ParserError(f"Failed to parse line: {line!r}")

        attributes: dict[str, frozenset[str]] = {
            "weak": frozenset(),
            "immune": frozenset(),
        }
        if match["attributes"]:
            for attr in match["attributes"].split("; "):
                attrtype, attrvalues = attr.split(" to ")
                assert attrtype in attributes, f"Unknown attribute type: {attrtype!r}"
                attributes[attrtype] = frozenset(attrvalues.split(", "))

        return cls(
            units=int(match["units"]),
            hitpoints=int(match["hitpoints"]),
            attackdamage=int(match["attackdamage"]),
            attacktype=match["attacktype"],
            initiative=int(match["initiative"]),
            weaknesses=attributes["weak"],
            immunities=attributes["immune"],
        )

    @property
    def effective_power(self) -> int:
        """Return the effective power of this group."""
        return self.units * self.attackdamage

    def damage_to(self, defender: "Group") -> int:
        """Return the damage this group would deal to the given defender."""
        if self.attacktype in defender.immunities:
            return 0
        elif self.attacktype in defender.weaknesses:
            return self.effective_power * 2
        else:
            return self.effective_power

    def attack(self, defender: "Group") -> None:
        """Attack the given defender."""
        damage = self.damage_to(defender)
        defender.units = max(defender.units - damage // defender.hitpoints, 0)

    def select_target(self, targets: Iterable["Group"]) -> Optional["Group"]:
        """Select a target from the given targets.

        Returns:
            The target group, or None if no target is selected.
        """
        candidates = sorted(
            (
                (
                    self.damage_to(target),
                    target.effective_power,
                    target.initiative,
                    target,
                )
                for target in targets
            ),
            reverse=True,
        )
        return next((target for damage, _, _, target in candidates if damage), None)


@dataclass(frozen=True)
class Army:
    type: ArmyType
    groups: set[Group] = field(hash=False)

    @classmethod
    def from_lines(cls, lines: Iterable[str]) -> "Army":
        """Parse the given lines and return a new Army."""
        it = iter(lines)
        armytype = ArmyType(next(it).rstrip(":"))
        groups: set[Group] = set()
        for line in it:
            groups.add(Group.from_line(line))
        return cls(armytype, groups)

    def boost(self, amount: int) -> "Army":
        """Return a new army with the given boost applied to all groups."""
        return type(self)(
            type=self.type,
            groups={
                replace(group, attackdamage=group.attackdamage + amount)
                for group in self.groups
            },
        )

    def select_targets(self, enemy: "Army") -> Sequence[tuple[Group, Group, "Army"]]:
        # attacker, defender, enemy army
        selected: list[tuple[Group, Group, "Army"]] = []
        available = set(enemy.groups)
        for group in sorted(
            self.groups, key=attrgetter("effective_power", "initiative"), reverse=True
        ):
            target = group.select_target(available)
            if target is None:
                continue
            available.remove(target)
            selected.append((group, target, enemy))
        return selected

    def copy(self) -> "Army":
        """Return a copy of this army."""
        return type(self)(type=self.type, groups=copy.deepcopy(self.groups))

    @property
    def units(self) -> int:
        """Return the number of units in this army."""
        return sum(group.units for group in self.groups)

    def __bool__(self) -> bool:
        """Return True if this army has any units."""
        return bool(self.groups)


def parse_input(data: str) -> tuple[Army, Army]:
    """Parse the input data and return the two armies."""
    immune_system_data, _, infection_data = data.partition("\n\n")
    return (
        Army.from_lines(immune_system_data.splitlines()),
        Army.from_lines(infection_data.splitlines()),
    )


def simulate_battle(immune_system_army: Army, infection_army: Army) -> int:
    while immune_system_army and infection_army:
        selections = [
            *immune_system_army.select_targets(infection_army),
            *infection_army.select_targets(immune_system_army),
        ]
        if not selections:
            raise StalemateError("No targets selected")

        changed = False
        for attacker, defender, defender_army in sorted(
            selections, key=lambda comb: comb[0].initiative, reverse=True
        ):
            before = defender.units
            attacker.attack(defender)
            if defender.units <= 0:
                defender_army.groups.remove(defender)
            if defender.units < before:
                changed = True
        if not changed:
            raise StalemateError("No units were damaged")

    return immune_system_army.units + infection_army.units


def boost_until_victory(data: str) -> int:
    """Boost the immune system army until it wins."""
    immune_system_army, infection_army = parse_input(data)
    for boost in itertools.count(1):
        boosted_immune_system = immune_system_army.boost(boost)
        try:
            simulate_battle(boosted_immune_system, infection_army.copy())
        except StalemateError:
            continue
        if boosted_immune_system:
            return boosted_immune_system.units


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    args = parser.parse_args()

    data = utils.read(day=24, year=2018, test=args.test)
    immune_system_army, infection_army = parse_input(data)

    remaining_units = simulate_battle(immune_system_army, infection_army)
    boosted_immune_system_units = boost_until_victory(data)

    print(f"Part 1: {remaining_units}")
    print(f"Part 2: {boosted_immune_system_units}")
