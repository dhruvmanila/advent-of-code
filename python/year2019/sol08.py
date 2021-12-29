from collections import Counter
from typing import Final, Iterable, Sequence

import utils

WIDE: Final[int] = 25
TALL: Final[int] = 6
SIZE: Final[int] = WIDE * TALL

PIXEL = (" ", "█")


def chunks(seq: Sequence[int], n: int) -> Iterable[Sequence[int]]:
    yield from (seq[i : i + n] for i in range(0, len(seq), n))


def least_zero_layer_counts(image: list[int]) -> Counter[int]:
    layercounts = [Counter(layer) for layer in chunks(image, SIZE)]
    return min(layercounts, key=lambda c: c[0])


def stack_layers(image: list[int]) -> list[int]:
    stacked = [2] * SIZE
    for layer in chunks(image, SIZE):
        for i, pixel in enumerate(layer):
            if stacked[i] == 2:
                stacked[i] = pixel
    return stacked


if __name__ == "__main__":
    data = utils.read(day=8, year=2019)
    image = list(map(int, data))

    layer0 = least_zero_layer_counts(image)
    print(f"8.1: {layer0[1] * layer0[2]}")

    stacked = stack_layers(image)
    print("8.2:")
    for row in chunks(stacked, WIDE):
        for pixel in row:
            print(PIXEL[pixel], end="")
        print()
