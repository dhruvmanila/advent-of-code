package year2021

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/queue"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

// octopusGrid contains information regarding the grid formed by all the octopuses.
type octopusGrid struct {
	// grid is a map from position to the octopus energy level.
	grid map[position]int

	// flashQueue is a position queue used to enqueue all the positions which
	// needs to be flashed.
	flashQueue *queue.Queue[position]
}

// newOctopusGrid is used to create a new octopus grid.
func newOctopusGrid(grid map[position]int) *octopusGrid {
	return &octopusGrid{
		grid:       grid,
		flashQueue: queue.New[position](),
	}
}

// adjacentPos is used to get all the adjacent position from the given position,
// including the diagonal positions. The list will not include the given position.
func (og *octopusGrid) adjacentPos(pos position) []position {
	var adjPos []position
	for x := -1; x <= 1; x++ {
		for y := -1; y <= 1; y++ {
			if x == 0 && y == 0 {
				continue
			}
			adjPos = append(adjPos, position{pos.row + x, pos.col + y})
		}
	}
	return adjPos
}

// incLevel is used to increase the energy level of an octopus at position. If
// the energy level is greater than 9, it is enqueued to be flashed later.
func (og *octopusGrid) incLevel(pos position) {
	og.grid[pos]++
	if og.grid[pos] > 9 {
		og.flashQueue.Enqueue(pos)
	}
}

// step is used to model a single step which involves the energy level increase
// and the flashes of light.
func (og *octopusGrid) step() int {
	for pos := range og.grid {
		og.incLevel(pos)
	}
	return og.flash()
}

// flash is used to flash all the octopuses which have been enqueued.
func (og *octopusGrid) flash() int {
	flashes := 0
	for {
		pos, ok := og.flashQueue.Dequeue()
		if !ok {
			break
		}
		if og.grid[pos] == 0 {
			continue
		}
		og.grid[pos] = 0
		flashes++
		for _, adjPos := range og.adjacentPos(pos) {
			if energyLevel, exist := og.grid[adjPos]; exist && energyLevel != 0 {
				og.incLevel(adjPos)
			}
		}
	}
	return flashes
}

// isAllFlashing is used to check if all the octopuses in the grid are
// currently flashing.
func (og *octopusGrid) isAllFlashing() bool {
	for _, energyLevel := range og.grid {
		if energyLevel != 0 {
			return false
		}
	}
	return true
}

func Sol11(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	grid := make(map[position]int)
	for row, line := range lines {
		for col, level := range line {
			grid[position{row, col}] = int(level - '0')
		}
	}
	og := newOctopusGrid(grid)

	flashes := 0
	step := 1
	for {
		currFlashes := og.step()
		if step <= 100 {
			flashes += currFlashes
		}
		if og.isAllFlashing() {
			break
		}
		step++
	}

	return fmt.Sprintf("11.1: %d\n11.2: %d\n", flashes, step), nil
}
