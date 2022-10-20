from collections.abc import Sequence
from typing import cast

import utils
from common.year2018.cpu import BaseCPU

Instruction = tuple[str, int, int, int]


class CPU(BaseCPU):
    def execute(self, ipregister: int, instructions: Sequence[Instruction]) -> None:
        self.registers[ipregister] = 0
        while 0 <= self.registers[ipregister] < len(instructions):
            opcode, *operands = instructions[self.registers[ipregister]]
            self.opcodes[opcode](*operands)
            self.registers[ipregister] += 1


def parser_lines(lines: Sequence[str]) -> tuple[int, Sequence[Instruction]]:
    ipregister = int(lines[0].split()[-1])
    instructions = []
    for line in lines[1:]:
        opcode, *operands = line.split()
        assert len(operands) == 3, f"Invalid instruction: {line!r}"
        instructions.append((opcode, *map(int, operands)))
    return ipregister, cast(Sequence[Instruction], instructions)


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    args = parser.parse_args()

    lines = utils.read(day=19, year=2018, test=args.test).splitlines()
    ipregister, instructions = parser_lines(lines)

    cpu = CPU(register_count=6)
    cpu.execute(ipregister, instructions)

    print(f"Part 1: {cpu.registers[0]}")
