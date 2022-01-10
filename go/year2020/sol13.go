package year2020

import (
	"fmt"
	"math"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func earliestDeparture(earliest int, buses [][2]int) (earliestBus, wait int) {
	wait = math.MaxInt
	for _, bus := range buses {
		bId := bus[0]
		currentWait := int(math.Ceil(float64(earliest)/float64(bId)))*bId - earliest
		if currentWait < wait {
			wait = currentWait
			earliestBus = bId
		}
	}
	return earliestBus, wait
}

func earliestTimestamp(buses [][2]int) int {
	t := 0
	offset := 1
	for _, bus := range buses {
		bId, idx := bus[0], bus[1]
		for ((t + idx) % bId) != 0 {
			t += offset
		}
		offset *= bId
	}
	return t
}

func Sol13(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	earliest := util.MustAtoi(lines[0])
	var buses [][2]int
	for i, s := range strings.Split(lines[1], ",") {
		if s == "x" {
			continue
		}
		buses = append(buses, [2]int{util.MustAtoi(s), i})
	}

	earliestBus, wait := earliestDeparture(earliest, buses)

	fmt.Printf("13.1: %d\n13.2: %d\n", earliestBus*wait, earliestTimestamp(buses))
	return nil
}
