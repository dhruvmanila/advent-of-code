package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func treesForSlope(lines []string, right, down int) int {
	size := len(lines[0])
	var x, y, trees int
	for y < len(lines) {
		item := lines[y][x%size]
		if item == '#' {
			trees++
		}
		x += right
		y += down
	}
	return trees
}

func Sol03(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	trees11 := treesForSlope(lines, 1, 1)
	trees31 := treesForSlope(lines, 3, 1)
	trees51 := treesForSlope(lines, 5, 1)
	trees71 := treesForSlope(lines, 7, 1)
	trees12 := treesForSlope(lines, 1, 2)

	return fmt.Sprintf("3.1: %d\n3.2: %d\n", trees31, trees11*trees31*trees51*trees71*trees12), nil
}
