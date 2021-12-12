package year2020

import (
	"bytes"
	"fmt"
	"math"
	"math/bits"
	"os"
	"strings"
)

func Sol6(input string) error {
	content, err := os.ReadFile(input)
	if err != nil {
		return err
	}
	// Remove the trailing newlines.
	content = bytes.Trim(content, "\n")

	var count1, count2 int
	// lines is a single group.
	for _, lines := range strings.Split(string(content), "\n\n") {
		var set1 uint
		var set2 uint = math.MaxUint
		// line is a single line in a group.
		for _, line := range strings.Split(lines, "\n") {
			var mask uint
			// c is a single character of a line in a group.
			for _, c := range line {
				set1 |= 1 << (c - 'a')
				mask |= 1 << (c - 'a')
			}
			set2 &= mask
		}
		count1 += bits.OnesCount(set1)
		count2 += bits.OnesCount(set2)
	}

	fmt.Printf("6.1: %d\n6.2: %d\n", count1, count2)
	return nil
}
