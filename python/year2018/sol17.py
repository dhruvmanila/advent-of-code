import re
import sys
from collections import defaultdict
from collections.abc import Iterable, MutableSequence
from enum import Enum, auto
from typing import NamedTuple

import utils

_parse_line = re.compile(
    r"^(?P<constvar>[xy])=(?P<const>\d+), [xy]=(?P<start>\d+)\.\.(?P<stop>\d+)$"
).search


class ParserError(Exception):
    """Exception raised when parsing fails."""

    def __init__(self, idx: int, line: str) -> None:
        super().__init__(f"Failed to parse line {idx}: {line!r}")


class Point(NamedTuple):
    x: int
    y: int


class State(Enum):
    FLOWING = auto()
    FILLING = auto()


class Scanline:
    """Scanline is used to scan the grid and simulate the flow of water."""

    def __init__(self, grid: list[list[str]], tap: int) -> None:
        self.y = 0
        self.grid = grid

        self._max_y = len(grid) - 1

        # Current state of the scanline and accordingly the direction of the scanline.
        # Flowing state is always downwards and filling state is always upwards.
        self._state = State.FLOWING
        self._direction = 1

        # Current streams are the streams that are flowing or filling in the
        # current scanline.
        self._current_streams: set[int] = {tap}

        # Paused streams are the streams that are paused due to a change in state
        # to filling some streams in the current scanline. As there could be multiple
        # streams at multiple y positions, a dictionary is used to map the y position
        # to all the paused streams at that position.
        self._paused_streams: dict[int, set[int]] = defaultdict(set)

    @property
    def view(self) -> MutableSequence[str]:
        """Return the view of the current scanline. Updating the view will be
        reflected in the grid."""
        return self.grid[self.y]

    def set_state(self, state: State) -> None:
        """Set the state and accordingly the direction of the scanline."""
        self._state = state
        match state:
            case State.FLOWING:
                self._direction = 1
            case State.FILLING:
                self._direction = -1

    def is_filled(self, stream: int, y: int | None = None) -> bool:
        """Return True if the stream is filled with water or clay. If y is None,
        the current y position is used."""
        if y is None:
            y = self.y
        return self.grid[y][stream] in "#~"

    def is_clay(self, stream: int) -> bool:
        """Return True if the stream in the current scanline is clay, False otherwise."""
        return self.view[stream] == "#"

    def get_filling_range(self, stream: int) -> tuple[range, str]:
        """Return the filling range of the stream and the character that is used to
        fill the stream.

        If the character is ~, the stream can be filled with water, otherwise there
        are unfilled sand where the stream will overflow into.
        """
        fill_type = "~"
        left = stream
        while not self.is_clay(left):
            if not self.is_filled(left, self.y + 1):
                fill_type = "|"
                left -= 1
                break
            left -= 1
        right = stream
        while not self.is_clay(right):
            if not self.is_filled(right, self.y + 1):
                fill_type = "|"
                right += 1
                break
            right += 1
        return range(left + 1, right), fill_type

    def update(self) -> None:
        """Update the scanline based on the current state."""
        match self._state:
            case State.FLOWING:
                done_streams = {
                    stream for stream in self._current_streams if self.is_filled(stream)
                }
                if done_streams:
                    # If there are streams that are filled, pause the other streams
                    # and start filling the done streams.
                    paused_streams = self._current_streams - done_streams
                    if paused_streams:
                        self._paused_streams[self.y] |= paused_streams
                    self._current_streams = done_streams
                    self.set_state(State.FILLING)
                else:
                    # If there are no streams that are filled, resume the paused streams
                    # for the current scanline and update the grid with flowing water.
                    paused_streams = self._paused_streams.pop(self.y, None)  # type: ignore
                    if paused_streams is not None:
                        self._current_streams |= paused_streams
                    for stream in self._current_streams:
                        self.view[stream] = "|"

            case State.FILLING:
                for stream in tuple(self._current_streams):
                    # If multiple streams are filling the same clay block, let's not
                    # update the scanline multiple times.
                    if self.is_filled(stream):
                        continue

                    filling_range, fill_type = self.get_filling_range(stream)
                    if fill_type == "|":
                        self._current_streams.remove(stream)

                        # Add the overflowing streams from either side if they are not
                        # filled with water or clay.
                        if not self.is_filled(filling_range[0], self.y + 1):
                            self._current_streams.add(filling_range[0])
                        if not self.is_filled(filling_range[-1], self.y + 1):
                            self._current_streams.add(filling_range[-1])

                        self.set_state(State.FLOWING)

                    for stream in filling_range:
                        self.view[stream] = fill_type

    def move(self) -> None:
        """Move the scanline to the next position based on the current state."""
        self.y += self._direction

    def done(self) -> bool:
        """Return True if the scanline has reached the end of the grid, False otherwise."""
        return self.y > self._max_y


