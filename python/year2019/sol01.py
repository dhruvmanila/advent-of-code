import utils


def fuel_requirement(mass: int) -> int:
    return (mass // 3) - 2


def recursive_fuel_calc(mass: int) -> int:
    total = 0
    requirement = fuel_requirement(mass)
    while requirement > 0:
        total += requirement
        requirement = fuel_requirement(requirement)
    return total


if __name__ == "__main__":
    data = utils.read(day=1, year=2019, test=False)
    module_mass = list(map(int, data.splitlines()))

    print(f"Part 1: {sum(map(fuel_requirement, module_mass))}")
    print(f"Part 2: {sum(map(recursive_fuel_calc, module_mass))}")
