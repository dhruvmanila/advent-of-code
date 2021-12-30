import utils
from year2018.sol08 import Node


def test_node():
    data = utils.read(day=8, year=2018, test=True)
    datastream = map(int, data.split())
    root = Node.from_datastream(datastream)
    assert root.checksum == 138
    assert root.value == 66
