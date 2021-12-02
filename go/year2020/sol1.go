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

	m := make(map[int]bool)
	for _, s := range lines {
		i, err := strconv.Atoi(s)
		if err != nil {
			return err
		}
		m[i] = true
	}

	var x, y int
	for x = range m {
		rem := 2020 - x
		if _, exist := m[rem]; exist {
			y = rem
			break
		}
	}

	fmt.Printf("1.1: %d\n", x*y)
	return nil
}
