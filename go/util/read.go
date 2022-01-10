package util

import (
	"bufio"
	"bytes"
	"os"
	"strings"
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

// ReadSections is used to read the content of the file at a given path by
// sections. A section is defined as being separated by two newlines and
// each section is then converted into string slice where each element
// corresponds to a single line of that section.
func ReadSections(path string) ([][]string, error) {
	content, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	content = bytes.Trim(content, "\n")

	var sections [][]string
	for _, section := range strings.Split(string(content), "\n\n") {
		var lines []string
		for _, line := range strings.Split(section, "\n") {
			lines = append(lines, line)
		}
		sections = append(sections, lines)
	}
	return sections, nil
}
