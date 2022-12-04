package year2022

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol04(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	pairs1, pairs2 := 0, 0
	for idx, line := range lines {
		var min1, max1, min2, max2 int
		_, err := fmt.Sscanf(line, "%d-%d,%d-%d", &min1, &max1, &min2, &max2)
		if err != nil {
			return fmt.Errorf("line %d: %q: %w", idx, line, err)
		}
		// Check if the first range is entirely within the second range or
		// vice versa.
		if (min1 >= min2 && max1 <= max2) || (min2 >= min1 && max2 <= max1) {
			pairs1++
		}
		// Check if there's any overlap between the two ranges.
		if min1 <= max2 && min2 <= max1 {
			pairs2++
		}
	}

	fmt.Printf("4.1: %d\n4.2: %d\n", pairs1, pairs2)
	return nil
}
