package util

import "strings"

// ReadLines is used to read the content of the file at a given path into a
// string slice where each element corresponds to a single line.
func ReadLines(input string) []string {
	var lines []string
	for _, line := range strings.Split(input, "\n") {
		lines = append(lines, line)
	}

	return lines
}

// ReadLinesAsInt is similar to ReadLines, except this will convert each line
// into an integer and return an int slice instead.
func ReadLinesAsInt(input string) []int {
	lines := ReadLines(input)

	ints := make([]int, len(lines))
	for i, line := range lines {
		ints[i] = MustAtoi(line)
	}

	return ints
}

// ReadSections is used to read the content of the file at a given path by
// sections. A section is defined as being separated by two newlines and
// each section is then converted into string slice where each element
// corresponds to a single line of that section.
func ReadSections(input string) [][]string {
	input = strings.Trim(input, "\n")

	var sections [][]string
	for _, section := range strings.Split(input, "\n\n") {
		var lines []string
		for _, line := range strings.Split(section, "\n") {
			lines = append(lines, line)
		}
		sections = append(sections, lines)
	}

	return sections
}
