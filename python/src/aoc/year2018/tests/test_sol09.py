import pytest

from aoc.year2018.day09 import highest_score


@pytest.mark.parametrize(
    ("playercount", "marbles", "expected"),
    (
        (9, 25, 32),
        (10, 1618, 8317),
        (13, 7999, 146373),
        (17, 1104, 2764),
        (21, 6111, 54718),
        (30, 5807, 37305),
    ),
)
def test_highest_score(playercount: int, marbles: int, expected: int):
    assert highest_score(playercount, marbles) == expected
