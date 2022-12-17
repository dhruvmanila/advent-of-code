package year2022

import (
	"fmt"
	"math"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geom"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type sensor struct {
	// pos is the sensor position.
	pos geom.Point2D[int]
	// beacon is the position of the closest beacon this sensor can detect.
	beacon geom.Point2D[int]
	// distance is the manhattan distance between beacon and sensor position.
	distance int
}

func newSensor(position, beacon geom.Point2D[int]) *sensor {
	return &sensor{
		pos:      position,
		beacon:   beacon,
		distance: position.ManhattanDistance(beacon),
	}
}

// perimeterPoints returns the points at the perimeter of the sensor.
func (s *sensor) perimeterPoints() []geom.Point2D[int] {
	pdist := s.distance + 1
	points := make([]geom.Point2D[int], 0, pdist*4)
	for dx := 0; dx <= pdist; dx++ {
		dy := pdist - dx
		points = append(points,
			geom.Point2D[int]{X: s.pos.X + dx, Y: s.pos.Y + dy},
			geom.Point2D[int]{X: s.pos.X + dx, Y: s.pos.Y - dy},
			geom.Point2D[int]{X: s.pos.X - dx, Y: s.pos.Y + dy},
			geom.Point2D[int]{X: s.pos.X - dx, Y: s.pos.Y - dy},
		)
	}
	return points
}

func (s *sensor) String() string {
	return fmt.Sprintf("Sensor%s - %d - Beacon%s", s.pos, s.distance, s.beacon)
}

// coveredCountAt returns the number of points at y which are covered by
// the given sensors.
//
// The solution basically involves using the ranges of x to our advantage
// to get the minx and maxx or the range between which the coverage exists.
// This assumes that there are no disjoint ranges to create holes between
// the x range.
func coveredCountAt(sensors []*sensor, y int) int {
	minx, maxx := math.MaxInt, math.MinInt
	for _, s := range sensors {
		count := s.distance - util.Abs(s.pos.Y-y)
		minx = util.Min(s.pos.X-count, minx)
		maxx = util.Max(s.pos.X+count, maxx)
	}
	count := maxx - minx + 1
	seen := set.New[int]()
	for _, s := range sensors {
		if s.beacon.Y == y && !seen.Contains(s.beacon.X) {
			seen.Add(s.beacon.X)
			count--
		}
	}
	return count
}

// findDistressBeacon finds and returns the distress beacon position. It panics
// if unable to find it.
//
// The algorithm is to loop over every sensor's perimeter points and skip
// if it's out of bounds or covered by some other sensor.
func findDistressBeacon(sensors []*sensor, max int) geom.Point2D[int] {
	box := geom.NewBoundingBox2D(0, max, 0, max)
	for _, s := range sensors {
		for _, p := range s.perimeterPoints() {
			if !box.Contains(p.X, p.Y) {
				continue
			}
			found := true
			for _, other := range sensors {
				if p.ManhattanDistance(other.pos) <= other.distance {
					found = false
					break
				}
			}
			if found {
				return p
			}
		}
	}
	panic("unable to find the distress beacon")
}

func parseSensors(lines []string) ([]*sensor, error) {
	sensors := make([]*sensor, 0, len(lines))

	var sx, sy, bx, by int
	for idx, line := range lines {
		_, err := fmt.Sscanf(
			line, "Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d",
			&sx, &sy, &bx, &by,
		)
		if err != nil {
			return nil, fmt.Errorf("line %d: %q: %w", idx, line, err)
		}
		sensors = append(
			sensors,
			newSensor(geom.Point2D[int]{X: sx, Y: sy}, geom.Point2D[int]{X: bx, Y: by}),
		)
	}

	return sensors, nil
}

func Sol15(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	var y, max int
	if strings.Contains(input, "test") {
		y = 10
		max = 20
	} else {
		y = 2000000
		max = 4000000
	}

	sensors, err := parseSensors(lines)
	if err != nil {
		return "", err
	}

	distressBeacon := findDistressBeacon(sensors, max)
	tuningFrequency := distressBeacon.X*4000000 + distressBeacon.Y

	return fmt.Sprintf("15.1: %d\n15.2: %d\n", coveredCountAt(sensors, y), tuningFrequency), nil
}
