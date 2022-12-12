import functools
import math
import operator
import re
from collections.abc import Callable, Iterable, Sequence
from dataclasses import dataclass, field

import utils

NOTES_RE = re.compile(
    """\
Monkey (?P<id>\d+):
  Starting items: (?P<items>.+)
  Operation: new = (?P<left_operand>(?:old|\d+)) (?P<operator>[*+]) (?P<right_operand>(?:old|\d+))
  Test: divisible by (?P<mod>\d+)
    If true: throw to monkey (?P<true_target>\d+)
    If false: throw to monkey (?P<false_target>\d+)""",
    re.MULTILINE,
)


def square(a: int) -> int:
    """Return a * a"""
    return a * a


@dataclass(frozen=False, order=True)
class Monkey:
    id: int = field(compare=False)
    items: list[int] = field(compare=False)
    operation: Callable[[int], int] = field(compare=False)
    mod: int = field(compare=False)
    true_target: int = field(compare=False)
    false_target: int = field(compare=False)

    inspected: int = field(init=False, default=0)

    @classmethod
    def from_note(cls, notes: str) -> "Monkey":
        m = NOTES_RE.match(notes)
        if m is None:
            raise ValueError(f"failed to match: {notes!r}")

        id_, mod, true_target, false_target = map(
            int, m.group("id", "mod", "true_target", "false_target")
        )
        items = [int(item) for item in m["items"].split(",")]

        match [m["left_operand"], m["operator"], m["right_operand"]]:
            case ["old", "*", "old"]:
                operation = functools.partial(square)
            case ["old", "+", value] | [value, "+" "old"]:
                operation = functools.partial(operator.add, int(value))
            case ["old", "*", value] | [value, "*", "old"]:
                operation = functools.partial(operator.mul, int(value))
            case _:
                raise ValueError(
                    f"invalid operation: "
                    + f"'{m['left_operand']} {m['operator']} {m['right_operand']}'"
                )

        return cls(
            id=id_,
            items=items,
            operation=operation,
            mod=mod,
            true_target=true_target,
            false_target=false_target,
        )


@dataclass(frozen=False)
class Simulator:
    monkeys: Sequence[Monkey]

    def __post_init__(self):
        self.mod = math.prod((m.mod for m in self.monkeys))

    @classmethod
    def from_notes(cls, notes: Iterable[str]) -> "Simulator":
        return cls([Monkey.from_note(note) for note in notes])

    def _do_round(self, *, reduce_worry: bool) -> None:
        for m in self.monkeys:
            while m.items:
                m.inspected += 1
                worry = m.items.pop(0)
                worry = m.operation(worry)
                if reduce_worry:
                    worry = math.floor(worry / 3)
                else:
                    worry %= self.mod
                target = m.true_target if worry % m.mod == 0 else m.false_target
                self.monkeys[target].items.append(worry)

    def do(self, rounds: int, *, reduce_worry: bool = True) -> None:
        for _ in range(rounds):
            self._do_round(reduce_worry=reduce_worry)

    @property
    def monkey_business(self) -> int:
        return math.prod((m.inspected for m in sorted(self.monkeys, reverse=True)[:2]))


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    args = parser.parse_args()

    notes = utils.read(day=11, year=2022, test=args.test).split("\n\n")

    simulator = Simulator.from_notes(notes)
    simulator.do(20)
    print(f"11.1: {simulator.monkey_business}")

    simulator = Simulator.from_notes(notes)
    simulator.do(10000, reduce_worry=False)
    print(f"11.2: {simulator.monkey_business}")
