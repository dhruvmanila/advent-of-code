import pytest

from year2018.sol25 import Point, count_constallations


@pytest.mark.parametrize(
    "data, expected",
    (
        (
            """\
 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0""",
            2,
        ),
        (
            """\
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0""",
            4,
        ),
        (
            """\
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2""",
            3,
        ),
        (
            """\
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2""",
            8,
        ),
    ),
)
def test_count_constellations(data: str, expected: int) -> None:
    points = [Point.from_line(line) for line in data.splitlines()]
    assert count_constallations(points) == expected
