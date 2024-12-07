from string import ascii_uppercase

from aoc.utils import get_sample_input
from aoc.year2018.day07 import execute_steps, parse_graph, topological_sort

TEST_GRAPH = parse_graph(get_sample_input(day=7, year=2018))
TEST_STEPCOST = {letter: cost for cost, letter in enumerate(ascii_uppercase, start=1)}


def test_topological_sort():
    assert topological_sort(TEST_GRAPH) == "CABDFE"


def test_execute_steps():
    assert execute_steps(TEST_GRAPH, 2, TEST_STEPCOST) == 15
