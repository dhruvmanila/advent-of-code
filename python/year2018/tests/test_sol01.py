import pytest

from year2018.sol01 import duplicate_frequency


@pytest.mark.parametrize(
    ("changes", "expected"),
    (
        ([1, -1], 0),
        ([3, 3, 4, -2, -4], 10),
        ([-6, 3, 8, 5, -6], 5),
        ([7, 7, -2, -7, -4], 14),
    ),
)
def test_duplicate_frequency(changes: list[int], expected: int):
    assert duplicate_frequency(changes) == expected
