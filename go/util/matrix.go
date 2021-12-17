package util

var cardinalDirection = [][]int{
	{0, -1},
	{1, 0},
	{0, 1},
	{-1, 0},
}

// CardinalDirection returns a list of coordinates in the four cardinal
// directions from the given point (x, y).
//
// The list contains only the coordinates which are within the matrix bounds as
// given by the rows and cols argument. The four cardinal directions are:
// North, South, East and West.
func CardinalDirection(y, x, rows, cols int) [][]int {
	// Initialize the slice with a capacity of 4 (possible directions).
	pos := make([][]int, 0, 4)
	for _, d := range cardinalDirection {
		// y -> row
		// x -> column
		r, c := y+d[1], x+d[0]
		if r < 0 || c < 0 || r >= rows || c >= cols {
			continue
		}
		pos = append(pos, []int{r, c})
	}
	return pos
}

// MatrixCopy is used to copy integer slice elements from source slice to a
// destination slice. Internally, this uses the built-in copy function to copy
// individual slice elements.
func MatrixCopy(dest, src [][]int) {
	for i, row := range src {
		dest[i] = make([]int, len(row))
		copy(dest[i], row)
	}
}
