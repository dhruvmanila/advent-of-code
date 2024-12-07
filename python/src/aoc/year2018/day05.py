import string


def polymer_reaction(polymer: str) -> str:
    it = iter(polymer.encode("ascii"))
    stack = [next(it)]
    # Uppercase and lowercase letters in ASCII differs by only one bit at the
    # sixth position from right:
    #                             v
    #     f"{ord('a'):08b}" -> '01100001'
    #     f"{ord('A'):08b}" -> '01000001'
    shift = 1 << 5
    for unit in it:
        if stack and stack[-1] ^ shift == unit:
            stack.pop()
        else:
            stack.append(unit)
    return bytes(stack).decode()


def shortest_polymer(polymer: str) -> int:
    return min(
        len(polymer_reaction(polymer.replace(lower, "").replace(upper, "")))
        for lower, upper in zip(string.ascii_lowercase, string.ascii_uppercase)
    )


def solve(input: str) -> None:
    result = polymer_reaction(input)

    print(f"5.1: {len(result)}")
    print(f"5.2: {shortest_polymer(result)}")
