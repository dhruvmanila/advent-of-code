package year2021

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

// cell is an individual unit of the board. It contains information about the
// number contained in the cell and whether the cell is marked or not.
type cell struct {
	value  int
	marked bool
}

// board is a 5x5 grid of cell.
type board struct {
	grid     [5][5]*cell
	complete bool
}

func newBoard(grid [5][5]int) *board {
	var b board
	for i, row := range grid {
		for j, num := range row {
			b.grid[i][j] = &cell{value: num, marked: false}
		}
	}
	return &b
}

// mark is used to mark a board cell containing the provided number.
func (b *board) mark(n int) {
	for _, row := range b.grid {
		for _, cell := range row {
			if cell.value == n {
				cell.marked = true
				b.checkComplete()
				return
			}
		}
	}
}

// checkComplete is used to check if the current board is complete. A completed
// board is found out by checking if the cells in any of the horizontal or
// vertical rows are all marked.
func (b *board) checkComplete() {
	for i, row := range b.grid {
		// isHorUnmarked and isVerUnmarked states whether the current horizontal
		// row and vertical column contains any unmarked cell.
		isHorUnmarked, isVerUnmarked := false, false
		for j, cell := range row {
			// Going horizontal
			if !cell.marked {
				isHorUnmarked = true
			}
			// Going vertical
			if !b.grid[j][i].marked {
				isVerUnmarked = true
			}
		}
		// If the cells in the current horizontal row or vertical column are
		// all marked, this board is the winner.
		if !isHorUnmarked || !isVerUnmarked {
			b.complete = true
		}
	}
}

// unmarkedSum is used to find out the total value of all the unmarked cells
// in the board.
func (b *board) unmarkedSum() int {
	var total int
	for _, row := range b.grid {
		for _, cell := range row {
			if !cell.marked {
				total += cell.value
			}
		}
	}
	return total
}

func playBingo(toDraw []int, boards []*board) (int, int) {
	var winnerScore, lastScore int

	for _, num := range toDraw {
		for _, b := range boards {
			if b.complete {
				continue
			}
			b.mark(num)
			if b.complete {
				if winnerScore == 0 {
					winnerScore = b.unmarkedSum() * num
				} else {
					lastScore = b.unmarkedSum() * num
				}
			}
		}
	}

	return winnerScore, lastScore
}

// parseBoards is used to parse the input lines into a list of board.
func parseBoards(lines []string) ([]*board, error) {
	var boards []*board

	// Input is 5 lines containing the board values separated by a newline.
	for i := 0; i < len(lines); i += 6 {
		var grid [5][5]int
		// Skip the empty line between board values.
		for i, line := range lines[i : i+5] {
			var row [5]int
			for j, s := range strings.Fields(line) {
				num, err := strconv.Atoi(s)
				if err != nil {
					return nil, err
				}
				row[j] = num
			}
			grid[i] = row
		}
		boards = append(boards, newBoard(grid))
	}

	return boards, nil
}

func Sol4(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	var draws []int
	// Collect all the numbers which are to be drawn. This is the first line
	// of the input and is a comma-separated list of numbers.
	for _, s := range strings.Split(lines[0], ",") {
		num, err := strconv.Atoi(s)
		if err != nil {
			return err
		}
		draws = append(draws, num)
	}

	boards, err := parseBoards(lines[2:])
	if err != nil {
		return err
	}

	first, last := playBingo(draws, boards)
	fmt.Printf("4.1: %d\n4.2: %d\n", first, last)

	return nil
}
