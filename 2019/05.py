# https://adventofcode.com/2019/day/5

from operator import add, eq, getitem, lt, mul, not_, setitem, truth
from typing import MutableSequence, Tuple

INTCODE_PROGRAM = []

with open("input/05.txt") as inp:
    data = inp.readline().strip().split(",")
    INTCODE_PROGRAM.extend(list(map(int, data)))
    del data

OPCODE = {
    1: add,
    2: mul,
    3: lambda program, index: setitem(program, index, int(input("System ID: "))),
    4: lambda _, value: print(value),
    5: truth,
    6: not_,
    7: lt,
    8: eq,
}

PARAM_MODE = {
    0: getitem,  # position mode
    1: lambda _, value: value,  # immediate mode
}


def param_val_and_mode(
    program: MutableSequence[int], pointer: int, parameters_qty: int
) -> Tuple[int, ...]:
    """
    Helper function for executing Intcode program.
    Takes in the program code, current position of the pointer and
    the number of parameters required.
    Returns all the parameters and its mode in the order: (*params, *param_modes)
    """
    instruction = str(program[pointer]).zfill(parameters_qty + 2)
    parameters = program[pointer + 1 : pointer + (parameters_qty + 1)]
    parameters_mode = map(int, instruction[-3::-1])
    return *parameters, *parameters_mode


def execute_intcode(intcode_program: MutableSequence[int]) -> None:
    """
    Executes the intcode_program.
    """
    program = intcode_program[:]  # Never mutate the original reference
    pointer = 0
    # Assignment expression: getting the instruction value at pointer,
    # extracting the code part (last two digit), assigning it to the variable
    # code and checking whether it is 99 (end of program) or not.
    while (code := int(str(program[pointer])[-2::1])) != 99:

        # Arithmetic and comparison
        if code in {1, 2, 7, 8}:
            p1, p2, p3, p1_mode, p2_mode, p3_mode = param_val_and_mode(
                program, pointer, 3
            )
            value1 = PARAM_MODE[p1_mode](program, p1)
            value2 = PARAM_MODE[p2_mode](program, p2)
            # Comparison gives us True or False, so use int to convert it to
            # 1 or 0 respectively.
            program[p3] = int(OPCODE[code](value1, value2))
            pointer += 4

        # Input/Output
        elif code in {3, 4}:
            p1, p1_mode = param_val_and_mode(program, pointer, 1)
            # For code 4 we have to check the parameter mode
            p1 = PARAM_MODE[p1_mode](program, p1) if code == 4 else p1
            OPCODE[code](program, p1)
            pointer += 2

        # Conditional jumps
        elif code in {5, 6}:
            p1, p2, p1_mode, p2_mode = param_val_and_mode(program, pointer, 2)
            check = PARAM_MODE[p1_mode](program, p1)
            jump = PARAM_MODE[p2_mode](program, p2)
            # For code 5(truth): jump if check is True (non-zero)
            # For code 6(not_): jump if check is False (zero)
            pointer = jump if OPCODE[code](check) else pointer + 3


execute_intcode(INTCODE_PROGRAM)
# For first puzzle, system ID is 1
# For second puzzle, system ID is 5
