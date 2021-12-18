package year2021

import (
	"fmt"
	"math"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

type ratingType int

const (
	oxygenGenerator ratingType = iota
	co2Scrubber
)

func recursiveFilter(binaryNums []string, rt ratingType, pos int) string {
	// Base case: There is only one value in the slice which is the final answer.
	if len(binaryNums) == 1 {
		return binaryNums[0]
	}
	// Divide the slice into two: numbers starting with 1 and 0.
	var numStartingWithOne, numStartingWithZero []string
	for _, num := range binaryNums {
		switch num[pos] {
		case '0':
			numStartingWithZero = append(numStartingWithZero, num)
		case '1':
			numStartingWithOne = append(numStartingWithOne, num)
		}
	}
	// Increase the bit position.
	pos++
	switch rt {
	case oxygenGenerator:
		if len(numStartingWithOne) >= len(numStartingWithZero) {
			return recursiveFilter(numStartingWithOne, rt, pos)
		} else {
			return recursiveFilter(numStartingWithZero, rt, pos)
		}
	case co2Scrubber:
		if len(numStartingWithOne) >= len(numStartingWithZero) {
			return recursiveFilter(numStartingWithZero, rt, pos)
		} else {
			return recursiveFilter(numStartingWithOne, rt, pos)
		}
	default:
		panic(fmt.Sprintf("Invalid rating type: %v", rt))
	}
}

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

	oxygenGeneratorRating := util.ParseInt(
		recursiveFilter(lines, oxygenGenerator, 0), 2, 0,
	)
	co2ScrubberRating := util.ParseInt(
		recursiveFilter(lines, co2Scrubber, 0), 2, 0,
	)

	fmt.Printf(
		"3.1: %d\n3.2: %d\n",
		gammaRate*epsilonRate,
		oxygenGeneratorRating*co2ScrubberRating,
	)
	return nil
}
