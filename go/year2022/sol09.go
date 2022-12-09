package year2022

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geom"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var directionDelta = map[byte]geom.Point2D[int]{
	'U': {X: 0, Y: 1},
	'D': {X: 0, Y: -1},
	'L': {X: -1, Y: 0},
	'R': {X: 1, Y: 0},
}

type motion struct {
	direction byte
	steps     int
}

func parseMotions(lines []string) []*motion {
	m := make([]*motion, 0, len(lines))
	for _, line := range lines {
		m = append(m, &motion{
			direction: line[0],
			steps:     util.MustAtoi(line[2:]),
		})
	}
	return m
}

// isTouching returns true if tail is adjacent to the head in either of
// the 9 direction (adjacent and diagonal).
func isTouching(head, tail geom.Point2D[int]) bool {
	for dx := -1; dx <= 1; dx++ {
		for dy := -1; dy <= 1; dy++ {
			if tail.X+dx == head.X && tail.Y+dy == head.Y {
				return true
			}
		}
	}
	return false
}

// simulateMotions simulates the rope motions consisting of n knots.
func simulateMotions(motions []*motion, n int) int {
	// knots is a slice of n knots each initialized to origin (0, 0).
	knots := make([]geom.Point2D[int], n)

	// seen is a set of points seen by the tail knot of the rope.
	seen := set.New(knots[0])

	for _, m := range motions {
		delta := directionDelta[m.direction]

		for s := 0; s < m.steps; s++ {
			// Update the position of the head knot. The points are passed
			// by value so we can update them in place.
			knots[0].X += delta.X
			knots[0].Y += delta.Y

			for i := 0; i < n-1; i++ {
				head, tail := knots[i], knots[i+1]
				if isTouching(head, tail) {
					continue
				}

				var delta geom.Point2D[int]

				// Compute the delta for the current tail knot.
				switch {
				case head.X == tail.X:
					if head.Y > tail.Y {
						delta.Y = 1
					} else {
						delta.Y = -1
					}
				case head.Y == tail.Y:
					if head.X > tail.X {
						delta.X = 1
					} else {
						delta.X = -1
					}
				default:
					if head.X > tail.X {
						delta.X = 1
					} else {
						delta.X = -1
					}
					if head.Y > tail.Y {
						delta.Y = 1
					} else {
						delta.Y = -1
					}
				}

				// Update the position of the current tail knot. The points are
				// passed by value so we can update them in place.
				knots[i+1].X = tail.X + delta.X
				knots[i+1].Y = tail.Y + delta.Y
			}
			seen.Add(knots[n-1])
		}
	}

	return seen.Len()
}

func Sol09(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	motions := parseMotions(lines)

	fmt.Printf(
		"9.1: %d\n9.2: %d\n",
		simulateMotions(motions, 2),
		simulateMotions(motions, 10),
	)
	return nil
}
