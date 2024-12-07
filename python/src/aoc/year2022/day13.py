import ast
import functools
import itertools
import math
from collections.abc import Sequence
from enum import Enum

PacketItem = int | Sequence["PacketItem"]


class Order(int, Enum):
    LESS = -1
    EQUAL = 0
    MORE = 1


def pair_order(lhs: PacketItem, rhs: PacketItem) -> Order:
    match (lhs, rhs):
        case (list(left), list(right)):
            for a, b in itertools.zip_longest(left, right):
                if a is None:
                    return Order.LESS
                if b is None:
                    return Order.MORE
                match pair_order(a, b):
                    case Order.EQUAL:
                        continue
                    case order:
                        return order
        case (int(num), list(right)):
            return pair_order([num], right)
        case (list(left), int(num)):
            return pair_order(left, [num])
        case (int(lnum), int(rnum)):
            if lnum < rnum:
                return Order.LESS
            elif lnum > rnum:
                return Order.MORE
            else:
                return Order.EQUAL
        case _:
            raise ValueError(f"unexpected lhs '{lhs!r}' and rhs '{rhs!r}'")
    return Order.EQUAL


@functools.total_ordering
class Packet:
    def __init__(self, items: Sequence[PacketItem]) -> None:
        self.items = items

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Packet):
            return NotImplemented
        return pair_order(self.items, other.items) is Order.EQUAL

    def __lt__(self, other: object) -> bool:
        if not isinstance(other, Packet):
            return NotImplemented
        return pair_order(self.items, other.items) is Order.LESS


def solve(input: str) -> None:
    packets: list[Packet] = []

    for pairs in input.split("\n\n"):
        left, _, right = pairs.partition("\n")
        packets.append(Packet(ast.literal_eval(left)))
        packets.append(Packet(ast.literal_eval(right)))

    ordered_idx_sum = 0
    for pair_idx, packet_idx in enumerate(range(0, len(packets), 2), start=1):
        lhs, rhs = (
            packets[packet_idx].items,
            packets[packet_idx + 1].items,
        )
        if pair_order(lhs, rhs) is Order.LESS:
            ordered_idx_sum += pair_idx

    divider_packets = [Packet([[2]]), Packet([[6]])]
    packets.extend(divider_packets)
    packets.sort()
    decoder_key = math.prod(packets.index(packet) + 1 for packet in divider_packets)

    print(f"13.1: {ordered_idx_sum}")
    print(f"13.2: {decoder_key}")
