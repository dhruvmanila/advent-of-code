# https://adventofcode.com/2019/day/5


if __name__ == "__main__":
    import os.path

    try:
        from intcode import IntcodeComputer
    except ImportError:
        from .intcode import IntcodeComputer

    with open(os.path.join(os.path.dirname(__file__), "input/05.txt")) as fd:
        intcode_program = list(map(int, fd.readline().strip().split(",")))

    print(
        "System ID: 1 =>",
        IntcodeComputer(intcode_program, inputs=[1], gather_output=True).run(),
    )
    print(
        "System ID: 5 =>",
        IntcodeComputer(intcode_program, inputs=[5], return_output=True).run(),
    )
