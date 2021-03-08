from collections import Counter

# Keys: Layer number starting from 1
# Values: Data on each layer
LAYER_DATA: dict[int, str] = {}

# Keys: Layer number starting from 1
# Values: Counter object representing the count of each individual data in the layer
#         namely the count of "0", "1" and "2"
DATA_COUNT: dict[int, Counter[str]] = {}

# Given data
WIDE = 25
TALL = 6
SIZE = WIDE * TALL

# ░▒▓█▇▆▅▄▃▂
WHITE_PIXEL = "█"
BLACK_PIXEL = " "


def parse_data(data: str) -> None:
    layer_number = 1
    for i in range(0, len(data), SIZE):
        row = data[i : i + SIZE]
        LAYER_DATA.setdefault(layer_number, row)
        DATA_COUNT.setdefault(layer_number, Counter(row))
        layer_number += 1
    # Subtract 1 as we started for layer +1 but we already reached the end of the data.
    assert layer_number - 1 == len(data) / SIZE


def part_a() -> int:
    get_layer_counter = DATA_COUNT.__getitem__
    row_num = min(DATA_COUNT, key=lambda layer_num: get_layer_counter(layer_num)["0"])
    return get_layer_counter(row_num)["1"] * get_layer_counter(row_num)["2"]


def decode_data() -> list[str]:
    image_data = []
    row_data = []
    for pixel in range(SIZE):
        for layer_data in LAYER_DATA.values():
            data_at_pixel = layer_data[pixel]
            if data_at_pixel == "2":  # Transparent pixel
                continue
            elif data_at_pixel == "0":  # Black pixel
                row_data.append(BLACK_PIXEL)
                break
            else:  # White pixel
                row_data.append(WHITE_PIXEL)
                break
        if len(row_data) == 25:
            image_data.append("".join(row_data))
            row_data.clear()
    return image_data


if __name__ == "__main__":
    import os.path

    with open(os.path.join(os.path.dirname(__file__), "input/08.txt")) as fd:
        puzzle_input = fd.read().strip()

    parse_data(puzzle_input)

    print("Part A answer =>", part_a())
    print("Decoded image =>")

    for row in decode_data():
        print(row)
