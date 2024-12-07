def solve(input: str) -> None:
    calories = sorted(
        (
            sum(int(line) for line in section.splitlines())
            for section in input.split("\n\n")
        ),
        reverse=True,
    )

    print(f"1.1: {calories[0]}")
    print(f"1.2: {sum(calories[:3])}")
