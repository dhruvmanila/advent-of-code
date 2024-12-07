# TODO: Use the `IntcodeComputer` class
from operator import add, mul

INTCODE_PROGRAM = []

OPCODE = {
    1: add,
    2: mul,
}


def intcode_computer(val1=12, val2=2):
    program = INTCODE_PROGRAM[:]
    program[1], program[2] = val1, val2
    for index in range(0, len(program), 4):
        code, inp1_add, inp2_add, outadd = program[index : index + 4]
        if code == 99:
            break
        program[outadd] = OPCODE[code](program[inp1_add], program[inp2_add])
    return program[0]


EXPECTED = 19690720


def solution2_2():
    for p1 in range(100):
        for p2 in range(100):
            try:
                output = intcode_computer(p1, p2)
                if output == EXPECTED:
                    return 100 * p1 + p2
            except KeyError:
                pass


def solve(input: str) -> None:
    data = list(map(int, input.split(",")))
    INTCODE_PROGRAM.extend(data)

    print(f"First part answer: {intcode_computer()}")
    print(f"Second part answer: {solution2_2()}")
