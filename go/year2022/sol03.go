package year2022

import (
	"fmt"
	"unicode"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

func getPriority(item rune) int {
	if unicode.IsLower(item) {
		return int(item-'a') + 1
	} else if unicode.IsUpper(item) {
		return int(item-'A') + 27
	}
	panic(fmt.Sprintf("invalid item: %q", item))
}

func Sol03(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	sharedItemPriority, badgePriority := 0, 0

	for i := 0; i < len(lines); i += 3 {
		group := lines[i : i+3]

		// Part 1
		for _, line := range group {
			mid := len(line) / 2
			first, second := set.NewFromSlice([]rune(line[:mid])), set.NewFromSlice([]rune(line[mid:]))
			shared := first.Intersection(second)
			if shared.Len() != 1 {
				return "", fmt.Errorf(
					"%q %q: expected only 1 shared rucksack item, got %q",
					line[:mid], line[mid:], shared.ToSlice(),
				)
			}
			sharedItemPriority += getPriority(shared.Pop())
		}

		// Part 2
		badge := set.NewFromSlice([]rune(group[0])).
			Intersection(set.NewFromSlice([]rune(group[1]))).
			Intersection(set.NewFromSlice([]rune(group[2])))
		if badge.Len() != 1 {
			return "", fmt.Errorf(
				"group %q: expected only 1 badge for the group, got %q",
				group, badge.ToSlice(),
			)
		}
		badgePriority += getPriority(badge.Pop())
	}

	return fmt.Sprintf("3.1: %d\n3.2: %d\n", sharedItemPriority, badgePriority), nil
}
