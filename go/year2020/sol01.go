package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol1(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	// entries is a slice of all the expense entry.
	entries := make([]int, len(lines))

	// report is a set to simplify checking whether a specific expense entry
	// exists in the report.
	report := set.New()

	for i, s := range lines {
		expense := util.Atoi(s)
		report.Add(expense)
		entries[i] = expense
	}

	var x, y int
	for _, x = range entries {
		y = 2020 - x
		if report.Contains(y) {
			break
		}
	}

	var i, a, b, c int
Loop:
	for i, a = range entries {
		for _, b = range entries[i+1:] {
			c = 2020 - a - b
			if report.Contains(c) {
				break Loop
			}
		}
	}

	fmt.Printf("1.1: %d\n1.2: %d\n", x*y, a*b*c)
	return nil
}
