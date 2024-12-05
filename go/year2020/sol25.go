package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

const (
	keySeed int = 7
	mod     int = 20201227
)

func transformSubjectNumber(publicKey int, loopSize int) int {
	value := 1
	for ; loopSize > 0; loopSize-- {
		value = (value * publicKey) % mod
	}
	return value
}

func getLoopSize(publicKey int) int {
	var loopSize int
	for value := 1; value != publicKey; loopSize++ {
		value = (value * keySeed) % mod
	}
	return loopSize
}

func Sol25(input string) (string, error) {
	lines := util.ReadLines(input)

	cardPublicKey := util.MustAtoi(lines[0])
	doorPublicKey := util.MustAtoi(lines[1])
	cardLoopSize := getLoopSize(cardPublicKey)
	doorLoopSize := getLoopSize(doorPublicKey)

	encryptionKey := transformSubjectNumber(doorPublicKey, cardLoopSize)
	if otherKey := transformSubjectNumber(cardPublicKey, doorLoopSize); encryptionKey != otherKey {
		return "", fmt.Errorf("keys do not match: %d != %d", encryptionKey, otherKey)
	}

	return fmt.Sprintf("25.1: %d\n", encryptionKey), nil
}
