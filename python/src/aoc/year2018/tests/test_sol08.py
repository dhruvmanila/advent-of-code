from aoc.utils import get_sample_input
from aoc.year2018.day08 import Node


def test_node():
    data = get_sample_input(day=8, year=2018)
    datastream = map(int, data.split())
    root = Node.from_datastream(datastream)
    assert root.checksum == 138
    assert root.value == 66
