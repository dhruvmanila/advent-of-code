import pytest

from year2018.sol22 import Cave, Position, parse_input


@pytest.mark.parametrize(
    "data, expected",
    (
        ("depth: 510\ntarget: 10,10", (510, (10, 10))),
        ("depth: 5355\ntarget: 14,796", (5355, (14, 796))),
    ),
)
def test_parse_input(data: str, expected: tuple[int, Position]) -> None:
    assert parse_input(data) == expected


@pytest.mark.parametrize(
    "depth, target, risk_level, min_time",
    (
        (510, (10, 10), 114, 45),
        (5355, (14, 796), 11972, 1092),
    ),
)
def test_cave(depth: int, target: Position, risk_level: int, min_time: int) -> None:
    cave = Cave(depth, target)
    assert cave.risk_level == risk_level
    assert cave.search() == min_time
