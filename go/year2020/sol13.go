package year2020

import (
	"fmt"
	"math"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

const sentinel = -1

func earliestDeparture(earliest int, busIds []int) (earliestBus, wait int) {
	wait = math.MaxInt
	for _, bId := range busIds {
		currentWait := int(math.Ceil(float64(earliest)/float64(bId)))*bId - earliest
		if currentWait < wait {
			wait = currentWait
			earliestBus = bId
		}
	}
	return earliestBus, wait
}

func Sol13(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	earliest := util.MustAtoi(lines[0])
	var busIds []int
	for _, s := range strings.Split(lines[1], ",") {
		if s == "x" {
			continue
		}
		busIds = append(busIds, util.MustAtoi(s))
	}

	earliestBus, wait := earliestDeparture(earliest, busIds)

	fmt.Printf("13.1: %d\n13.2: %d\n", earliestBus*wait, 0)
	return nil
}
