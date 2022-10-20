from collections.abc import Callable, Mapping
from typing import TypeAlias

__all__ = ("BaseCPU", "OpcodeFunc")

OpcodeFunc: TypeAlias = Callable[[int, int, int], None]


class BaseCPU:
    """Base class for the CPU used in the 2018 Advent of Code.

    This class provides the common functionality like the registers and the
    opcodes. It is not meant to be used directly, but rather as a base class
    for the CPU used in the specific puzzle.
    """

    registers: list[int]

    opcodes: Mapping[str, OpcodeFunc]
    """Mapping of opcode names to their functions."""

    def __init__(self, register_count: int) -> None:
        """Initialize the CPU with the given number of registers all set to 0."""
        self.registers = [0] * register_count
        self.opcodes = {
            attr.removeprefix("op_"): getattr(self, attr)
            for attr in dir(self)
            if attr.startswith("op")
        }

    def reset(self) -> None:
        """Reset the registers."""
        for i in range(len(self.registers)):
            self.registers[i] = 0

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
