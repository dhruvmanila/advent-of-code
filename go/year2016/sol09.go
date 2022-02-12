package year2016

import (
	"bytes"
	"fmt"
	"os"

	"github.com/dhruvmanila/advent-of-code/go/pkg/iterator"
)

type formatVersion int

const (
	v1 formatVersion = iota + 1
	v2
)

func decompress(data []byte, version formatVersion) int {
	decompressedLen := 0
	var marker bytes.Buffer
	var repeat bytes.Buffer

	it := iterator.New(data)
	for it.Next() {
		switch it.Value() {
		case '(':
			// repeat 'length' characters 'count' number of times
			var length, count int
			for it.Next(); it.Value() != ')'; it.Next() {
				if err := marker.WriteByte(it.Value()); err != nil {
					panic("failed to write to marker buffer: " + string(it.Value()))
				}
			}
			fmt.Sscanf(marker.String(), "%dx%d", &length, &count)

			switch version {
			case v1:
				decompressedLen += count * length
				for ; length > 0; length-- {
					it.Next()
				}
			case v2:
				// Collect all the bytes which needs to be repeated. Here, the
				// condition to compare the length should appear first as we
				// don't want to move beyond the last byte. That will be done
				// by the `for` loop condition.
				for ; length > 0 && it.Next(); length-- {
					if err := repeat.WriteByte(it.Value()); err != nil {
						panic("failed to write to repeat buffer: " + string(it.Value()))
					}
				}
				decompressedLen += count * decompress(repeat.Bytes(), version)
				repeat.Reset()
			}

			marker.Reset()
		default:
			decompressedLen++
		}
	}

	return decompressedLen
}

func Sol09(input string) error {
	content, err := os.ReadFile(input)
	if err != nil {
		return err
	}
	content = bytes.Trim(content, "\n")

	lengthV1 := decompress(content, v1)
	lengthV2 := decompress(content, v2)
	fmt.Printf("9.1: %d\n9.2: %d\n", lengthV1, lengthV2)
	return nil
}
