package year2020

import (
	"bytes"
	"container/ring"
	"fmt"
	"math"
	"os"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

func predict(current *ring.Ring, positions map[int]*ring.Ring, n int, extended bool) int {
	removedLabels := set.New()
	for ; n > 0; n-- {
		removed := current.Unlink(3)
		removed.Do(func(e interface{}) {
			removedLabels.Add(e.(int))
		})
		destination := util.Mod(current.Value.(int)-2, len(positions)) + 1
		for removedLabels.Contains(destination) {
			destination = util.Mod(destination-2, len(positions)) + 1
		}
		positions[destination].Link(removed)
		current = current.Next()
		removedLabels.Clear()
	}
	var outcome int
	if extended {
		outcome = positions[1].Next().Value.(int)
		outcome *= positions[outcome].Next().Value.(int)
	} else {
		power := 7
		for r := positions[1].Next(); r.Value.(int) != 1; r, power = r.Next(), power-1 {
			outcome += int(math.Pow10(power)) * r.Value.(int)
		}
	}
	return outcome
}

func parseLabels(labels []byte, extended bool) (*ring.Ring, map[int]*ring.Ring) {
	// positions is a map of cup value to a pointer pointing to the actual
	// ring element representing the cup. This will improve the performance
	// when trying to search for the destination cup.
	positions := make(map[int]*ring.Ring)

	var current *ring.Ring
	if extended {
		current = ring.New(1_000_000)
	} else {
		current = ring.New(len(labels))
	}

	for _, digit := range labels {
		value := int(digit - '0')
		positions[value] = current
		current.Value = value
		current = current.Next()
	}

	if !extended {
		return current, positions
	}

	for value := 10; value < 1_000_001; value++ {
		positions[value] = current
		current.Value = value
		current = current.Next()
	}

	return current, positions
}

func Sol23(input string) error {
	labels, err := os.ReadFile(input)
	if err != nil {
		return err
	}
	labels = bytes.Trim(labels, "\n")

	current, positions := parseLabels(labels, false)
	outcome1 := predict(current, positions, 100, false)

	current, positions = parseLabels(labels, true)
	outcome2 := predict(current, positions, 10_000_000, true)

	fmt.Printf("23.1: %d\n23.2: %d\n", outcome1, outcome2)
	return nil
}
