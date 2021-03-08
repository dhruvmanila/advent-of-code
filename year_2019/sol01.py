# https://adventofcode.com/2019/day/1

MASS_LIST = []

with open("input/01.txt") as inp:
    for line in inp:
        data = int(line.strip())
        MASS_LIST.append(data)


def solution1_1():
    total_fuel = 0
    for mass in MASS_LIST:
        total_fuel += (mass // 3) - 2
    return total_fuel


print(f"{solution1_1() = }")


def solution1_2():
    total_fuel = 0
    for mass in MASS_LIST:
        fuel = 0
        while mass > 0:
            mass = (mass // 3) - 2
            fuel += mass
        total_fuel += fuel + abs(mass)
    return total_fuel


print(f"{solution1_2() = }")
