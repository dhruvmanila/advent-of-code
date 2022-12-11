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
// the 8 direction (adjacent and diagonal) or both head and tail are at
// the same position.
func isTouching(head, tail *geom.Point2D[int]) bool {
	return util.Abs(head.X-tail.X) <= 1 && util.Abs(head.Y-tail.Y) <= 1
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
			knots[0] = knots[0].Add(delta)

			for i := 0; i < n-1; i++ {
				head, tail := knots[i], &knots[i+1]
				if isTouching(&head, tail) {
					continue
				}
				tail.X += util.Signum(head.X - tail.X)
				tail.Y += util.Signum(head.Y - tail.Y)
			}

			seen.Add(knots[n-1])
		}
	}

	return seen.Len()
}

func Sol09(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	motions := parseMotions(lines)

	return fmt.Sprintf(
		"9.1: %d\n9.2: %d\n",
		simulateMotions(motions, 2),
		simulateMotions(motions, 10),
	), nil
}
