package year2016

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol06(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	// size is the length of the message.
	size := len(lines[0])

	counters := make([]counter.Counter[rune], 0, size)
	for i := 0; i < size; i++ {
		counters = append(counters, counter.New[rune]())
	}
	for _, line := range lines {
		for position, character := range line {
			counters[position].Increment(character)
		}
	}

	message1, message2 := "", ""
	for i := 0; i < size; i++ {
		message1 += string(counters[i].MostCommon())
		message2 += string(counters[i].LeastCommon())
	}

	return fmt.Sprintf("6.1: %s\n6.2: %s\n", message1, message2), nil
}
