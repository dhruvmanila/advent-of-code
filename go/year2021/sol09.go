package year2021

import (
	"fmt"
	"sort"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

// location contains information regarding the height of the floor at a
// particular location.
type location struct {
	// row and col are the row and column number where the location is situated
	// at respectively.
	row int
	col int

	// height is the height of the floor at the given location.
	height int

	// visited is an internal attribute to indicate whether this location was
	// visited when calculating the basin size.
	visited bool
}

// newLocation is used to create a new location.
func newLocation(row, col, height int) *location {
	return &location{
		row:     row,
		col:     col,
		height:  height,
		visited: false,
	}
}

// heightMap is a two dimensional array of location.
type heightMap [][]*location

// locAt is used to get the location at a given position (row and column). It
// returns nil if the location does not exist.
func (hm heightMap) locAt(row, col int) *location {
	if row >= 0 && row < len(hm) && col >= 0 && col < len(hm[0]) {
		return hm[row][col]
	}
	return nil
}

// adjacentPos is used to get the position of all the four adjacent locations
// (up, right, down, left) for the given position.
func (hm heightMap) adjacentPos(row, col int) [][]int {
	return [][]int{
		{row - 1, col},
		{row, col + 1},
		{row + 1, col},
		{row, col - 1},
	}
}

// adjacentLoc is similar to adjacentPos, but this returns the actual location
// object at the adjacent positions.
func (hm heightMap) adjacentLoc(row, col int) []*location {
	var adjLoc []*location
	for _, pos := range hm.adjacentPos(row, col) {
		if loc := hm.locAt(pos[0], pos[1]); loc != nil {
			adjLoc = append(adjLoc, loc)
		}
	}
	return adjLoc
}

// basinSizeAt is used to calculate the basin size at the given location.
//
// A basin is all locations that eventually flow downward to a single low
// point. The size of a basin is the number of locations within the basin,
// including the low point.
func (hm heightMap) basinSizeAt(row, col int) int {
	var size int
	if loc := hm.locAt(row, col); loc != nil {
		if loc.visited || loc.height == 9 {
			return size
		}
		size++
		loc.visited = true
		for _, adjLoc := range hm.adjacentLoc(row, col) {
			size += hm.basinSizeAt(adjLoc.row, adjLoc.col)
		}
	}
	return size
}

func parseHeightMap(lines []string) heightMap {
	grid := make(heightMap, len(lines))
	for i, line := range lines {
		row := make([]*location, len(line))
		for j, height := range line {
			row[j] = newLocation(i, j, int(height-'0'))
		}
		grid[i] = row
	}
	return grid
}

func Sol09(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	hm := parseHeightMap(lines)

	var lowPoints []*location
	for i, row := range hm {
	NextLocation:
		for j, centerloc := range row {
			for _, adjLoc := range hm.adjacentLoc(i, j) {
				if centerloc.height >= adjLoc.height {
					continue NextLocation
				}
			}
			lowPoints = append(lowPoints, centerloc)
		}
	}

	var riskLevel int
	var basinSize []int
	for _, loc := range lowPoints {
		riskLevel += loc.height + 1
		basinSize = append(basinSize, hm.basinSizeAt(loc.row, loc.col))
	}

	// Sort in decreasing order.
	sort.Slice(basinSize, func(i, j int) bool {
		return basinSize[i] > basinSize[j]
	})

	return fmt.Sprintf("9.1: %d\n9.2: %d\n", riskLevel, basinSize[0]*basinSize[1]*basinSize[2]), nil
}
