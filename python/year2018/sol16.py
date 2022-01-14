import re
from dataclasses import dataclass
from typing import Callable, Iterable, Mapping, Sequence

import utils

Opcode = Callable[[int, int, int], None]


@dataclass(frozen=True)
class Sample:
    registers: Sequence[int]
    instruction: Sequence[int]
    expected: Sequence[int]


class CPU:
    registers: list[int]
    opcodes: Mapping[str, Opcode]

    def __init__(self) -> None:
        self.registers = [0, 0, 0, 0]
        self.opcodes = {
            attr: getattr(self, attr) for attr in dir(self) if attr.startswith("op")
        }

    def reset(self) -> None:
        """Reset the registers."""
        self.registers = [0, 0, 0, 0]

    def execute_sample(self, sample: Sample) -> set[str]:
        """Returns a set of opcode names which matches the given sample."""
        possible = set()
        for name, opcode in self.opcodes.items():
            self.registers[:] = sample.registers
            opcode(*sample.instruction[1:])
            if self.registers == sample.expected:
                possible.add(name)
        return possible

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


def compute(
    samples: Iterable[Sample], instructions: Iterable[list[int]]
) -> tuple[int, int]:
    cpu = CPU()
    similar_count = 0
    possible_opcodes: dict[int, set[str]] = {}
    for sample in samples:
        opcodes = cpu.execute_sample(sample)
        possible_opcodes.setdefault(sample.instruction[0], set()).update(opcodes)
        if len(opcodes) > 2:
            similar_count += 1

    opcode_map: dict[int, Opcode] = {}
    while len(opcode_map) < 16:
        for op, opcodes in possible_opcodes.items():
            if len(opcodes) == 1:
                opcode = opcodes.pop()
                opcode_map[op] = cpu.opcodes[opcode]
                for opcodes in possible_opcodes.values():
                    opcodes.discard(opcode)

    cpu.reset()
    for op, *operands in instructions:
        opcode_map[op](*operands)

    return similar_count, cpu.registers[0]


def parse_data(data: str) -> tuple[Sequence[Sample], Sequence[list[int]]]:
    samples: list[Sample] = []
    sample_section, instruction_section = data.split("\n\n\n\n")
    for section in sample_section.split("\n\n"):
        lines = section.splitlines()
        registers = [int(m) for m in re.findall(r"\d+", lines[0])]
        instruction = [int(s) for s in lines[1].split()]
        expected = [int(m) for m in re.findall(r"\d+", lines[2])]
        samples.append(Sample(registers, instruction, expected))
    return samples, [
        [int(s) for s in line.split()] for line in instruction_section.splitlines()
    ]


if __name__ == "__main__":
    data = utils.read(day=16, year=2018, test=False)
    samples, instructions = parse_data(data)
    similar_count, register0 = compute(samples, instructions)

    print(f"16.1: {similar_count}")
    print(f"16.2: {register0}")
