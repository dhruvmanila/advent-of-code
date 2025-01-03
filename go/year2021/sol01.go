package year2021

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol01(input string) (string, error) {
	depths := util.ReadLinesAsInt(input)

	count1 := 0
	for i := 0; i < len(depths)-1; i++ {
		if depths[i+1] > depths[i] {
			count1++
		}
	}

	count2 := 0
	for i := 0; i < len(depths)-3; i++ {
		if util.Sum(depths[i+1:i+4]) > util.Sum(depths[i:i+3]) {
			count2++
		}
	}

	return fmt.Sprintf("1.1: %d\n1.2: %d\n", count1, count2), nil
}
