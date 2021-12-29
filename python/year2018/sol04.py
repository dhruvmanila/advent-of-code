from __future__ import annotations

import enum
import re
from collections import Counter
from typing import Iterable, Mapping

import utils

GuardID = int

SleepDurations = list[range]
"""
List of sleep durations each represented by a range of start (inclusive) to
end (exclusive) minute.
"""

Schedule = Mapping[GuardID, SleepDurations]
"""
Schedule is a mapping from guard id referencing the sleep durations.
"""

LOG_RECORD_RE = re.compile(r"^\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (.*)$")
BEGIN_SHIFT_RE = re.compile(r"^Guard #(\d+) begins shift$")


class State(str, enum.Enum):
    ASLEEP = "falls asleep"
    AWAKE = "wakes up"


def get_guard_schedule(logs: Iterable[str]) -> Schedule:
    guard_schedule: dict[GuardID, list[range]] = {}
    guard_id = sleep_minute = -1

    for log in logs:
        record = LOG_RECORD_RE.fullmatch(log)
        assert record is not None, log
        minute, message = record.groups()
        if match := BEGIN_SHIFT_RE.fullmatch(message):
            guard_id = int(match.group(1))
        elif message == State.ASLEEP:
            sleep_minute = int(minute)
        elif message == State.AWAKE:
            asleep_range = range(sleep_minute, int(minute))
            guard_schedule.setdefault(guard_id, []).append(asleep_range)

    return guard_schedule


def strategy_1(guard_schedule: Schedule) -> int:
    guard_id = max(
        guard_schedule,
        key=lambda guard_id: sum(map(len, guard_schedule[guard_id])),
    )

    asleep_minutes: Counter[int] = Counter()
    for asleep_range in guard_schedule[guard_id]:
        asleep_minutes.update(asleep_range)

    minute, _ = asleep_minutes.most_common(1)[0]
    return guard_id * minute


def strategy_2(guard_schedule: Schedule) -> int:
    asleep_guard_minutes: Counter[tuple[int, int]] = Counter()

    for guard_id, schedule in guard_schedule.items():
        for asleep_range in schedule:
            for minute in asleep_range:
                asleep_guard_minutes[(guard_id, minute)] += 1

    ((guard_id, minute), _) = asleep_guard_minutes.most_common(1)[0]
    return guard_id * minute


if __name__ == "__main__":
    data = utils.read(day=4, year=2018)
    logs = sorted(data.splitlines())
    guard_schedule = get_guard_schedule(logs)

    print(f"4.1: {strategy_1(guard_schedule)}")
    print(f"4.2: {strategy_2(guard_schedule)}")
