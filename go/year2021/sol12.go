package year2021

import (
	"fmt"
	"strings"
	"unicode"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

type caveMap map[string][]string

// connect is used to connect one cave to another.
func (cm caveMap) connect(from, to string) {
	if _, exist := cm[from]; exist {
		cm[from] = append(cm[from], to)
	} else {
		cm[from] = []string{to}
	}
}

// countPaths is used to count the total number of possible paths from origin
// to target for both the first and second part of the puzzle.
func (cm caveMap) countPaths(origin, target string) (int, int) {
	var pathsFrom func(string, map[string]bool, bool) (int, int)

	pathsFrom = func(from string, visited map[string]bool, smallCaveVisitedTwice bool) (int, int) {
		if from == target {
			return 1, 1
		}

		// Always work with a copy of the original map, otherwise the same map
		// would be used in other branches even though we haven't visited the
		// cave from that branch.
		visitedCopy := map[string]bool{
			// Invariant: Big and small caves will always have the value true
			// and false respectively.
			from: isSmallCave(from),
		}
		for k, v := range visited {
			visitedCopy[k] = v
		}

		var count1, count2 int
		for _, cave := range cm[from] {
			if !visitedCopy[cave] {
				c1, c2 := pathsFrom(cave, visitedCopy, smallCaveVisitedTwice)
				count1 += c1
				count2 += c2
				// Here, the type of the cave is always going to be small.
			} else if !smallCaveVisitedTwice && cave != "start" {
				_, c2 := pathsFrom(cave, visitedCopy, true)
				count2 += c2
			}
		}
		return count1, count2
	}

	return pathsFrom(origin, make(map[string]bool), false)
}

// isSmallCave is used to check if the given cave is small. This assumes that
// the given string contains all letters.
func isSmallCave(name string) bool {
	for _, c := range name {
		if unicode.IsUpper(c) {
			return false
		}
	}
	return true
}

func Sol12(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	m := make(caveMap)
	for _, line := range lines {
		caves := strings.Split(line, "-")
		m.connect(caves[0], caves[1])
		m.connect(caves[1], caves[0])
	}

	count1, count2 := m.countPaths("start", "end")
	fmt.Printf("12.1: %d\n12.2: %d\n", count1, count2)
	return nil
}
