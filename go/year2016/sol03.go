package year2016

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func isValidTriangle(x int, y int, z int) bool {
	return x+y > z && x+z > y && y+z > x
}

func Sol03(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	triangles := make([][3]int, len(lines))
	for i, line := range lines {
		for j, number := range strings.Fields(line) {
			triangles[i][j] = util.MustAtoi(number)
		}
	}

	valid1 := 0
	valid2 := 0
	for i := 0; i < len(triangles); i += 3 {
		group := triangles[i : i+3]
		for j := 0; j < len(group); j++ {
			if isValidTriangle(group[j][0], group[j][1], group[j][2]) {
				valid1++
			}
			if isValidTriangle(group[0][j], group[1][j], group[2][j]) {
				valid2++
			}
		}
	}

	fmt.Printf("3.1: %d\n3.2: %d\n", valid1, valid2)
	return nil
}
