from aoc.utils import get_sample_input
from aoc.year2018.day03 import Rectangle, no_overlap_id, sum_overlap


def test_parsing():
    r = Rectangle.from_line("#123 @ 3,2: 5x4")
    assert r.id == 123
    assert r.left == 3
    assert r.top == 2
    assert r.width == 5
    assert r.height == 4
    assert r.right == 3 + 5
    assert r.bottom == 2 + 4


def test_sum_overlap():
    data = get_sample_input(day=3, year=2018)
    rectangles = map(Rectangle.from_line, data.splitlines())
    assert sum_overlap(rectangles) == 4


def test_no_overlap_id():
    data = get_sample_input(day=3, year=2018)
    rectangles = map(Rectangle.from_line, data.splitlines())
    assert no_overlap_id(rectangles) == 3
