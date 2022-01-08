import contextlib
import sys
import time
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


def read(*, day: int, year: int, test: bool = False) -> str:
    """Read the input data from a file for the respective day and year.

    If test is `True`, read the test input file. Raises `FileNotFoundError`
    if the file does not exists.
    """
    datafile = Path(__file__).parent.parent.joinpath(
        f"year{year}",
        "input",
        "test" if test else "",
        f"{day:02}.txt",
    )
    if not datafile.exists():
        raise FileNotFoundError(datafile)
    return datafile.read_text().strip()
