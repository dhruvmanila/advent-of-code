def fuel_requirement(mass: int) -> int:
    return (mass // 3) - 2


def recursive_fuel_calc(mass: int) -> int:
    total = 0
    requirement = fuel_requirement(mass)
    while requirement > 0:
        total += requirement
        requirement = fuel_requirement(requirement)
    return total


def solve(input: str) -> None:
    module_mass = list(map(int, input.splitlines()))

    print(f"1.1: {sum(map(fuel_requirement, module_mass))}")
    print(f"1.2: {sum(map(recursive_fuel_calc, module_mass))}")
