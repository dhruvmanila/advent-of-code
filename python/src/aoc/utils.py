import contextlib
import os
import sys
import time
import urllib.request
from pathlib import Path
from typing import Generator


@contextlib.contextmanager
def timing(name: str = "") -> Generator[None, None, None]:
    before = time.time()
    try:
        yield
    finally:
        after = time.time()
        t = (after - before) * 1000
        unit = "ms"
        if t < 100:
            t *= 1000
            unit = "μs"
        if name:
            name = f" ({name})"
        print(f"> {int(t)} {unit}{name}", file=sys.stderr, flush=True)


def supports_unicode() -> bool:
    """Return `True` if the terminal supports unicode."""
    return sys.stdout.encoding.lower().startswith("utf")


def get_puzzle_input(*, year: int, day: int, sample: bool = False) -> str:
    """Return the puzzle input for the given year and day.

    If `sample` is `True`, return the sample input instead. The sample input should be
    stored in a `sample/` directory in the respective year's directory.
    """
    if sample:
        return get_sample_input(year, day)

    cache_path = _get_cache_path(year, day)
    if os.path.exists(cache_path):
        with open(cache_path, "r") as f:
            return f.read()

    token = _read_session_token()
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    request = urllib.request.Request(url, headers={"Cookie": f"session={token}"})

    with urllib.request.urlopen(request) as response:
        input_data = response.read().decode("utf-8")

    _ensure_dirs(cache_path)
    with open(cache_path, "w") as f:
        f.write(input_data)

    return input_data


def get_sample_input(year: int, day: int) -> str:
    """Read the sample input for the given year and day.

    Raises:
        FileNotFoundError: If the sample input file does not exist.
    """
    filepath = Path(__file__).parent.joinpath(f"year{year}", "sample", f"{day:02}.txt")
    if not filepath.exists():
        raise FileNotFoundError(filepath)
    return filepath.read_text().strip("\n")


def _get_cache_path(year: int, day: int) -> str:
    """Generate the cache file path for a specific puzzle input."""
    cache_dir = os.path.expanduser("~/.cache/aoc")
    return os.path.join(cache_dir, str(year), f"{day}.txt")


def _read_session_token() -> str:
    """Read the Advent of Code session token from the default location."""
    token_path = os.path.expanduser("~/.config/aoc/token")
    with open(token_path, "r") as f:
        return f.read().strip()


def _ensure_dirs(file_path: str) -> None:
    """Ensure the directory where the file path resides exists."""
    os.makedirs(os.path.dirname(file_path), exist_ok=True)
