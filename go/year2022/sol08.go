package year2022

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/matrix"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

// heightMap represents a map with the height of each tree.
type heightMap struct {
	// height is a matrix to store the tree height at (row, column).
	height *matrix.Dense[byte]

	// scenicScore is a matrix to store the scenic score for the tree
	// at (row, column).
	scenicScore *matrix.Dense[int]

	// rows and columns are the total number of rows and columns in the grid.
	rows    int
	columns int
}

// newHeightMap creates a new height map from the given lines.
func newHeightMap(lines []string) *heightMap {
	rows, columns := len(lines), len(lines[0])

	height := make([]byte, 0, rows*columns)
	for _, line := range lines {
		height = append(height, []byte(line)...)
	}

	return &heightMap{
		height: matrix.NewDense(rows, columns, height),
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
func (t *heightMap) topVisibility(row, col int, height byte) (bool, int) {
	for r := row - 1; r >= 0; r-- {
		if t.height.At(r, col) >= height {
			return false, row - r
		}
	}
	return true, row
}

// leftVisibility returns whether the tree at (row, col) of height is
// visible from the left side. It also returns the number of trees visible
// from (row, col) to the left side.
func (t *heightMap) leftVisibility(row, col int, height byte) (bool, int) {
	for c := col - 1; c >= 0; c-- {
		if t.height.At(row, c) >= height {
			return false, col - c
		}
	}
	return true, col
}

// bottomVisibility returns whether the tree at (row, col) of height is
// visible from the bottom side. It also returns the number of trees visible
// from (row, col) to the bottom side.
func (t *heightMap) bottomVisibility(row, col int, height byte) (bool, int) {
	for r := row + 1; r < t.rows; r++ {
		if t.height.At(r, col) >= height {
			return false, r - row
		}
	}
	return true, t.rows - row - 1
}

// rightVisibility returns whether the tree at (row, col) of height is
// visible from the right side. It also returns the number of trees visible
// from (row, col) to the right side.
func (t *heightMap) rightVisibility(row, col int, height byte) (bool, int) {
	for c := col + 1; c < t.columns; c++ {
		if t.height.At(row, c) >= height {
			return false, c - col
		}
	}
	return true, t.columns - col - 1
}

// VisibleCount returns the count of all the visible trees from either of
// the four direction. It'll also compute the scenic score for all the
// trees and store it.
func (t *heightMap) VisibleCount() int {
	// Initialize the count with all the trees visible on the edge.
	count := (t.rows * 2) + (t.columns-2)*2

	for r := 1; r < t.rows-1; r++ {
		for c := 1; c < t.columns-1; c++ {
			height := t.height.At(r, c)

			isVisibleFromTop, viewingDistanceTop := t.topVisibility(r, c, height)
			isVisibleFromLeft, viewingDistanceLeft := t.leftVisibility(r, c, height)
			isVisibleFromBottom, viewingDistanceBottom := t.bottomVisibility(r, c, height)
			isVisibleFromRight, viewingDistanceRight := t.rightVisibility(r, c, height)

			t.scenicScore.Set(r, c, viewingDistanceTop*viewingDistanceLeft*viewingDistanceBottom*viewingDistanceRight)
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
func (t *heightMap) MaxScore() int {
	maxScore := 0
	for _, score := range t.scenicScore.Data {
		maxScore = util.Max(maxScore, score)
	}
	return maxScore
}

func (t *heightMap) String() string {
	s := ""
	for _, row := range t.height.Data {
		s += string(row) + "\n"
	}
	return s
}

func Sol08(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	t := newHeightMap(lines)

	fmt.Printf("8.1: %d\n8.2: %d\n", t.VisibleCount(), t.MaxScore())
	return nil
}
