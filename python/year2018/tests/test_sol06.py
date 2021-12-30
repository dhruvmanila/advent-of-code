import utils
from year2018.sol06 import compute_area, parse_data


def test_compute_area():
    data = utils.read(day=6, year=2018, test=True)
    dangerous_area, safe_area = compute_area(*parse_data(data), 32)
    assert dangerous_area == 17
    assert safe_area == 16
