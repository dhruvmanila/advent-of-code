package geom

import (
	"fmt"

	"golang.org/x/exp/constraints"
)

// Point2D represents a 2 dimensional point in the coordinate system.
type Point2D[T constraints.Integer] struct {
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

func (p Point2D[T]) String() string {
	return fmt.Sprintf("(%d, %d)", p.X, p.Y)
}

// Point3D represents a 3 dimensional point in the coordinate system.
type Point3D[T constraints.Integer] struct {
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

func (p Point3D[T]) String() string {
	return fmt.Sprintf("(%d, %d, %d)", p.X, p.Y, p.Z)
}
