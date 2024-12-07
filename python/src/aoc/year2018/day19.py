import argparse
from collections.abc import Sequence
from typing import cast

from aoc.year2018.cpu import BaseCPU

Instruction = tuple[str, int, int, int]


class CPU(BaseCPU):
    def execute(self, ipregister: int, instructions: Sequence[Instruction]) -> None:
        self.registers[ipregister] = 0
        while 0 <= self.registers[ipregister] < len(instructions):
            opcode, *operands = instructions[self.registers[ipregister]]
            self.opcodes[opcode](*operands)
            self.registers[ipregister] += 1


def sum_of_divisors(n: int) -> int:
    """Return the sum of all divisors of n."""
    return sum(i + n // i for i in range(1, int(n**0.5) + 1) if n % i == 0)


def parse_lines(lines: Sequence[str]) -> tuple[int, Sequence[Instruction]]:
    ipregister = int(lines[0].split()[-1])
    instructions = []
    for line in lines[1:]:
        opcode, *operands = line.split()
        assert len(operands) == 3, f"Invalid instruction: {line!r}"
        instructions.append((opcode, *map(int, operands)))
    return ipregister, cast(Sequence[Instruction], instructions)


def solve(input: str, args: argparse.Namespace) -> None:
    if args.fast:
        if args.sample:
            raise Exception("Sample input not supported with --fast flag.")

        # Analyzing the instructions, we see that it computes the sum of divisors
        # of the given number. In my case, the number is stored in register 5.
        print(f"Part 1: {sum_of_divisors(914)}")
        print(f"Part 2: {sum_of_divisors(10551314)}")
    else:
        ipregister, instructions = parse_lines(input.splitlines())

        cpu = CPU(register_count=6)
        cpu.execute(ipregister, instructions)
        print(f"Part 1: {cpu.registers[0]}")

        if args.sample:
            # Although the test input is small, the intruction pointer register is 0,
            # which means that initializing register 0 with 1 has no effect.
            cpu.reset()
            cpu.registers[0] = 1
            cpu.execute(ipregister, instructions)
            print(f"Part 2: {cpu.registers[0]}")
        else:
            print("Use the `--fast` flag for part 2.")
