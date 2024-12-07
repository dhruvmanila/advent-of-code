import pytest

from aoc.year2018.day20 import ProjectMap


@pytest.mark.parametrize(
    ("regex, expected"),
    (
        ("^WNE$", 3),
        ("^ENWWW(NEEE|SSE(EE|N))$", 10),
        ("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$", 18),
        ("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$", 23),
        ("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$", 31),
    ),
)
def test_furthest_room(regex: str, expected: int) -> None:
    furthest_room, _ = ProjectMap.from_regex(regex).furthest_room()
    assert furthest_room == expected


@pytest.mark.parametrize(
    ("regex, expected"),
    (
        (
            "^WNE$",
            """\
#####
#.|.#
#-###
#.|X#
#####""",
        ),
        (
            "^ENWWW(NEEE|SSE(EE|N))$",
            """\
#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########""",
        ),
        (
            "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$",
            """\
###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########""",
        ),
        (
            "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$",
            """\
#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############""",
        ),
        (
            "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$",
            """\
###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############""",
        ),
    ),
)
def test_str(regex: str, expected: str) -> None:
    assert str(ProjectMap.from_regex(regex)) == expected
