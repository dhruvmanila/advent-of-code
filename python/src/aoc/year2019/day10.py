from math import atan2, pi, sqrt

ASTEROIDS_LOCATION: list[tuple[int, int]] = []


def parse_data(data: list[str]) -> None:
    for y, row in enumerate(data):
        for x, obj in enumerate(row):
            if obj == ".":
                continue
            ASTEROIDS_LOCATION.append((x, y))


def angle(x1: int, y1: int, x2: int, y2: int) -> float:
    """Return the angle of two points in terms of the positive Y-axis."""
    a = atan2(y2 - y1, x2 - x1)
    if a < 0.0 and a < (-pi / 2):
        a += (5 * pi) / 2
    else:
        a += pi / 2
    return a


def distance(x1: int, y1: int, x2: int, y2: int) -> float:
    """Return the distance between the two given points."""
    return sqrt(((y2 - y1) ** 2) + ((x2 - x1) ** 2))


def max_asteroids_detected() -> tuple[tuple[int, int], int]:
    """Return a tuple where the first element is the best location for the monitoring
    station and the second element is the number of asteroids in the line of sight
    from the station.

    Algorithm:
        Loop over each asteroid making it the origin and count the number of asteroids
        visible from that station. The counting is done by calculating the arctangent
        which is basically the angle where the tangent of the angle equals opposite by
        adjacent distance.

        tan(θ) = y/x  (y = opposite distance, x = adjacent distance)
        Thus, arctan(y/x) = θ

        By keeping the angles in a set, we will remove all the duplicates meaning we
        will remove all the asteroids which are not in our line of sight. The length
        of the set will become the number of asteroids visible from that station.
    """
    count_data: dict[tuple[int, int], int] = {}
    for index, origin in enumerate(ASTEROIDS_LOCATION):
        angles: set[float] = set()
        for other in ASTEROIDS_LOCATION[:index] + ASTEROIDS_LOCATION[index + 1 :]:
            angles.add(angle(*origin, *other))
        count_data[origin] = len(angles)
    max_count_point = max(count_data, key=count_data.get)
    return max_count_point, count_data[max_count_point]


def get_sorted_points_from_station(
    station_location: tuple[int, int],
) -> dict[tuple[int, int], float]:
    """Return a sorted dictionary where the keys are the points other than the
    station and the values are the angles with respect to the positive Y-axis.

    The ties are broken by calculating the distance between the points. The closest
    asteroid will be vaporized first.
    """
    location_to_angle = {}
    index = ASTEROIDS_LOCATION.index(station_location)
    for other in ASTEROIDS_LOCATION[:index] + ASTEROIDS_LOCATION[index + 1 :]:
        curr_angle = angle(*station_location, *other)
        curr_distance = distance(*station_location, *other)
        location_to_angle[other] = curr_angle, curr_distance
    return {
        k: location_to_angle[k][0]
        for k in sorted(location_to_angle, key=location_to_angle.get)
    }


def asteroid_200(sorted_points: dict[tuple[int, int], float]) -> tuple[int, int]:
    """Return the 200th asteroid which will be vaporized by the laser when the laser
    starts from the top and rotates clockwise.

    As we now have the sorted dictionary of angles as per the distance of the
    asteroid, we just need to make sure the asteroids in the same line will be handled
    appropriately.

    Recursion will be used for the next rotation. If the angle of the current asteroid
    is the same as that of the previous asteroid, the item will be added in the
    dictionary which will be used to make the recursive call. Once we reach the 200th
    asteroid, we will return the location of that asteroid.

    There's a cheeky way to do this without recursion: As we know that the maximum
    number of asteroids visible in our line of sight from the station is 211 and we
    need the 200th asteroid, we can just do a normal loop, skip the same angle
    asteroids and return the 200th one without a need to make a second rotation. But,
    ofcourse this is assuming the number of asteroids visible is greater than 200.
    """
    items = sorted_points.copy()

    def loop(curr_items, count, prev_angle):
        next_items = {}
        for point, curr_angle in curr_items.items():
            if count == 200:
                return point
            if curr_angle == prev_angle:
                next_items[point] = curr_angle
                continue
            prev_angle = curr_angle
            count += 1
        if next_items:
            loop(next_items, count, prev_angle)

    return loop(items, 1, None)


def solve(input: str) -> None:
    parse_data(input.strip().splitlines())

    point, count = max_asteroids_detected()
    print(f"Maximum number of asteroids visible for {point} => {count}")

    sorted_angles = get_sorted_points_from_station(point)
    x, y = asteroid_200(sorted_angles)
    print(f"200th asteroid is at {(x, y)} => {x * 100 + y}")
