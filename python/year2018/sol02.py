from collections import Counter
from itertools import product

import utils


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


if __name__ == "__main__":
    data = utils.read(day=2, year=2018)
    boxids = data.splitlines()

    print(f"2.1: {checksum(boxids)}")
    print(f"2.2: {matching_ids(boxids)}")
