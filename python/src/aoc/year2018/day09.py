from __future__ import annotations

import re
from collections import deque
from typing import Generic, TypeVar

LINE_RE = re.compile(r"^(\d+) players; last marble is worth (\d+) points$")

T = TypeVar("T")


class Node(Generic[T]):
    def __init__(
        self, value: T, prev: Node | None = None, next: Node | None = None
    ) -> None:
        self.value = value
        self.prev = prev
        self.next = next


class CircularDoublyLinkedList(Generic[T]):
    def __init__(self) -> None:
        self.head: Node | None = None
        self.length: int = 0

    def append(self, value: T) -> None:
        node = Node(value)
        if self.head is None:
            self.head = node
            self.head.prev = node
            self.head.next = node
        else:
            temp = self.head.prev
            assert temp is not None
            temp.next = node
            node.prev = temp
            node.next = self.head
            self.head.prev = node
        self.length += 1

    def pop(self) -> T:
        if self.head is None:
            raise IndexError("pop from an empty list")
        node = self.head.prev
        if node is self.head:
            self.head = None
        else:
            assert node is not None
            temp = node.prev
            assert temp is not None
            temp.next = self.head
            self.head.prev = temp
        self.length -= 1
        return node.value

    def rotate(self, n: int = 1) -> None:
        # If n > 0, circle moves clockwise, head moves backward.
        # If n < 0, circle moves counter-clockwise, head moves forward.
        shift = "prev" if n > 0 else "next"
        for _ in range(abs(n)):
            self.head = getattr(self.head, shift)

    def is_empty(self) -> bool:
        return self.head is None

    def __len__(self) -> int:
        return self.length

    def __str__(self) -> str:
        if self.head is None:
            return "None"
        values = [self.head.value]
        current = self.head.next
        while current != self.head:
            assert current is not None
            values.append(current.value)
            current = current.next
        return " <-> ".join(map(str, values))


def highest_score(playercount: int, marbles: int) -> int:
    score = [0] * playercount
    # Invariant: Current marble is last in the deque
    circle = deque([0])

    # Custom implementation takes a long time: 18s
    # circle: CircularDoublyLinkedList[int] = CircularDoublyLinkedList()
    # circle.append(0)

    for marble in range(1, marbles + 1):
        if marble % 23 == 0:
            circle.rotate(7)
            # (marble - 1) is the current marble value (circle[-1]).
            score[(marble - 1) % playercount] += marble + circle.pop()
            circle.rotate(-1)
        else:
            # To move the pointer to the current marble (last in circle) to the right,
            # we need to move the circle counter-clockwise.
            circle.rotate(-1)
            circle.append(marble)

    return max(score)


def parse_input(line: str) -> tuple[int, int]:
    match = LINE_RE.search(line)
    assert match is not None, line
    playercount, marbles = map(int, match.groups())
    return playercount, marbles


def solve(input: str) -> None:
    playercount, marbles = parse_input(input)

    print(f"9.1: {highest_score(playercount, marbles)}")
    print(f"9.2: {highest_score(playercount, marbles * 100)}")
