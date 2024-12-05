package year2022

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geom"
	"github.com/dhruvmanila/advent-of-code/go/pkg/iterator"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var possibleDirections = []geom.Point2D[int]{
	{X: 0, Y: 1},  // down one step
	{X: -1, Y: 1}, // one step down and to the left
	{X: 1, Y: 1},  // one step down and to the right
}

type cave struct {
	rocks set.Set[geom.Point2D[int]]
	maxy  int
}

func (c *cave) simulateSand(withFloor bool) int {
	sands := set.New[geom.Point2D[int]]()
	directions := iterator.New(possibleDirections)

	// Create a local copy to avoid updating the original values.
	maxy := c.maxy
	if withFloor {
		// The floor is 2 plus the maxy, but the comparison of the next sand
		// Y position is done by > operator.
		maxy += 1
	}

	// sandPath is a stack to keep track of the current sand path from the
	// source position. This will only contain the falling sand position and
	// not the ones which has come to rest. This means that the next sand
	// will start from the last known falling position and not to simulate
	// the same path again and again. This works as each sand follows the
	// exact same path as the last one minus the last step.
	sandPath := stack.New(geom.Point2D[int]{X: 500, Y: 0})

MainLoop:
	for {
		// If there's a floor at the bottom, then we need to stop when the
		// source itself gets blocked and stops the flow of sand into the cave.
		// The stack is empty, meaning the source position was poped.
		if withFloor && sandPath.IsEmpty() {
			break
		}
		for directions.Next() {
			direction := directions.Value()
			for {
				// This will be the last position of the previous sand grain
				// minus the last step.
				sand, _ := sandPath.Peek()
				nextSand := sand.Add(direction)
				if nextSand.Y > maxy {
					if withFloor {
						break
					} else {
						break MainLoop
					}
				}
				// If the next sand position is already occupied, change
				// the direction.
				if c.rocks.Contains(nextSand) || sands.Contains(nextSand) {
					break
				}
				// Move the sand and reset the direction to start falling
				// down again. Add the next position of the sand in the path.
				sandPath.Push(nextSand)
				directions.Reset()
				break
			}
		}
		// The sand can't move in any of the possible directions now.
		// The next sand position is blocked, so the position we get here
		// is the one where the sand will come at rest. This means that the
		// last value in the stack is the starting position for the next sand.
		sand, _ := sandPath.Pop()
		sands.Add(sand)
		directions.Reset()
	}

	return sands.Len()
}

func parseInput(lines []string) (*cave, error) {
	rocks := set.New[geom.Point2D[int]]()
	maxy := 0

	// These points form the line of rock from (fx, fy) through (tx, ty).
	var fx, fy, tx, ty int

	for _, line := range lines {
		coordinates := strings.Split(line, " -> ")

		// Get the first coordinate out and then loop from the second
		// coordinate onwards. This is to make sure we have the start
		// and end point of the line.
		_, err := fmt.Sscanf(coordinates[0], "%d,%d", &fx, &fy)
		if err != nil {
			return nil, err
		}

		// Add the first point in the set. This will be skipped when looping
		// through the point range.
		rocks.Add(geom.Point2D[int]{X: fx, Y: fy})

		for _, coordinate := range coordinates[1:] {
			_, err := fmt.Sscanf(coordinate, "%d,%d", &tx, &ty)
			if err != nil {
				return nil, err
			}

			switch {
			case fx == tx: // vertical line
				dy := util.Signum(ty - fy)
				for y := fy + dy; y != ty+dy; y += dy {
					rocks.Add(geom.Point2D[int]{X: fx, Y: y})
				}
			case fy == ty: // horizontal line
				dx := util.Signum(tx - fx)
				for x := fx + dx; x != tx+dx; x += dx {
					rocks.Add(geom.Point2D[int]{X: x, Y: fy})
				}
			}

			maxy = util.Max(maxy, ty)
			fx, fy = tx, ty
		}
	}

	return &cave{rocks: rocks, maxy: maxy}, nil
}

func Sol14(input string) (string, error) {
	lines := util.ReadLines(input)

	cave, err := parseInput(lines)
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("14.1: %d\n14.2: %d\n", cave.simulateSand(false), cave.simulateSand(true)), nil
}
