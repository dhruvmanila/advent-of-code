from aoc.year2019.intcode import IntcodeComputer


def solve(input: str) -> None:
    intcode_program = list(map(int, input.split(",")))

    print(
        "System ID: 1 =>",
        IntcodeComputer(intcode_program, inputs=[1], gather_output=True).run(),
    )
    print(
        "System ID: 5 =>",
        IntcodeComputer(intcode_program, inputs=[5], return_output=True).run(),
    )
