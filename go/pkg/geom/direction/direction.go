package direction

import "github.com/dhruvmanila/advent-of-code/go/pkg/geom"

// Type is the direction type.
type Type int

const (
	Right Type = iota
	Down
	Left
	Up
)

var directionDelta = map[Type]geom.Point2D[int]{
	Right: {X: 1, Y: 0},
	Down:  {X: 0, Y: 1},
	Left:  {X: -1, Y: 0},
	Up:    {X: 0, Y: -1},
}

// Delta returns the difference in X and Y coordinates to move in the
// receiver direction.
func (d Type) Delta() geom.Point2D[int] {
	return directionDelta[d]
}

// Clockwise returns the direction after moving in the clockwise manner.
func (d Type) Clockwise() Type {
	switch d {
	case Right:
		return Down
	case Down:
		return Left
	case Left:
		return Up
	default:
		return Right
	}
}

// CounterClockwise returns the direction after moving in the counter
// clockwise manner.
func (d Type) CounterClockwise() Type {
	switch d {
	case Right:
		return Up
	case Down:
		return Right
	case Left:
		return Down
	default:
		return Left
	}
}

func (d Type) String() string {
	switch d {
	case Right:
		return "Right"
	case Down:
		return "Down"
	case Left:
		return "Left"
	default:
		return "Up"
	}
}
