from itertools import cycle


def duplicate_frequency(changes: list[int]) -> int:
    frequency = 0
    seen = {frequency}
    for change in cycle(changes):
        frequency += change
        if frequency in seen:
            break
        seen.add(frequency)
    return frequency


def solve(input: str) -> None:
    frequencies = list(map(int, input.splitlines()))

    print(f"Part 1: {sum(frequencies)}")
    print(f"Part 2: {duplicate_frequency(frequencies)}")
