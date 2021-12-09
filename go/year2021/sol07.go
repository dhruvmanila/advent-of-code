package year2021

import (
	"fmt"
	"math"
	"strconv"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol7(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	var currentPos []int
	for _, s := range strings.Split(lines[0], ",") {
		num, err := strconv.Atoi(s)
		if err != nil {
			return err
		}
		currentPos = append(currentPos, num)
	}

	minFuel1, minFuel2 := math.MaxInt, math.MaxInt
	minPos, maxPos := util.MinMax(currentPos)
	for p := minPos; p <= maxPos; p++ {
		var totalFuel1, totalFuel2 int
		for _, hp := range currentPos {
			steps := int(math.Abs(float64(hp - p)))
			totalFuel1 += steps
			totalFuel2 += util.SumN(steps)
		}
		if totalFuel1 < minFuel1 {
			minFuel1 = totalFuel1
		}
		if totalFuel2 < minFuel2 {
			minFuel2 = totalFuel2
		}
	}

	fmt.Printf("7.1: %d\n7.2: %d\n", minFuel1, minFuel2)
	return nil
}
