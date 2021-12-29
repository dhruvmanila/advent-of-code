import utils
from year2018.sol04 import get_guard_schedule, strategy_1, strategy_2


def test_strategy_1():
    data = utils.read(day=4, year=2018, test=True)
    guard_schedule = get_guard_schedule(sorted(data.splitlines()))
    assert strategy_1(guard_schedule) == 240


def test_strategy_2():
    data = utils.read(day=4, year=2018, test=True)
    guard_schedule = get_guard_schedule(sorted(data.splitlines()))
    assert strategy_2(guard_schedule) == 4455
