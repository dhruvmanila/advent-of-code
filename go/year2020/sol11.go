package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

const (
	empty    byte = 'L'
	occupied byte = '#'
	floor    byte = '.'
)

var directions = [][2]int{
	{-1, -1},
	{-1, 0},
	{-1, 1},
	{0, -1},
	{0, 1},
	{1, -1},
	{1, 0},
	{1, 1},
}

type occupiedAroundFunc func(int, int) int

type seatLayout struct {
	grid [][]byte

	// rows and cols are the total number of rows and columns in the grid.
	rows int
	cols int
}

func newSeatLayout(grid [][]byte) *seatLayout {
	return &seatLayout{
		grid: grid,
		rows: len(grid),
		cols: len(grid[0]),
	}
}

// occupiedAroundV1 is used to get the count of occupied seats adjacent
// to the given seat for part 1.
func (sl *seatLayout) occupiedAroundV1(row, col int) int {
	count := 0
	for _, pos := range util.AllDirection(row, col, sl.rows, sl.cols) {
		if sl.grid[pos[0]][pos[1]] == occupied {
			count++
		}
	}
	return count
}

// occupiedAroundV2 is used to get the count of occupied seats adjacent
// to the given seat for part 2.
func (sl *seatLayout) occupiedAroundV2(row, col int) int {
	count := 0
NextDirection:
	for _, d := range directions {
		dy, dx := d[0], d[1]
		for mult := 1; true; mult++ {
			r, c := row+dy*mult, col+dx*mult
			if r < 0 || r >= sl.rows || c < 0 || c >= sl.cols {
				break
			}
			switch sl.grid[r][c] {
			case floor:
				continue
			case occupied:
				count++
				fallthrough
			case empty:
				continue NextDirection
			}
		}
	}
	return count
}

func (sl *seatLayout) apply(occupiedAround occupiedAroundFunc, limit int) bool {
	isUpdated := false
	newLayout := make([][]byte, len(sl.grid))
	for y, row := range sl.grid {
		newRow := make([]byte, len(row))
		copy(newRow, row)
		for x, seat := range row {
			switch seat {
			case empty:
				if occupiedAround(y, x) == 0 {
					newRow[x] = occupied
					isUpdated = true
				}
			case occupied:
				if occupiedAround(y, x) >= limit {
					newRow[x] = empty
					isUpdated = true
				}
			}
		}
		newLayout[y] = newRow
	}
	if isUpdated {
		sl.grid = newLayout
	}
	return isUpdated
}

func (sl *seatLayout) totalOccupied() int {
	count := 0
	for _, row := range sl.grid {
		for _, seat := range row {
			if seat == occupied {
				count++
			}
		}
	}
	return count
}

func (sl *seatLayout) String() string {
	var s string
	for _, row := range sl.grid {
		for _, seat := range row {
			switch seat {
			case empty:
				s += "L"
			case occupied:
				s += "#"
			case floor:
				s += "."
			}
		}
		s += "\n"
	}
	return s
}

func parseSeatLayout(lines []string) *seatLayout {
	layout := make([][]byte, len(lines))
	for i, line := range lines {
		layout[i] = []byte(line)
	}
	return newSeatLayout(layout)
}

func Sol11(input string) (string, error) {
	lines := util.ReadLines(input)

	layout := parseSeatLayout(lines)
	for layout.apply(layout.occupiedAroundV1, 4) {
	}
	count1 := layout.totalOccupied()

	layout = parseSeatLayout(lines)
	for layout.apply(layout.occupiedAroundV2, 5) {
	}
	count2 := layout.totalOccupied()

	return fmt.Sprintf("11.1: %d\n11.2: %d\n", count1, count2), nil
}
