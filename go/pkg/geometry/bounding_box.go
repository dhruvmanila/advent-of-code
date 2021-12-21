package geometry

// BoundingBox contains information about coordinates of a rectangular border.
// This could represent the minimum and maximum value of X and Y coorindates in
// the coordinate system or minimum and maximum rows and columns for a matrix.
type BoundingBox struct {
	Minx int
	Maxx int
	Miny int
	Maxy int
}

// NewBoundingBox creates a new bounding box.
func NewBoundingBox(minx, maxx, miny, maxy int) *BoundingBox {
	return &BoundingBox{
		Minx: minx,
		Maxx: maxx,
		Miny: miny,
		Maxy: maxy,
	}
}

// Contains is used to check whether the point formed by given x and y values
// is within the bounding box.
func (bbox *BoundingBox) Contains(x, y int) bool {
	return bbox.Minx <= x && x <= bbox.Maxx && bbox.Miny <= y && y <= bbox.Maxy
}
