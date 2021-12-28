from pathlib import Path


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