class Reservoir:
    def __init__(self, grid: list[list[str]], tap: int) -> None:
        """Initialize the reservoir with the grid and the tap position.

        Args:
            grid: The grid of the reservoir.
            tap: The tap position. This is the x position as the tap is always at the
                top of the grid.
        """
        self.grid = grid
        self.tap = tap
        self.scanline = Scanline(grid, tap=tap)
        self._first_y = next(y for y, row in enumerate(grid) if "#" in row)

    @classmethod
    def from_lines(cls, lines: Iterable[str]) -> "Reservoir":
        max_y = 0
        min_x, max_x = sys.maxsize, 0

        points: set[Point] = set()
        for idx, line in enumerate(lines, start=1):
            m = _parse_line(line)
            if m is None:
                raise ParserError(idx, line)
            const, start, end = map(int, (m["const"], m["start"], m["stop"]))
            for n in range(start, end + 1):
                point = Point(const, n) if m["constvar"] == "x" else Point(n, const)
                points.add(point)
                max_y = max(max_y, point.y)
                min_x, max_x = min(min_x, point.x - 1), max(max_x, point.x + 1)

        grid = [["."] * (max_x - min_x + 1) for _ in range(max_y + 1)]
        for point in points:
            grid[point.y][point.x - min_x] = "#"
        return cls(grid, tap=(500 - min_x))

    def fill(self) -> None:
        while not self.scanline.done():
            self.scanline.update()
            self.scanline.move()

    def water_count(self) -> tuple[int, int]:
        """Return the count of standing water and flowing water in the reservoir."""
        s = "".join("".join(row) for row in self.grid[self._first_y :])
        return s.count("~"), s.count("|")

    def __str__(self) -> str:
        return "\n".join("".join(row) for row in self.grid)


def generate_image(reservoir: Reservoir, scale: int = 1) -> None:
    from PIL import Image

    width = len(reservoir.grid[0])
    height = len(reservoir.grid)
    colors = {
        "#": (0x8B, 0x45, 0x13),
        "~": (0x41, 0x69, 0xE1),
        "|": (0xB0, 0xE0, 0xE6),
    }

    img = Image.new("RGB", size=(width, height), color=(0xDE, 0xBE, 0x87))
    for y, row in enumerate(reservoir.grid):
        for x, char in enumerate(row):
            if char in colors:
                img.putpixel((x, y), colors[char])
    img.putpixel((reservoir.tap, 0), (0, 0, 0x80))

    if scale != 1:
        img = img.resize(
            (width * scale, height * scale), resample=Image.Resampling.NEAREST
        )

    img.save("reservoir.png")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument("-t", "--test", action="store_true", help="use the test input")
    parser.add_argument(
        "-i", "--image", action="store_true", help="generate an image (reservoir.png)"
    )
    parser.add_argument("--scale", type=int, default=1, help="image scale")
    args = parser.parse_args()

    lines = utils.read(year=2018, day=17, test=args.test).splitlines()
    reservoir = Reservoir.from_lines(lines)
    reservoir.fill()

    standing_water, flowing_water = reservoir.water_count()
    print(f"Part 1: {standing_water + flowing_water}")
    print(f"Part 2: {standing_water}")

    if args.image:
        generate_image(reservoir, scale=args.scale)
