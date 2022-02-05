package year2021

import (
	"fmt"
	"math"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol07(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	var currentPos []int
	for _, s := range strings.Split(lines[0], ",") {
		currentPos = append(currentPos, util.MustAtoi(s))
	}

	minFuel1, minFuel2 := math.MaxInt, math.MaxInt
	minPos, maxPos := util.MinMax(currentPos)
	for p := minPos; p <= maxPos; p++ {
		var totalFuel1, totalFuel2 int
		for _, hp := range currentPos {
			steps := util.Abs(hp - p)
			totalFuel1 += steps
			totalFuel2 += util.SumN(steps)
		}
		minFuel1 = util.Min(minFuel1, totalFuel1)
		minFuel2 = util.Min(minFuel2, totalFuel2)
	}

	fmt.Printf("7.1: %d\n7.2: %d\n", minFuel1, minFuel2)
	return nil
}
