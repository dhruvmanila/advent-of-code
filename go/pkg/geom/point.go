package geom

import (
	"fmt"

	"golang.org/x/exp/constraints"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

// Directions2D is an array of points corresponding to the difference to
// move in a certain direction in 2D. The order of the points is clockwise
// starting from up, i.e., UP, RIGHT, DOWN, LEFT.
var Directions2D = [4]Point2D[int]{
	{0, -1}, // UP
	{1, 0},  // RIGHT
	{0, 1},  // DOWN
	{-1, 0}, // LEFT
}

// Directions3D is an array of points corresponding to the difference to
// move in a certain direction in 3D. The order of the points is clockwise
// starting from up, and then front and back (z-axis), i.e., UP, RIGHT, DOWN,
// LEFT, FRONT, BACK.
var Directions3D = [6]Point3D[int]{
	{0, -1, 0}, // UP
	{1, 0, 0},  // RIGHT
	{0, 1, 0},  // DOWN
	{-1, 0, 0}, // LEFT
	{0, 0, 1},  // FRONT
	{0, 0, -1}, // BACK
}

// Point2D represents a 2 dimensional point in the coordinate system.
type Point2D[T constraints.Signed] struct {
	X, Y T
}

// Add adds p to other, returning the new point.
func (p Point2D[T]) Add(other Point2D[T]) Point2D[T] {
	p.X += other.X
	p.Y += other.Y
	return p
}

// Sub subtract other from p, returning the new point.
func (p Point2D[T]) Sub(other Point2D[T]) Point2D[T] {
	p.X -= other.X
	p.Y -= other.Y
	return p
}

// Equal returns true if p and other are the same point.
func (p Point2D[T]) Equal(other Point2D[T]) bool {
	return p.X == other.X && p.Y == other.Y
}

// ManhattanDistance returns the manhattan distance between p and other.
func (p Point2D[T]) ManhattanDistance(other Point2D[T]) T {
	return util.Abs(p.X-other.X) + util.Abs(p.Y-other.Y)
}

// Neighbors returns the neighboring points for p. These are the 6 directions
// corresponding to +ve and -ve X, Y and Z axis.
func (p Point2D[T]) Neighbors() []Point2D[T] {
	neighbors := make([]Point2D[T], 0, len(Directions2D))
	for _, direction := range Directions2D {
		neighbors = append(neighbors, Point2D[T]{
			X: p.X + T(direction.X),
			Y: p.X + T(direction.Y),
		})
	}
	return neighbors
}

func (p Point2D[T]) String() string {
	return fmt.Sprintf("(%d, %d)", p.X, p.Y)
}

// Point3D represents a 3 dimensional point in the coordinate system.
type Point3D[T constraints.Signed] struct {
	X, Y, Z T
}

// Add adds p to other, returning the new point.
func (p Point3D[T]) Add(other Point3D[T]) Point3D[T] {
	p.X += other.X
	p.Y += other.Y
	p.Z += other.Z
	return p
}

// Sub subtract other from p, returning the new point.
func (p Point3D[T]) Sub(other Point3D[T]) Point3D[T] {
	p.X -= other.X
	p.Y -= other.Y
	p.Z -= other.Z
	return p
}

// Equal returns true if p and other are the same point.
func (p Point3D[T]) Equal(other Point3D[T]) bool {
	return p.X == other.X && p.Y == other.Y && p.Z == other.Z
}

// ManhattanDistance returns the manhattan distance between p and other.
func (p Point3D[T]) ManhattanDistance(other Point3D[T]) T {
	return util.Abs(p.X-other.X) + util.Abs(p.Y-other.Y) + util.Abs(p.Z-other.Z)
}

// Neighbors returns the neighboring points for p. These are the 6 directions
// corresponding to +ve and -ve X, Y and Z axis.
func (p Point3D[T]) Neighbors() []Point3D[T] {
	neighbors := make([]Point3D[T], 0, len(Directions3D))
	for _, direction := range Directions3D {
		neighbors = append(neighbors, Point3D[T]{
			X: p.X + T(direction.X),
			Y: p.Y + T(direction.Y),
			Z: p.Z + T(direction.Z),
		})
	}
	return neighbors
}

func (p Point3D[T]) String() string {
	return fmt.Sprintf("(%d, %d, %d)", p.X, p.Y, p.Z)
}
