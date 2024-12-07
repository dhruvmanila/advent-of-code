import pytest

from aoc.year2018.day11 import power_level


@pytest.mark.parametrize(
    ("x", "y", "serial_number", "expected"),
    (
        (3, 5, 8, 4),
        (122, 79, 57, -5),
        (217, 196, 39, 0),
        (101, 153, 71, 4),
    ),
)
def test_power_level(x: int, y: int, serial_number: int, expected: int):
    assert power_level(x, y, serial_number) == expected
