from collections import Counter
from itertools import combinations_with_replacement
from typing import Callable, Iterable

Checker = Callable[[int], bool]
CountValidator = Callable[[int], bool]


def generate_passwords() -> Iterable[int]:
    """Generate all the possible combinations of password as per the given conditions.

    Conditions:
        * Six digit number.
        * Digits are either increasing or repeating but never decreasing.
    """
    for digits in combinations_with_replacement(range(1, 10), 6):
        yield int("".join(map(str, digits)))


def password_checker_factory(
    lo: int, hi: int, count_validator: CountValidator
) -> Checker:
    """Factory function to create a password checker as per the given input."""

    def is_valid(password: int) -> bool:
        return lo < password < hi and any(
            count_validator(count) for _, count in Counter(str(password)).items()
        )

    return is_valid


def valid_count(checker: Checker) -> int:
    count = 0
    for password in generate_passwords():
        if checker(password):
            count += 1
    return count


def solve(input: str) -> None:
    lo, hi = map(int, input.split("-"))

    # For part one, we need atleast one digit which is repeating atleast two times.
    count1 = valid_count(password_checker_factory(lo, hi, lambda c: c >= 2))

    # For part two, we need atleast one digit which is repeating atmost two times.
    count2 = valid_count(password_checker_factory(lo, hi, lambda c: c == 2))

    print(f"4.1: {count1}")
    print(f"4.2: {count2}")
