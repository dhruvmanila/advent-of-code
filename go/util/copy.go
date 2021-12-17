package util

// MatrixCopy is used to copy integer slice elements from source slice to a
// destination slice. Internally, this uses the built-in copy function to copy
// individual slice elements.
func MatrixCopy(dest, src [][]int) {
	for i, row := range src {
		dest[i] = make([]int, len(row))
		copy(dest[i], row)
	}
}
