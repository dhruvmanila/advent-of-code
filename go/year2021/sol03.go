package year2021

import (
	"fmt"
	"math"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func Sol3(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	// size is the length of a single binary number.
	size := len(lines[0])

	// ones and zeroes represent the quantity of 1 and 0 in the list of binary
	// numbers for the corresponding position. The index of the slice represents
	// the position.
	ones := make([]int, size)
	zeroes := make([]int, size)

	// Count the number of 1 and 0 for each binary number in the respective
	// position.
	for _, line := range lines {
		for pos, char := range line {
			switch char {
			case '0':
				zeroes[pos]++
			case '1':
				ones[pos]++
			}
		}
	}

	var gammaRate, epsilonRate int
	for pos := 0; pos < size; pos++ {
		if ones[size-pos-1] > zeroes[size-pos-1] {
			gammaRate += int(math.Pow(2, float64(pos)))
		} else {
			epsilonRate += int(math.Pow(2, float64(pos)))
		}
	}

	fmt.Printf("3.1: %d\n", gammaRate*epsilonRate)
	return nil
}
