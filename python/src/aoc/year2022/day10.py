from collections.abc import Sequence

LIT_PIXEL = "█"

# Initialize the pixels by setting them all off.
PIXELS = ["."] * 240

# Interesting cycles to get the signal strength for.
INTERESTING_CYCLE = range(20, 220 + 1, 40)


def run_program(instructions: Sequence[str]) -> int:
    register_x = 1
    current_cycle = 0
    signal = 0

    for instruction in instructions:
        match instruction.split():
            case ["noop"]:
                cycles = 1
                value = 0
            case ["addx", n]:
                cycles = 2
                value = int(n)
            case _:
                raise ValueError(f"invalid instruction: {instruction!r}")

        for _ in range(cycles):
            if register_x - 1 <= current_cycle % 40 <= register_x + 1:
                PIXELS[current_cycle] = LIT_PIXEL
            current_cycle += 1
            if current_cycle in INTERESTING_CYCLE:
                signal += current_cycle * register_x

        register_x += value

    return signal


def solve(input: str) -> None:
    signal = run_program(input.splitlines())

    print(f"10.1: {signal}")
    print("10.2: ")
    for i in range(6):
        print("".join(PIXELS[i * 40 : (i + 1) * 40]))
