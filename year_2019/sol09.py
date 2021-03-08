# https://adventofcode.com/2019/day/9


if __name__ == "__main__":
    import os

    try:
        from intcode import IntcodeComputer
    except ImportError:
        from .intcode import IntcodeComputer

    with open(os.path.join(os.path.dirname(__file__), "input/09.txt")) as fd:
        intcode_program = list(map(int, fd.readline().strip().split(",")))

    print(
        "BOOST keycode =>",
        IntcodeComputer(intcode_program, inputs=[1], return_output=True).run(),
    )
    print(
        "Coordinates of the distress signal =>",
        IntcodeComputer(intcode_program, inputs=[2], return_output=True).run(),
    )
