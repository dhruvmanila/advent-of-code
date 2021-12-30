from string import ascii_uppercase

import utils
from year2018.sol07 import execute_steps, parse_graph, topological_sort

TEST_GRAPH = parse_graph(utils.read(day=7, year=2018, test=True))
TEST_STEPCOST = {letter: cost for cost, letter in enumerate(ascii_uppercase, start=1)}


def test_topological_sort():
    assert topological_sort(TEST_GRAPH) == "CABDFE"


def test_execute_steps():
    assert execute_steps(TEST_GRAPH, 2, TEST_STEPCOST) == 15
