import utils

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-s", "--sample", action="store_true", help="use the sample input"
    )
    args = parser.parse_args()

    calories = sorted(
        (
            sum(int(line) for line in section.splitlines())
            for section in utils.get_puzzle_input(
                day=1, year=2022, sample=args.sample
            ).split("\n\n")
        ),
        reverse=True,
    )

    print(f"1.1: {calories[0]}")
    print(f"1.2: {sum(calories[:3])}")
