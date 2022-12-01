package year2022

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol01(input string) error {
	_, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	fmt.Printf("1.1: %d\n1.2: %d\n", 0, 0)
	return nil
}
