package year2022

import (
	"container/ring"
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

const decryptionKey = 811589153

// positionKey represents a key for the position mapping.
type positionKey struct {
	index int
	value int
}

// constructList constructs a circular list from the given list of numbers.
//
// It returns a map of position key to pointer to the actual list element.
// The key consists of index and the number corresponding to the index. This
// is to accomodate same numbers at different positions. The second return
// value is the position key for number 0.
func constructList(numbers []int) (map[positionKey]*ring.Ring, positionKey) {
	positions := make(map[positionKey]*ring.Ring, len(numbers))
	r := ring.New(len(numbers))
	zeroKey := positionKey{value: 0}
	for idx, number := range numbers {
		if number == 0 {
			zeroKey.index = idx
		}
		positions[positionKey{idx, number}] = r
		r.Value = number
		r = r.Next()
	}
	return positions, zeroKey
}

// mix will mix the numbers n times and returns the sum of coordinates at the
// 1000th, 2000th and 3000th position after number 0.
func mix(numbers []int, n int) (coordinateSum int) {
	positions, zeroKey := constructList(numbers)

	// length is the list length after removing the current element and
	// halflen is the half of length.
	length := len(numbers) - 1
	halflen := length >> 1

	for ; n > 0; n-- {
		for idx, number := range numbers {
			r := positions[positionKey{idx, number}].Prev()
			removed := r.Unlink(1)
			// This optimization is adopted from Python's `deque.rotate` method.
			// If we need to move more than half of the list length, then we
			// should instead move in the other direction which will be shorter.
			if (number > halflen) || (number < -halflen) {
				number %= length
				switch {
				case number > halflen:
					number -= length
				case number < -halflen:
					number += length
				}
			}
			r.Move(number).Link(removed)
		}
	}

	r := positions[zeroKey]
	for i := 1; i <= 3; i++ {
		r = r.Move(1000)
		coordinateSum += r.Value.(int)
	}

	return coordinateSum
}

func Sol20(input string) (string, error) {
	numbers, err := util.ReadLinesAsInt(input)
	if err != nil {
		return "", err
	}

	coordinateSum1 := mix(numbers, 1)

	// Apply the decryption key
	for idx := range numbers {
		numbers[idx] *= decryptionKey
	}

	coordinateSum2 := mix(numbers, 10)

	return fmt.Sprintf("20.1: %d\n20.2: %d\n", coordinateSum1, coordinateSum2), nil
}
