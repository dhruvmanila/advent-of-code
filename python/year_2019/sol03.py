# https://adventofcode.com/2019/day/3

# I tried using sympy but that was way too slow and then I found this library
# which is based on GEOS which I have no idea what it is but it's really fast.
# One of the core reason is that shapely is in C while sympy is pure python.
from typing import Iterable, List, Tuple

from shapely.geometry import LineString, Point

WIRES_PATH = []

with open("input/03.txt") as inp:
    for line in inp:
        WIRES_PATH.append(line.strip().split(","))


# Helper function to generate the segments between points
def make_segments(wire_path: Iterable[str]) -> List[LineString]:
    # we will represent start and end as tuple because we just want the
    # segment object and not the intermediate point object
    start = (0, 0)
    segments = []
    for path in wire_path:
        shift = int(path[1:])
        if "R" in path:
            end = start[0] + shift, start[1]
        elif "L" in path:
            end = start[0] - shift, start[1]
        elif "U" in path:
            end = start[0], start[1] + shift
        else:
            end = start[0], start[1] - shift
        segments.append(LineString((start, end)))
        start = end
    return segments


# Keep this global to let both function have access to it
FIRST_WIRE_SEGMENTS = make_segments(WIRES_PATH[0])
SECOND_WIRE_SEGMENTS = make_segments(WIRES_PATH[1])
del WIRES_PATH  # declutter globals


# ------------------ FIRST HALF OF THE PUZZLE --------------------
def closest_distance() -> Tuple[int, Point, List[Point]]:
    # starting value 'inf' as we have to find the minimum distance
    min_distance = float("INF")
    min_intersection_pt = None
    intersection_points = []  # for second part of the puzzle
    # loop over all the possible combinations from first wire segments and
    # second wire segments
    for first_segment in FIRST_WIRE_SEGMENTS:
        for second_segment in SECOND_WIRE_SEGMENTS:
            if first_segment.intersects(second_segment):
                # Find the intersection point only if it intersects otherwise
                # its just a waste of computational time
                int_pt = first_segment.intersection(second_segment)
                intersection_points.append(int_pt)
                taxicab_dist = abs(int_pt.x) + abs(int_pt.y)
                # We don't want to include the 0.0 distance
                if taxicab_dist < min_distance and taxicab_dist:
                    min_distance = taxicab_dist
                    min_intersection_pt = int_pt
    return (
        int(min_distance),
        min_intersection_pt,
        intersection_points[1:],
    )  # first point is (0, 0)


distance, closest_pt, intersection_pts = closest_distance()
print("Shortest distance:", distance)
print("Closest intersection point:", closest_pt)


# --------------------- SECOND HALF OF THE PUZZLE ----------------------
def min_step_count() -> Tuple[int, Point]:
    min_steps = float("INF")
    min_intersection = None
    # loop through all the intersection points
    for point in intersection_pts:
        combined_steps = 0
        # find the step count for each wire
        for wire_segments in [FIRST_WIRE_SEGMENTS, SECOND_WIRE_SEGMENTS]:
            for segment in wire_segments:
                # if point is in segment then add the distance between the
                # starting point of the segment and the intersection point
                if point.within(segment):
                    # xy attribute gives you a two tuple containing array where
                    # the first array has all x coordinates and the second array
                    # has all the y coordinates
                    x1, y1 = segment.xy[0][0], segment.xy[1][0]
                    combined_steps += abs(x1 - point.x) + abs(y1 - point.y)
                    break  # we have reached the intersection point, so break
                # else just add the segment length
                else:
                    combined_steps += segment.length
        if combined_steps < min_steps:
            min_steps = combined_steps
            min_intersection = point
    return int(min_steps), min_intersection


steps, better_int = min_step_count()
print("\nMinimum combined steps:", steps)
print("Better intersection point:", better_int)
