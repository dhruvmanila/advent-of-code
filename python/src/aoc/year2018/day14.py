from typing import Sequence


def recipe_chart(recipes: int) -> str:
    """Return the scores of the next ten recipes immediately after the given number of
    recipes.

    elf1 and elf2 are the position of the current recipe for the respective elf.
    """
    scores = [3, 7]
    elf1, elf2 = 0, 1
    target = recipes + 10
    while len(scores) < target:
        new_score = scores[elf1] + scores[elf2]
        if new_score >= 10:
            scores.append(new_score // 10)
        scores.append(new_score % 10)
        elf1 = (elf1 + scores[elf1] + 1) % len(scores)
        elf2 = (elf2 + scores[elf2] + 1) % len(scores)
    return "".join(map(str, scores[recipes:target]))


def recipe_chart_substring(target: Sequence[int]) -> int:
    scores = [3, 7]
    elf1, elf2 = 0, 1

    while True:
        new_score = scores[elf1] + scores[elf2]
        if new_score >= 10:
            scores.append(new_score // 10)
        scores.append(new_score % 10)

        # Update the elf positions
        elf1 = (elf1 + scores[elf1] + 1) % len(scores)
        elf2 = (elf2 + scores[elf2] + 1) % len(scores)

        if scores[-1 * len(target) :] == target:
            return len(scores) - len(target)
        elif new_score >= 10 and scores[-1 * len(target) - 1 : -1] == target:
            return len(scores) - len(target) - 1


def solve(input: str) -> None:
    print(f"14.1: {recipe_chart(int(input))}")
    print(f"14.2: {recipe_chart_substring(list(map(int, input)))}")
