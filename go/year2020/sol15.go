package year2020

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func play(numbers []int, rounds int) int {
	seen := make(map[int]int)
	for round, n := range numbers[:len(numbers)-1] {
		seen[n] = round + 1
	}
	mostRecent := numbers[len(numbers)-1]
	for round := len(numbers); round < rounds; round++ {
		lastSeen, ok := seen[mostRecent]
		seen[mostRecent] = round
		if ok {
			mostRecent = round - lastSeen
		} else {
			mostRecent = 0
		}
	}
	return mostRecent
}

func Sol15(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	var numbers []int
	for _, s := range strings.Split(lines[0], ",") {
		numbers = append(numbers, util.MustAtoi(s))
	}

	fmt.Printf("15.1: %d\n15.2: %d\n", play(numbers, 2020), play(numbers, 30000000))
	return nil
}
