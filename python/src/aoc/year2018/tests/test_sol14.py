import pytest

from aoc.year2018.day14 import recipe_chart, recipe_chart_substring


@pytest.mark.parametrize(
    ("recipes, expected"),
    (
        (9, "5158916779"),
        (5, "0124515891"),
        (18, "9251071085"),
        (2018, "5941429882"),
    ),
)
def test_recipe_chart(recipes, expected):
    assert recipe_chart(recipes) == expected


@pytest.mark.parametrize(
    ("target, expected"),
    (
        ("51589", 9),
        ("01245", 5),
        ("92510", 18),
        ("59414", 2018),
    ),
)
def test_recipe_chart_substring(target, expected):
    assert recipe_chart_substring(list(map(int, target))) == expected
