from aoc.year2019.intcode import IntcodeComputer


def solve(input: str) -> None:
    intcode_program = list(map(int, input.split(",")))

    print(
        "BOOST keycode =>",
        IntcodeComputer(intcode_program, inputs=[1], return_output=True).run(),
    )
    print(
        "Coordinates of the distress signal =>",
        IntcodeComputer(intcode_program, inputs=[2], return_output=True).run(),
    )
