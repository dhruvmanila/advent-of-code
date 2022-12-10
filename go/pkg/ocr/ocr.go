// Package ocr provides ability to convert Advent of Code ASCII art letters
// to plain characters.
package ocr

import (
	"errors"
	"fmt"
	"strings"
)

var ErrRowLength = errors.New("ocr: row length mismatch (expected 6)")

// Convert6 will try to convert the given text of height 6 to characters.
// The text should be separated using the newline character ('\n') which
// will be used to split it. The expected length of lines is 6.
func Convert6(text string) (string, error) {
	return ConvertSlice6(strings.Split(text, "\n"))
}

// ConvertSlice6 will try to convert the given lines of height 6 to characters.
//
// The returned error might be if the length of lines is not 6, all columns are
// not of equal length or unable to recognize the text as a character.
//
// The pixel characters are expected to be a hash character ('#') as the
// fill pixel and a dot character ('.') as the empty pixel.
func ConvertSlice6(lines []string) (string, error) {
	if len(lines) != 6 {
		return "", ErrRowLength
	}
	cols := len(lines[0])
	for idx, line := range lines {
		if len(line) != cols {
			return "", fmt.Errorf("ocr: line %d: length is %d, expected %d", idx+1, len(line), cols)
		}
	}

	// Allocating space approximately. Each character is of 4 wide, but
	// there will be space between them. So, this will allocate more space
	// than the actual number of letters.
	letters := make([]string, 0, cols/4)

	charLines := make([]string, 6)
	for i := 0; i < cols; i += 5 {
		for idx, line := range lines {
			charLines[idx] = line[i : i+4]
		}
		text := strings.Join(charLines, "\n")
		letter, ok := alphabet6[text]
		if !ok {
			return "", fmt.Errorf("ocr: %q: unrecognized text", text)
		}
		letters = append(letters, letter)
	}

	return strings.Join(letters, ""), nil
}
