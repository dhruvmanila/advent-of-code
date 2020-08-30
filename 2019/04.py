# https://adventofcode.com/2019/day/4

from itertools import combinations_with_replacement
from collections import Counter
from operator import eq, gt
from typing import Iterable, Callable

# Puzzle input
MINIMUM, MAXIMUM = 178416, 676461


# Helper function to check whether the number is in input range
def input_range_check(digit_list: Iterable[int]) -> bool:
    number = int(''.join(map(str, digit_list)))
    return MINIMUM < number < MAXIMUM


# This creates a list of all the possible combinations for the base conditions:
# A six digit number
# Digits are either increasing or repeating but never decreasing
# Number lies within the puzzle input range
POSSIBLE_COMBINATIONS = list(filter(input_range_check,
                                    combinations_with_replacement(range(1, 10), 6)))


# The only difference between the first part and the second part of the puzzle
# is the compare and number input. For first part, we need atleast one digit
# which is repeating one or more times. For the second part, we need atleast
# one digit which is repeating atmost two times.
def password_count(compare: Callable[[int, int], bool], number: int) -> int:
    poss_count = 0
    for num_dig in POSSIBLE_COMBINATIONS:
        count_dict = Counter(num_dig)
        for digit in count_dict:
            if compare(count_dict[digit], number):
                poss_count += 1
                break
    return poss_count


first, second = password_count(gt, 1), password_count(eq, 2)
print(f"First part: {first} \nSecond part: {second}")
