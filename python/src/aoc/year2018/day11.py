def power_level(x: int, y: int, serial: int) -> int:
    rackid = x + 10
    result = (rackid * y + serial) * rackid
    return (result // 100) % 10 - 5


# https://en.wikipedia.org/wiki/Summed-area_table
def summed_area_table(serial: int) -> list[list[int]]:
    table = [[0] * 301 for _ in range(301)]
    for y in range(1, 301):
        for x in range(1, 301):
            table[x][y] = (
                power_level(x, y, serial)
                + table[x - 1][y]
                + table[x][y - 1]
                - table[x - 1][y - 1]
            )
    return table


def summed_area(x: int, y: int, size: int, table: list[list[int]]) -> int:
    return (
        table[x + size - 1][y + size - 1]
        + table[x - 1][y - 1]
        - table[x + size - 1][y - 1]
        - table[x - 1][y + size - 1]
    )


def largest_total_power(table: list[list[int]], size: int) -> tuple[int, int, int]:
    max_power = 0
    position = (0, 0)
    for y in range(1, 301 - size):
        for x in range(1, 301 - size):
            current_power = summed_area(x, y, size, table)
            if current_power > max_power:
                max_power = current_power
                position = x, y
    return max_power, *position


def solve(input: str) -> None:
    table = summed_area_table(int(input))
    _, x1, y1 = largest_total_power(table, 3)
    _, x2, y2, size = max(
        largest_total_power(table, size) + (size,) for size in range(1, 301)
    )

    print(f"11.1: {x1},{y1}")
    print(f"11.2: {x2},{y2},{size}")
