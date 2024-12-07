from collections import Counter
from itertools import product


def checksum(boxids: list[str]) -> int:
    twos, threes = 0, 0
    for boxid in boxids:
        counts = Counter(boxid).values()
        twos += 2 in counts
        threes += 3 in counts
    return twos * threes


def matching_ids(boxids: list[str]) -> str:
    for id1, id2 in product(boxids, boxids):
        if id1 == id2:
            continue
        char = ""
        diff = 0
        for a, b in zip(id1, id2):
            if a != b:
                diff += 1
                char = a
            if diff > 1:
                break
        else:
            break
    return id1.replace(char, "")


def solve(input: str) -> None:
    boxids = input.splitlines()

    print(f"2.1: {checksum(boxids)}")
    print(f"2.2: {matching_ids(boxids)}")
