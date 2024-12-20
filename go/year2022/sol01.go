package year2022

import (
	"fmt"
	"sort"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol01(input string) (string, error) {
	sections := util.ReadSections(input)

	var elves []int
	for _, lines := range sections {
		calories := 0
		for _, line := range lines {
			calories += util.MustAtoi(line)
		}
		elves = append(elves, calories)
	}

	sort.Slice(elves, func(i, j int) bool {
		return elves[i] > elves[j]
	})

	return fmt.Sprintf("1.1: %d\n1.2: %d\n", elves[0], elves[0]+elves[1]+elves[2]), nil
}
