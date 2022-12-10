import utils

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    args = parser.parse_args()

    calories = sorted(
        (
            sum(int(line) for line in section.splitlines())
            for section in utils.read(day=1, year=2022, test=args.test).split("\n\n")
        ),
        reverse=True,
    )

    print(f"1.1: {calories[0]}")
    print(f"1.2: {sum(calories[:3])}")
