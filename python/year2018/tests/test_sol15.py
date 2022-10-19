import pytest

from year2018.sol15 import CaveCombat, compute_elves_win

MOVE_TEST_INPUT = """\
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########""".splitlines()

MOVE_TEST_OUTPUT = (
    # After 1 round
    """\
#########
#.G...G.#
#...G...#
#...E..G#
#.G.....#
#.......#
#G..G..G#
#.......#
#########""",
    # After 2 rounds
    """\
#########
#..G.G..#
#...G...#
#.G.E.G.#
#.......#
#G..G..G#
#.......#
#.......#
#########""",
    # After 3 rounds
    """\
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########""",
)


def test_movement():
    movetest = CaveCombat.from_lines(MOVE_TEST_INPUT)
    for expected in MOVE_TEST_OUTPUT:
        movetest.turn()
        assert str(movetest) == expected


COMBAT_TEST_INPUT = """\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######""".splitlines()

COMBAT_TEST_OUTPUT_ROUNDS = {
    1: {
        "map": """\
#######
#..G..#
#...EG#
#.#G#G#
#...#E#
#.....#
#######""",
        "units": [
            ("G", 200),
            ("E", 197),
            ("G", 197),
            ("G", 200),
            ("G", 197),
            ("E", 197),
        ],
    },
    2: {
        "map": """\
#######
#...G.#
#..GEG#
#.#.#G#
#...#E#
#.....#
#######""",
        "units": [
            ("G", 200),
            ("G", 200),
            ("E", 188),
            ("G", 194),
            ("G", 194),
            ("E", 194),
        ],
    },
    23: {
        "map": """\
#######
#...G.#
#..G.G#
#.#.#G#
#...#E#
#.....#
#######""",
        "units": [
            ("G", 200),
            ("G", 200),
            ("G", 131),
            ("G", 131),
            ("E", 131),
        ],
    },
    24: {
        "map": """\
#######
#..G..#
#...G.#
#.#G#G#
#...#E#
#.....#
#######""",
        "units": [
            ("G", 200),
            ("G", 131),
            ("G", 200),
            ("G", 128),
            ("E", 128),
        ],
    },
    25: {
        "map": """\
#######
#.G...#
#..G..#
#.#.#G#
#..G#E#
#.....#
#######""",
        "units": [
            ("G", 200),
            ("G", 131),
            ("G", 125),
            ("G", 200),
            ("E", 125),
        ],
    },
    26: {
        "map": """\
#######
#G....#
#.G...#
#.#.#G#
#...#E#
#..G..#
#######""",
        "units": [
            ("G", 200),
            ("G", 131),
            ("G", 122),
            ("E", 122),
            ("G", 200),
        ],
    },
    27: {
        "map": """\
#######
#G....#
#.G...#
#.#.#G#
#...#E#
#...G.#
#######""",
        "units": [
            ("G", 200),
            ("G", 131),
            ("G", 119),
            ("E", 119),
            ("G", 200),
        ],
    },
    28: {
        "map": """\
#######
#G....#
#.G...#
#.#.#G#
#...#E#
#....G#
#######""",
        "units": [
            ("G", 200),
            ("G", 131),
            ("G", 116),
            ("E", 113),
            ("G", 200),
        ],
    },
    47: {
        "map": """\
#######
#G....#
#.G...#
#.#.#G#
#...#.#
#....G#
#######""",
        "units": [
            ("G", 200),
            ("G", 131),
            ("G", 59),
            ("G", 200),
        ],
    },
}


def test_combat() -> None:
    combattest = CaveCombat.from_lines(COMBAT_TEST_INPUT)
    for round, expected in COMBAT_TEST_OUTPUT_ROUNDS.items():
        while combattest.round != round:
            combattest.turn()
        assert str(combattest) == expected["map"]
        assert [
            (unit.type.value, unit.hitpoints) for unit in sorted(combattest.units)
        ] == expected["units"]
    assert combattest.turn() == 27730


@pytest.mark.parametrize(
    "map, expected",
    (
        (
            """\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######""",
            36334,
        ),
        (
            """\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######""",
            39514,
        ),
        (
            """\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######""",
            27755,
        ),
        (
            """\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######""",
            28944,
        ),
        (
            """\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########""",
            18740,
        ),
    ),
)
def test_outcome(map: str, expected: int) -> None:
    combat = CaveCombat.from_lines(map.splitlines())
    assert combat.do_battle() == expected


@pytest.mark.parametrize(
    "map, expected",
    (
        (
            """\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######""",
            4988,
        ),
        (
            """\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######""",
            31284,
        ),
        (
            """\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######""",
            3478,
        ),
        (
            """\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######""",
            6474,
        ),
        (
            """\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########""",
            1140,
        ),
    ),
)
def test_compute_elves_win(map: str, expected: int) -> None:
    assert compute_elves_win(map.splitlines()) == expected
