SAMPLE_DATA = """\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"""

# If DEBUG = 1, use SAMPLE_DATA
DEBUG = 0


def parse_map_data():
    if DEBUG:
        map_data = SAMPLE_DATA
    else:
        with open("input/06.txt") as fd:
            map_data = fd.read()
    child_to_parent, parent_to_child = {}, {}
    for orbit in map_data.strip().split("\n"):
        center, orbiter = orbit.split(")")
        parent_to_child.setdefault(center, []).append(orbiter)
        child_to_parent[orbiter] = center
    return parent_to_child, child_to_parent


PARENT_TO_CHILD, CHILD_TO_PARENT = parse_map_data()


def count_orbits() -> int:
    cache: dict[str, int] = {}

    def loop(object_id: str) -> int:
        if object_id in cache:
            return cache[object_id]
        count = 0
        for orbiter_orbit in PARENT_TO_CHILD.get(object_id, []):
            count += 1 + loop(orbiter_orbit)
        cache[object_id] = count
        return count

    return sum(map(loop, PARENT_TO_CHILD))


def min_orbital_transfers() -> int:
    # Backtracking search
    visited: list[str] = ["YOU"]

    def get_children(node: str) -> list[str]:
        # A parent can have multiple children but a child only has one parent.
        children = []
        child = CHILD_TO_PARENT.get(node, "")
        if child and child not in visited:
            children.append(child)
        for parent in PARENT_TO_CHILD.get(node, []):
            if parent not in visited:
                children.append(parent)
        return children

    def visit(node: str, transfers: int = 0):
        for child in get_children(node):
            if child == "SAN":
                return transfers
            visited.append(child)
            # `total_transfers` is different than `transfers`, where the former is
            # an indication whether the current path lead to "SAN" or a dead end.
            # If `total_transfers` is 0, then the visit lead us to a dead end, so we
            # will visit the other children.
            if total_transfers := visit(child, transfers + 1):
                return total_transfers
        return 0

    return visit(CHILD_TO_PARENT["YOU"])


print("Total orbits =>", count_orbits())
print("Minimum orbital transfers =>", min_orbital_transfers())
