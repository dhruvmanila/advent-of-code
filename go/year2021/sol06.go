package year2021

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func simulate(fishes []int, days int) int {
	// fishCount is an array containing the number of fishes corresponding to
	// the index which represents the number of days remaining until creating a
	// new fish.
	var fishCount [9]int

	// Start by adding the given fish to the counter.
	for _, f := range fishes {
		fishCount[f]++
	}

	for d := 0; d < days; d++ {
		newFishes := fishCount[0]
		// Shift all the elements in the array one position down which behaves
		// as the timer being decreased for every fish.
		for cycleDay, count := range fishCount[1:] {
			fishCount[cycleDay] = count
		}
		// Reset the cycle for all the fishes ready to give birth.
		fishCount[6] += newFishes
		// The cycle for all the new fishes is 8.
		fishCount[8] = newFishes
	}

	return util.Sum(fishCount[:])
}

func Sol06(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	// fishes is a slice of integer each representing the number of days
	// remaining until it creates a new fish.
	var fishes []int
	for _, s := range strings.Split(lines[0], ",") {
		fishes = append(fishes, util.MustAtoi(s))
	}

	count1 := simulate(fishes, 80)
	count2 := simulate(fishes, 256)
	fmt.Printf("6.1: %d\n6.2: %d\n", count1, count2)
	return nil
}
