from collections.abc import Callable, Mapping, Sequence
from typing import cast

import utils

Instruction = tuple[str, int, int, int]
Opcode = Callable[[int, int, int], None]


class CPU:
    registers: list[int]
    opcodes: Mapping[str, Opcode]

    def __init__(self) -> None:
        self.registers = [0, 0, 0, 0, 0, 0]
        self.opcodes = {
            attr.removeprefix("op_"): getattr(self, attr)
            for attr in dir(self)
            if attr.startswith("op")
        }

    def op_addr(self, a: int, b: int, c: int) -> None:
        """addr (add register) stores into register C the result of adding register A
        and register B."""
        self.registers[c] = self.registers[a] + self.registers[b]

    def op_addi(self, a: int, b: int, c: int) -> None:
        """addi (add immediate) stores into register C the result of adding register A
        and value B."""
        self.registers[c] = self.registers[a] + b

    def op_mulr(self, a: int, b: int, c: int) -> None:
        """mulr (multiply register) stores into register C the result of multiplying
        register A and register B."""
        self.registers[c] = self.registers[a] * self.registers[b]

    def op_muli(self, a: int, b: int, c: int) -> None:
        """muli (multiply immediate) stores into register C the result of multiplying
        register A and value B."""
        self.registers[c] = self.registers[a] * b

    def op_banr(self, a: int, b: int, c: int) -> None:
        """banr (bitwise AND register) stores into register C the result of the
        bitwise AND of register A and register B."""
        self.registers[c] = self.registers[a] & self.registers[b]

    def op_bani(self, a: int, b: int, c: int) -> None:
        """bani (bitwise AND immediate) stores into register C the result of the
        bitwise AND of register A and value B."""
        self.registers[c] = self.registers[a] & b

    def op_borr(self, a: int, b: int, c: int) -> None:
        """borr (bitwise OR register) stores into register C the result of the
        bitwise OR of register A and register B."""
        self.registers[c] = self.registers[a] | self.registers[b]

    def op_bori(self, a: int, b: int, c: int) -> None:
        """bori (bitwise OR immediate) stores into register C the result of the
        bitwise OR of register A and value B."""
        self.registers[c] = self.registers[a] | b

    def op_setr(self, a: int, _: int, c: int) -> None:
        """setr (set register) copies the contents of register A into register C.
        (Input B is ignored.)"""
        self.registers[c] = self.registers[a]

    def op_seti(self, a: int, _: int, c: int) -> None:
        """seti (set immediate) stores value A into register C. (Input B is ignored.)"""
        self.registers[c] = a

    def op_gtir(self, a: int, b: int, c: int) -> None:
        """gtir (greater-than immediate/register) sets register C to 1 if value A is
        greater than register B. Otherwise, register C is set to 0."""
        self.registers[c] = int(a > self.registers[b])

    def op_gtri(self, a: int, b: int, c: int) -> None:
        """gtri (greater-than register/immediate) sets register C to 1 if register A is
        greater than value B. Otherwise, register C is set to 0."""
        self.registers[c] = int(self.registers[a] > b)

    def op_gtrr(self, a: int, b: int, c: int) -> None:
        """gtrr (greater-than register/register) sets register C to 1 if register A is
        greater than register B. Otherwise, register C is set to 0."""
        self.registers[c] = int(self.registers[a] > self.registers[b])

    def op_eqir(self, a: int, b: int, c: int) -> None:
        """eqir (equal immediate/register) sets register C to 1 if value A is equal to
        register B. Otherwise, register C is set to 0."""
        self.registers[c] = int(a == self.registers[b])

    def op_eqri(self, a: int, b: int, c: int) -> None:
        """eqri (equal register/immediate) sets register C to 1 if register A is equal
        to value B. Otherwise, register C is set to 0."""
        self.registers[c] = int(self.registers[a] == b)

    def op_eqrr(self, a: int, b: int, c: int) -> None:
        """eqrr (equal register/register) sets register C to 1 if register A is equal to
        register B. Otherwise, register C is set to 0."""
        self.registers[c] = int(self.registers[a] == self.registers[b])

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

    cpu = CPU()
    cpu.execute(ipregister, instructions)

    print(f"Part 1: {cpu.registers[0]}")
