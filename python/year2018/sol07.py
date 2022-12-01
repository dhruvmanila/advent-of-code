import re
from heapq import heapify, heappop, heappush
from string import ascii_uppercase
from typing import Mapping

import utils

Graph = Mapping[str, set[str]]
"""
A Graph is a mapping of node names referencing a set of other node names.

This is establishing a parent to child relationship between the nodes.
"""

INSTRUCTION_RE = re.compile(
    r"Step ([A-Z]) must be finished before step ([A-Z]) can begin\."
)

DEFAULT_STEPCOST: Mapping[str, int] = {
    letter: cost for cost, letter in enumerate(ascii_uppercase, start=61)
}


def parse_graph(data: str) -> Graph:
    graph: dict[str, set[str]] = {}
    for dep, node in INSTRUCTION_RE.findall(data):
        graph.setdefault(dep, set()).add(node)
    return graph


def edges_from_graph(graph: Graph) -> Mapping[str, set[str]]:
    edges: dict[str, set[str]] = {}
    for node, deps in graph.items():
        for d in deps:
            edges.setdefault(d, set()).add(node)
    return edges


def topological_sort(graph: Graph) -> str:
    result: list[str] = []

    edges = edges_from_graph(graph)

    # Determine the nodes which have no incoming edges.
    # Any key in graph that's not a key in edges.
    start = graph.keys() - edges

    # Priority queue to choose the next best node (alphabetical order).
    queue = list(start)
    heapify(queue)

    while queue:
        node = heappop(queue)
        result.append(node)
        for dep in graph.get(node, ()):
            edges[dep].remove(node)
            if not edges[dep]:
                heappush(queue, dep)

    return "".join(result)


def execute_steps(
    graph: Graph,
    worker_count: int,
    stepcost: Mapping[str, int] = DEFAULT_STEPCOST,
) -> int:
    time = 0

    edges = edges_from_graph(graph)

    # Determine the nodes which have no incoming edges.
    # Any key in graph that's not a key in edges.
    start = graph.keys() - edges

    # Priority queue to choose the next best node (alphabetical order).
    queue = list(start)
    heapify(queue)

    workers: list[tuple[int, str]] = []

    while queue or workers:
        # Move items from the queue to worker
        while queue and len(workers) < worker_count:
            node = heappop(queue)
            complete_at = time + stepcost[node]
            heappush(workers, (complete_at, node))

        time, node = heappop(workers)
        for dep in graph.get(node, ()):
            edges[dep].remove(node)
            if not edges[dep]:
                heappush(queue, dep)

    return time


def render_graph(g: Graph, **graph_attr) -> None:
    import subprocess
    import tempfile

    import graphviz

    dot = graphviz.Digraph(strict=True)
    dot.attr(**graph_attr)
    dot.attr("node", shape="circle")

    with dot.subgraph(graph_attr={"rank": "min"}) as startnodes:
        for node in set(g) - set().union(*g.values()):
            startnodes.node(node)
    with dot.subgraph(graph_attr={"rank": "max"}) as endnodes:
        for node in set().union(*g.values()) - set(g):
            endnodes.node(node)
    for dep, nodes in sorted(g.items()):
        dot.edges((dep, n) for n in nodes)

    try:
        # Attempt to process the graph through tred, if available
        result = subprocess.run(
            "tred", input=dot.source, capture_output=True, encoding="utf8"
        )
        if result.returncode == 0:
            dot = graphviz.Source(result.stdout)
    except OSError:
        pass

    dot.render(
        tempfile.mktemp(prefix="aoc_2018_07_graph_"),
        format="png",
        view=True,
        cleanup=True,
    )


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    parser.add_argument("--render", action="store_true", help="render the graph")
    args = parser.parse_args()

    data = utils.read(day=7, year=2018, test=args.test)
    graph = parse_graph(data)

    print(f"7.1: {topological_sort(graph)}")
    print(f"7.2: {execute_steps(graph, 5)}")

    if args.render:
        render_graph(graph, rankdir="LR")
