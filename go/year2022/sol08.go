package year2022

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/matrix"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

// forest represents a map with the height of each tree.
type forest struct {
	// trees is a matrix to store the tree height at (row, column).
	trees *matrix.Dense[byte]

	// scenicScore is a matrix to store the scenic score for the tree
	// at (row, column).
	scenicScore *matrix.Dense[int]

	// rows and columns are the total number of rows and columns in the grid.
	rows    int
	columns int
}

// newForest creates a new height map from the given lines.
func newForest(lines []string) *forest {
	rows, columns := len(lines), len(lines[0])

	height := make([]byte, 0, rows*columns)
	for _, line := range lines {
		height = append(height, []byte(line)...)
	}

	return &forest{
		trees: matrix.NewDense(rows, columns, height),
		// Initialize a 0-slice for adding the scenic score. As it's initialized
		// to 0, make sure to not use this value when computing the score otherwise
		// the value will always be 0.
		scenicScore: matrix.NewDense[int](rows, columns, nil),
		rows:        rows,
		columns:     columns,
	}
}

// topVisibility returns whether the tree at (row, col) of height is
// visible from the top side. It also returns the number of trees visible
// from (row, col) to the top side.
func (f *forest) topVisibility(row, col int, height byte) (bool, int) {
	for r := row - 1; r >= 0; r-- {
		if f.trees.At(r, col) >= height {
			return false, row - r
		}
	}
	return true, row
}

// leftVisibility returns whether the tree at (row, col) of height is
// visible from the left side. It also returns the number of trees visible
// from (row, col) to the left side.
func (f *forest) leftVisibility(row, col int, height byte) (bool, int) {
	for c := col - 1; c >= 0; c-- {
		if f.trees.At(row, c) >= height {
			return false, col - c
		}
	}
	return true, col
}

// bottomVisibility returns whether the tree at (row, col) of height is
// visible from the bottom side. It also returns the number of trees visible
// from (row, col) to the bottom side.
func (f *forest) bottomVisibility(row, col int, height byte) (bool, int) {
	for r := row + 1; r < f.rows; r++ {
		if f.trees.At(r, col) >= height {
			return false, r - row
		}
	}
	return true, f.rows - row - 1
}

// rightVisibility returns whether the tree at (row, col) of height is
// visible from the right side. It also returns the number of trees visible
// from (row, col) to the right side.
func (f *forest) rightVisibility(row, col int, height byte) (bool, int) {
	for c := col + 1; c < f.columns; c++ {
		if f.trees.At(row, c) >= height {
			return false, c - col
		}
	}
	return true, f.columns - col - 1
}

// VisibleCount returns the count of all the visible trees from either of
// the four direction. It'll also compute the scenic score for all the
// trees and store it.
func (f *forest) VisibleCount() int {
	// Initialize the count with all the trees visible on the edge.
	count := (f.rows * 2) + (f.columns-2)*2

	for r := 1; r < f.rows-1; r++ {
		for c := 1; c < f.columns-1; c++ {
			height := f.trees.At(r, c)

			isVisibleFromTop, viewingDistanceTop := f.topVisibility(r, c, height)
			isVisibleFromLeft, viewingDistanceLeft := f.leftVisibility(r, c, height)
			isVisibleFromBottom, viewingDistanceBottom := f.bottomVisibility(r, c, height)
			isVisibleFromRight, viewingDistanceRight := f.rightVisibility(r, c, height)

			f.scenicScore.Set(r, c, viewingDistanceTop*viewingDistanceLeft*viewingDistanceBottom*viewingDistanceRight)
			if isVisibleFromTop || isVisibleFromLeft || isVisibleFromBottom || isVisibleFromRight {
				count++
			}
		}
	}

	return count
}

// MaxScore returns the max scenic score for the grid. This assumes that
// the individual scores has already been computed. Use the `VisibleCount`
// method to compute the scores.
func (f *forest) MaxScore() int {
	maxScore := 0
	for _, score := range f.scenicScore.Data {
		maxScore = util.Max(maxScore, score)
	}
	return maxScore
}

func (f *forest) String() string {
	s := ""
	for _, row := range f.trees.Data {
		s += string(row) + "\n"
	}
	return s
}

func Sol08(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	f := newForest(lines)

	return fmt.Sprintf("8.1: %d\n8.2: %d\n", f.VisibleCount(), f.MaxScore()), nil
}
