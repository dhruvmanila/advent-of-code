package geom

import "github.com/dhruvmanila/advent-of-code/go/util"

// BoundingBox2D contains information about coordinates of a rectangular border.
// This could represent the minimum and maximum value of X and Y coorindates in
// the coordinate system or minimum and maximum rows and columns for a matrix.
type BoundingBox2D struct {
	MinX int
	MaxX int
	MinY int
	MaxY int
}

// NewBoundingBox2D creates a new two dimensional bounding box.
func NewBoundingBox2D(minx, maxx, miny, maxy int) *BoundingBox2D {
	return &BoundingBox2D{
		MinX: minx,
		MaxX: maxx,
		MinY: miny,
		MaxY: maxy,
	}
}

// Contains is used to check whether the point formed by given x and y values
// is within the bounds of the box.
func (b *BoundingBox2D) Contains(x, y int) bool {
	return b.MinX <= x && x <= b.MaxX && b.MinY <= y && y <= b.MaxY
}

// Intersection returns the bounding box formed by the intersection of bbox2d
// with other, nil if they do not intersect.
func (b *BoundingBox2D) Intersection(other *BoundingBox2D) *BoundingBox2D {
	minx := util.Max(b.MinX, other.MinX)
	maxx := util.Min(b.MaxX, other.MaxX)
	miny := util.Max(b.MinY, other.MinY)
	maxy := util.Min(b.MaxY, other.MaxY)

	if minx <= maxx && miny <= maxy {
		return NewBoundingBox2D(minx, maxx, miny, maxy)
	}
	return nil
}

// Area returns the area of the bounding box.
func (b *BoundingBox2D) Area() int {
	return (b.MaxX - b.MinX + 1) * (b.MaxY - b.MinY + 1)
}

// BoundingBox3D is similar to BoundingBox2D, except this represents a three
// dimensional cuboid.
type BoundingBox3D struct {
	MinX int
	MaxX int
	MinY int
	MaxY int
	MinZ int
	MaxZ int
}

// NewBoundingBox3D creates a new three dimensional bounding box.
func NewBoundingBox3D(minx, maxx, miny, maxy, minz, maxz int) *BoundingBox3D {
	return &BoundingBox3D{
		MinX: minx,
		MaxX: maxx,
		MinY: miny,
		MaxY: maxy,
		MinZ: minz,
		MaxZ: maxz,
	}
}

// Contains is used to check whether the point formed by given x, y and z values
// is within the bounds of the box.
func (b *BoundingBox3D) Contains(x, y, z int) bool {
	return b.MinX <= x && x <= b.MaxX &&
		b.MinY <= y && y <= b.MaxY &&
		b.MinZ <= z && z <= b.MaxZ
}

// Intersection returns the bounding box formed by the intersection of bbox3d
// with other, nil if they do not intersect.
func (b *BoundingBox3D) Intersection(other *BoundingBox3D) *BoundingBox3D {
	minx := util.Max(b.MinX, other.MinX)
	maxx := util.Min(b.MaxX, other.MaxX)
	miny := util.Max(b.MinY, other.MinY)
	maxy := util.Min(b.MaxY, other.MaxY)
	minz := util.Max(b.MinZ, other.MinZ)
	maxz := util.Min(b.MaxZ, other.MaxZ)

	if minx <= maxx && miny <= maxy && minz <= maxz {
		return NewBoundingBox3D(minx, maxx, miny, maxy, minz, maxz)
	}
	return nil
}

// Volume returns the volume of the bounding box.
func (b *BoundingBox3D) Volume() int {
	return (b.MaxX - b.MinX + 1) * (b.MaxY - b.MinY + 1) * (b.MaxZ - b.MinZ + 1)
}
