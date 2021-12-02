package year2020

import (
	"fmt"
	"strconv"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol1(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	// entries is a slice of all the expense entry.
	entries := make([]int, len(lines))

	// entryMap is a map to simplify checking whether a specific expense entry
	// exists in the report.
	entryMap := make(map[int]bool)

	for i, s := range lines {
		expense, err := strconv.Atoi(s)
		if err != nil {
			return err
		}
		entryMap[expense] = true
		entries[i] = expense
	}

	var x, y int
	for _, x = range entries {
		y = 2020 - x
		if _, exist := entryMap[y]; exist {
			break
		}
	}

	var i, a, b, c int
Loop:
	for i, a = range entries {
		for _, b = range entries[i+1:] {
			c = 2020 - a - b
			if _, exist := entryMap[c]; exist {
				break Loop
			}
		}
	}

	fmt.Printf("1.1: %d\n1.2: %d\n", x*y, a*b*c)
	return nil
}
