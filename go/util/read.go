package util

import (
	"bufio"
	"os"
)

// ReadLines is used to read the content of the file at a given path into a
// string slice where each element corresponds to a single line.
func ReadLines(path string) ([]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

// ReadLinesAsInt is similar to ReadLines, except this will convert each line
// into an integer and return an int slice instead.
func ReadLinesAsInt(path string) ([]int, error) {
	lines, err := ReadLines(path)
	if err != nil {
		return nil, err
	}

	ints := make([]int, len(lines))
	for i, line := range lines {
		ints[i] = MustAtoi(line)
	}
	return ints, nil
}
