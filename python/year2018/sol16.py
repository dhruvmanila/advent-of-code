import re
from dataclasses import dataclass
from typing import Iterable, Sequence

import utils
from common.year2018.cpu import BaseCPU, OpcodeFunc


@dataclass(frozen=True)
class Sample:
    registers: Sequence[int]
    instruction: Sequence[int]
    expected: Sequence[int]


class CPU(BaseCPU):
    def execute_sample(self, sample: Sample) -> set[str]:
        """Returns a set of opcode names which matches the given sample."""
        possible = set()
        for name, opcode in self.opcodes.items():
            self.registers[:] = sample.registers
            opcode(*sample.instruction[1:])
            if self.registers == sample.expected:
                possible.add(name)
        return possible


def compute(
    samples: Iterable[Sample], instructions: Iterable[list[int]]
) -> tuple[int, int]:
    cpu = CPU(register_count=4)
    similar_count = 0
    possible_opcodes: dict[int, set[str]] = {}
    for sample in samples:
        opcodes = cpu.execute_sample(sample)
        possible_opcodes.setdefault(sample.instruction[0], set()).update(opcodes)
        if len(opcodes) > 2:
            similar_count += 1

    opcode_map: dict[int, OpcodeFunc] = {}
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
