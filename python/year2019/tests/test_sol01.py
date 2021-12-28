import pytest

from year2019.sol01 import fuel_requirement, recursive_fuel_calc


@pytest.mark.parametrize(
    ("mass", "expected"),
    (
        (12, 2),
        (14, 2),
        (1969, 654),
        (100756, 33583),
    ),
)
def test_fuel_requirement(mass: int, expected: int):
    assert fuel_requirement(mass) == expected


@pytest.mark.parametrize(
    ("mass", "expected"),
    (
        (14, 2),
        (1969, 966),
        (100756, 50346),
    ),
)
def test_recursive_fuel_calc(mass: int, expected: int):
    assert recursive_fuel_calc(mass) == expected
