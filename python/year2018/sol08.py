from __future__ import annotations

from dataclasses import dataclass
from typing import Iterator

import utils


@dataclass
class Node:
    children: list[Node]
    metadata: list[int]

    @classmethod
    def from_datastream(cls, datastream: Iterator[int]) -> Node:
        child_count, metadata_count = next(datastream), next(datastream)
        children = [Node.from_datastream(datastream) for _ in range(child_count)]
        metadata = [next(datastream) for _ in range(metadata_count)]
        return cls(children, metadata)

    @property
    def checksum(self) -> int:
        """Return the sum of metadata for the current and its children node."""
        return sum(self.metadata) + sum(child.checksum for child in self.children)

    @property
    def value(self) -> int:
        """Return the value for the current node."""
        if self.children:
            return sum(
                self.children[idx - 1].value
                for idx in self.metadata
                if idx <= len(self.children)
            )
        return sum(self.metadata)

    def __str__(self) -> str:
        """String representation of Node indented with 2 spaces:

        Node(
          children=[
            ...,
          ],
          metadata=[...],
        )
        """
        children = ""
        for child in self.children:
            children += str(child).replace("\n", "\n    ")
        if children:
            children = f"\n    {children.rstrip()}\n  ]"
        return (
            f"Node(\n  children=[{children or ']'},\n  metadata={self.metadata},\n)\n"
        )


if __name__ == "__main__":
    data = utils.read(day=8, year=2018)
    datastream = map(int, data.split())
    root = Node.from_datastream(datastream)

    print(f"8.1: {root.checksum}")
    print(f"8.2: {root.value}")
