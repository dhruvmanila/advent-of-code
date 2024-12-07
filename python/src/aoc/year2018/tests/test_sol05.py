from aoc.year2018.day05 import polymer_reaction, shortest_polymer


def test_polymer_reaction():
    assert len(polymer_reaction("dabAcCaCBAcCcaDA")) == 10


def test_shortest_polymer():
    assert shortest_polymer("dabAcCaCBAcCcaDA") == 4
