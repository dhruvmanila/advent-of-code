from __future__ import annotations

from typing import Iterable, Mapping

DGraph = Mapping[str, set[str]]
"""
A directed graph is a mapping of node names referencing a set of other node names.

This is establishing a parent to child relationship between the nodes.
"""

Orbiting = Mapping[str, str]
"""
Orbiting is a mapping of orbiting node referencing the center node.

This is establishing a child to parent relationship between the nodes.
"""


def parse_orbit_map(lines: Iterable[str]) -> tuple[DGraph, Orbiting]:
    graph: dict[str, set[str]] = {}
    orbiting: dict[str, str] = {}
    for line in lines:
        center, orbiter = line.split(")")
        graph.setdefault(center, set()).add(orbiter)
        orbiting[orbiter] = center
    return graph, orbiting


def count_orbits(graph: DGraph) -> int:
    def walk(node: str, depth: int) -> int:
        total = depth
        for child in graph.get(node, ()):
            total += walk(child, depth + 1)
        return total

    return walk("COM", 0)


def minimum_orbital_transfers(
    orbiting: Orbiting, from_: str = "YOU", to: str = "SAN"
) -> int:
    from_path = set()
    to_path = set()

    from_parent = orbiting[from_]
    while from_parent != "COM":
        from_path.add(from_parent)
        from_parent = orbiting[from_parent]

    to_parent = orbiting[to]
    while to_parent != "COM":
        to_path.add(to_parent)
        to_parent = orbiting[to_parent]

    return len(from_path ^ to_path)


def solve(input: str) -> None:
    graph, orbiting = parse_orbit_map(input.splitlines())

    print(f"6.1: {count_orbits(graph)}")
    print(f"6.2: {minimum_orbital_transfers(orbiting)}")
