try:
    from intcode import IntcodeComputer
except ImportError:
    from .intcode import IntcodeComputer


COMPONENTS = {0: " ", 1: "|", 2: chr(9604), 3: chr(9620), 4: "o"}
RENDER_GAME = True
FRAME_RATE = 0.0005


def count_block_tiles(program):
    computer = IntcodeComputer(program, return_output=True)
    count = 0
    while True:
        computer.run()
        computer.run()
        tile_id = computer.run()
        if computer.halted():
            break
        if tile_id == 2:
            count += 1
    return count


# Custom rendering engine :)
# Sort the tiles and paint (print) the screen from left to right, top to bottom.
# Once at the end reset the cursor back to the original position.
# Do ``reset_cursor=False`` for the last frame where the cursor should be below
# the render.
#
#  def render_game(game_tiles, reset_cursor=True):
#      tiles = {k: game_tiles[k] for k in sorted(game_tiles, key=lambda elem: elem[1])}
#      prev_y = None
#      newlines = 0
#      for coord, tile_id in tiles.items():
#          curr_y = coord[1]
#          if prev_y != curr_y:
#              print()
#              newlines += 1
#          prev_y = curr_y
#          print(COMPONENTS[tile_id], end="")
#      if reset_cursor:
#          print("\033[F" * newlines, end="")
#      else:
#          print("\n")


def ball_and_paddle_x(game_tiles):
    values = list(game_tiles.values())
    keys = list(game_tiles.keys())
    return keys[values.index(4)][0], keys[values.index(3)][0]


def play(screen, program):
    if RENDER_GAME:
        curses.curs_set(0)
        y_max = 0
    computer = IntcodeComputer(program, return_output=True, return_before_input=True)
    current_score = 0
    game_tiles = {}
    while True:
        output = computer.run()
        if output is computer.sentinel_return:
            bx, px = ball_and_paddle_x(game_tiles)
            computer.append_inputs(-1 if bx < px else 1 if bx > px else 0)
            x = computer.run()
        else:
            x = output
        y = computer.run()
        tile_id = computer.run()
        if computer.halted():
            break
        if x == -1 and y == 0:
            current_score = tile_id
            if RENDER_GAME:
                screen.addstr(y_max + 2, 0, f"Score => {current_score}")
                screen.refresh()
        else:
            game_tiles[(x, y)] = tile_id
            if RENDER_GAME:
                y_max = max(y_max, y)
                screen.addstr(y, x, COMPONENTS[tile_id])
                screen.refresh()
                sleep(FRAME_RATE)
    return current_score


if __name__ == "__main__":
    from pathlib import Path

    intcode_program = list(
        map(
            int,
            Path(__file__)
            .parent.joinpath("input", "13.txt")
            .read_text()
            .strip()
            .split(","),
        )
    )

    print(f"Number of block tiles => {count_block_tiles(intcode_program)}")

    # Adding a quarter to the program
    intcode_program[0] = 2
    if RENDER_GAME:
        import curses
        from time import sleep

        score = curses.wrapper(play, intcode_program)
    else:
        score = play(object(), intcode_program)
    print(f"Score after playing the game => {score}")
