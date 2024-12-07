from __future__ import annotations

import re
from dataclasses import dataclass, field
from enum import Enum
from typing import TYPE_CHECKING, Sequence

CART_RE = re.compile(r"[><^v]")


class Direction(str, Enum):
    UP = "^", -1, 0, "LEFT", "RIGHT"
    DOWN = "v", 1, 0, "RIGHT", "LEFT"
    LEFT = "<", 0, -1, "DOWN", "UP"
    RIGHT = ">", 0, 1, "UP", "DOWN"

    if TYPE_CHECKING:
        from typing import Mapping

        dy: int
        dx: int
        turns: Mapping[str, str]

    # https://docs.python.org/3/library/enum.html#when-to-use-new-vs-init
    def __new__(cls, value: str, dy: int, dx: int, left: str, right: str) -> Direction:
        obj = str.__new__(cls, value)
        obj._value_ = value
        obj.dy = dy
        obj.dx = dx
        obj.turns = {"LEFT": left, "RIGHT": right}
        return obj

    def make_turn(self, move: "Move") -> Direction:
        turn = self.turns.get(move.name, self.name)
        return type(self)[turn]


class Move(int, Enum):
    LEFT = 0
    STRAIGHT = 1
    RIGHT = 2

    @property
    def next(self) -> Move:
        cls = type(self)
        return cls((self.value + 1) % len(cls))


@dataclass(frozen=True, order=True)
class Cart:
    y: int
    x: int
    direction: Direction = field(compare=False)
    intersection_move: Move = field(default=Move.RIGHT, compare=False)

    @property
    def position(self) -> tuple[int, int]:
        return self.y, self.x

    def move(self, track: str) -> Cart:
        y, x = self.position
        direction = self.direction
        intersection_move = self.intersection_move
        if track == "+":
            intersection_move = intersection_move.next
            direction = self.direction.make_turn(intersection_move)
        elif track == "/":
            direction = self.direction.make_turn(
                Move.LEFT
                if self.direction in (Direction.LEFT, Direction.RIGHT)
                else Move.RIGHT
            )
        elif track == "\\":
            direction = self.direction.make_turn(
                Move.RIGHT
                if self.direction in (Direction.LEFT, Direction.RIGHT)
                else Move.LEFT
            )
        x += direction.dx
        y += direction.dy
        return type(self)(y, x, direction, intersection_move)


class TrackMap:
    def __init__(self, tracks: Sequence[str], carts: set[Cart]) -> None:
        self.tracks = tracks
        self.initial_state = carts.copy()
        self.carts = carts

    @classmethod
    def from_lines(cls, lines: Sequence[str]) -> TrackMap:
        cleaned = []
        carts: set[Cart] = set()
        for y, line in enumerate(lines):
            for match in CART_RE.finditer(line):
                x, char = match.start(), match.group(0)
                direction = Direction(char)  # type: ignore
                section = "|" if char in {"^", "v"} else "-"
                line = line[:x] + section + line[x + 1 :]
                carts.add(Cart(y, x, direction))
            cleaned.append(line)
        return cls(cleaned, carts)

    def at(self, y: int, x: int) -> str:
        return self.tracks[y][x]

    def step(self, *, remove_collided: bool) -> tuple[int, int] | None:
        next_carts = self.carts.copy()
        for cart in sorted(self.carts):
            try:
                next_carts.remove(cart)
            except KeyError:
                continue
            cart = cart.move(self.at(*cart.position))
            if cart in next_carts:
                if not remove_collided:
                    return cart.position
                next_carts.remove(cart)
                continue
            next_carts.add(cart)
        self.carts = next_carts
        return None

    def run_carts(self, *, remove_collided: bool) -> tuple[int, int]:
        while len(self.carts) != 1:
            pos = self.step(remove_collided=remove_collided)
            if pos is not None:
                return pos[::-1]
        (cart,) = self.carts
        return cart.position[::-1]

    def reset(self):
        self.carts = self.initial_state

    def __str__(self) -> str:
        lines = [list(s) for s in self.tracks]
        for cart in self.carts:
            lines[cart.y][cart.x] = cart.direction
        return "\n".join("".join(line) for line in lines)


def solve(input: str) -> None:
    tracks = TrackMap.from_lines(input.splitlines())
    print(f"13.1: {tracks.run_carts(remove_collided=False)}")
    tracks.reset()
    print(f"13.2: {tracks.run_carts(remove_collided=True)}")
