from aoc.utils import get_sample_input
from aoc.year2018.day06 import compute_area, parse_data


def test_compute_area():
    data = get_sample_input(day=6, year=2018)
    dangerous_area, safe_area = compute_area(*parse_data(data), 32)
    assert dangerous_area == 17
    assert safe_area == 16
