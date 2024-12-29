# Advent of Code (Python)

Solutions to the Advent of Code puzzles in the Python programming language.

[uv] is used to manage the project. Install the dependencies using [uv]:

```
uv sync
```

The project provides a command-line interface to run the solutions. Run it via [uv]:

```
uv run aoc --help
```

Or, using Python (requires manually creating the virtual environment, activating it and
installing the dependencies):

```
python -m aoc --help
```

By default, it'll run the solution for current date. Refer to the `--help` output for the
day and year it will run the solution for.

To run a solution from a specific year and day:
```
uv run aoc -y 2023 -d 15
```

The project is structured in a way where the solutions are stored in a separate module for
each year. They need to be imported into the final [solutions mapping](https://github.com/dhruvmanila/advent-of-code/blob/master/python/src/aoc/solution.py)
for the command-line runner to recognize.

The year specific module can optionally include a `sample/` directory which will contain the
sample input for each puzzle for that year. They can be used as the input to the solution by:

```
uv run aoc --sample
```

Otherwise, the runner will try to download the input from the website and cache it. This requires
the session token to be available at the following location:

```
~/.config/aoc/token
```

And, the cached puzzle inputs are stored in `~/.cache/aoc` which will have the following directory structure:

```
~/.cache/aoc
├── 2023
│   ├── 1.txt
│   ├── 2.txt
│   └── ...
└── 2024
    └── ...
```

[uv]: https://github.com/astral-sh/uv
