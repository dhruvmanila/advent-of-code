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


def count_orbits() -> int:
    parsed_data, _ = parse_map_data()
    cache: dict[str, int] = {}

    def loop(object_id: str) -> int:
        if object_id in cache:
            return cache[object_id]
        count = 0
        for orbiter_orbit in parsed_data.get(object_id, []):
            count += 1 + loop(orbiter_orbit)
        cache[object_id] = count
        return count

    return sum(map(loop, parsed_data))


def min_orbital_transfers() -> int:
    # Complicated backtracking search
    parent_to_child, child_to_parent = parse_map_data()
    prev_parent = "YOU"
    count = 0

    def get_children(node):
        # A parent can have multiple children but a child only has one parent.
        children = []
        if node in child_to_parent:
            child = child_to_parent[node]
            if child != prev_parent:
                children.append(child)
        for parent in parent_to_child.get(node, []):
            if parent != prev_parent:
                children.append(parent)
        return children

    def visit(node):
        nonlocal count, prev_parent
        for child in get_children(node):
            prev_parent = node
            if child == "SAN":
                print("Minimum orbital transfers =>", count)
                return
            count += 1
            visit(child)
            # Above visit ended with a dead end or printed the actual answer, so reset
            # the prev_parent and count value, we're backtracking!
            prev_parent = node
            count -= 1

    return visit(child_to_parent[prev_parent])


print("Total orbits =>", count_orbits())
min_orbital_transfers()
