from itertools import cycle

import utils


def duplicate_frequency(changes: list[int]) -> int:
    frequency = 0
    seen = {frequency}
    for change in cycle(changes):
        frequency += change
        if frequency in seen:
            break
        seen.add(frequency)
    return frequency


if __name__ == "__main__":
    data = utils.read(day=1, year=2018)
    frequencies = list(map(int, data.splitlines()))

    print(f"Part 1: {sum(frequencies)}")
    print(f"Part 2: {duplicate_frequency(frequencies)}")
