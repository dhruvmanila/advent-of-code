package year2022

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

func detectMarker(stream string, packetLen int) int {
	processed := packetLen
	// packet is a counter containing all the elements in the current
	// sequence. The total number of elements in the packet will always
	// be equal to the given packetLen.
	packet := counter.NewFromSlice([]rune(stream[:packetLen]))
	for _, char := range stream[packetLen:] {
		if packet.Len() == packetLen {
			break
		}
		toRemoveChar := rune(stream[processed-packetLen])
		packet.Decrement(toRemoveChar)
		if packet.Get(toRemoveChar) == 0 {
			packet.Delete(toRemoveChar)
		}
		packet.Increment(char)
		processed++
	}
	return processed
}

func Sol06(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}
	stream := lines[0]

	return fmt.Sprintf("6.1: %d\n6.2: %d\n", detectMarker(stream, 4), detectMarker(stream, 14)), nil
}
