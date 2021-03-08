try:
    from intcode import IntcodeComputer
except ImportError:
    from .intcode import IntcodeComputer

Position = tuple[int, int]

# ░▒▓█▇▆▅▄▃▂
COLOR = {0: " ", 1: "█"}


def move_robot(curr_position, turn_right, pointing):
    x, y = curr_position
    if pointing == "U":
        return ((x + 1, y), "R") if turn_right else ((x - 1, y), "L")
    elif pointing == "D":
        return ((x - 1, y), "L") if turn_right else ((x + 1, y), "R")
    elif pointing == "R":
        return ((x, y - 1), "D") if turn_right else ((x, y + 1), "U")
    else:
        return ((x, y + 1), "U") if turn_right else ((x, y - 1), "D")


def painted_panels(
    intcode_program: list[int], starting_input: int, get_image_meta: bool
) -> dict[Position, int]:
    computer = IntcodeComputer(
        intcode_program, inputs=[starting_input], return_output=True
    )
    if get_image_meta:
        xmin = xmax = ymin = ymax = 0
    panels_painted = {}
    position: Position = (0, 0)
    pointing = "U"
    while True:
        color = computer.run()
        if computer.halted():
            break
        panels_painted[position] = color
        if get_image_meta:
            x, y = position
            xmin = min(xmin, x)
            xmax = max(xmax, x)
            ymin = min(ymin, y)
            ymax = max(ymax, y)
        direction = computer.run()
        position, pointing = move_robot(position, direction, pointing)
        computer.inputs.append(panels_painted.get(position, 0))
    if get_image_meta:
        return panels_painted, (xmin, ymax), (abs(xmin) + xmax), (abs(ymin) + ymax)
    else:
        return panels_painted


def render_image(panels_painted, origin: Position, wide: int, tall: int) -> None:
    x0, y0 = origin
    for y in range(tall + 1):
        for x in range(wide + 1):
            color = panels_painted.get((x0 + x, y0 - y), 0)
            print(COLOR[color], end="")
        print()


if __name__ == "__main__":
    from pathlib import Path

    intcode_program = list(
        map(
            int,
            (
                Path(__file__)
                .parent.joinpath("input", "11.txt")
                .read_text()
                .strip()
                .split(",")
            ),
        )
    )

    image_data = painted_panels(intcode_program, 0, False)
    print(f"Number of panels painted => {len(image_data)}")

    image_data, origin, wide, tall = painted_panels(intcode_program, 1, True)
    print("Registration identifier =>")
    render_image(image_data, origin, wide, tall)
