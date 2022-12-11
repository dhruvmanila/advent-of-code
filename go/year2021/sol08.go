package year2021

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

// groupByLength is used to group the given list of strings as per their length.
func groupByLength(strings []string) map[int][]string {
	lengthBin := make(map[int][]string)
	for _, s := range strings {
		if _, exist := lengthBin[len(s)]; exist {
			lengthBin[len(s)] = append(lengthBin[len(s)], s)
		} else {
			lengthBin[len(s)] = []string{s}
		}
	}
	return lengthBin
}

// contains is used to check if all the characters in string a is contained in
// string b.
func contains(a, b string) bool {
	for _, char := range a {
		if !strings.ContainsRune(b, char) {
			return false
		}
	}
	return true
}

// diffString is used to get a string comprising of characters present in 'a'
// and absent in 'b'.
func diffString(a, b string) string {
	var diff string
	for _, char := range a {
		if !strings.ContainsRune(b, char) {
			diff += string(char)
		}
	}
	return diff
}

func deduceSignalPatterns(patterns []string) map[string]string {
	// digitPattern is an array of pattern where the index corresponds to the
	// number representing the pattern.
	digitPattern := make([]string, 10)

	lengthBin := groupByLength(patterns)
	digitPattern[1] = lengthBin[2][0]
	digitPattern[7] = lengthBin[3][0]
	digitPattern[4] = lengthBin[4][0]
	digitPattern[8] = lengthBin[7][0]

	// Reference example:
	// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab
	// | cdfeb fcadb cdfeb cdbaf

	// cAndF and bAndD are a string of wire characters for the current display
	// which corresponds to the segments at 'c' and 'f' and at 'b' and 'd'
	// respectively.
	//
	// In terms of the reference example, the variable value will be:
	// - cAndF: "ab"
	// - bAndD: "ef"
	cAndF := digitPattern[1]
	bAndD := diffString(digitPattern[4], digitPattern[1])

	// Deduce the patterns of length 6. The deduction is based on the unique
	// property where for a particular digit, both segments are not turned on.
	for _, pattern := range lengthBin[6] {
		switch {
		case !contains(cAndF, pattern):
			digitPattern[6] = pattern
		case !contains(bAndD, pattern):
			digitPattern[0] = pattern
		default:
			digitPattern[9] = pattern
		}
	}

	// Deduce the patterns of length 5. The deductino is based on the unique
	// property where for a particular digit, both the segments are turned on.
	for _, pattern := range lengthBin[5] {
		switch {
		case contains(bAndD, pattern):
			digitPattern[5] = pattern
		case contains(cAndF, pattern):
			digitPattern[3] = pattern
		default:
			digitPattern[2] = pattern
		}
	}

	// deducedMap is a map from sorted pattern string to the corresponding
	// digit in string.
	deducedMap := make(map[string]string)
	for digit, pattern := range digitPattern {
		deducedMap[util.SortString(pattern)] = strconv.Itoa(digit)
	}
	return deducedMap
}

func Sol08(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	var count, totalOutput int
	for _, line := range lines {
		entry := strings.Split(line, " | ")
		deducedMap := deduceSignalPatterns(strings.Fields(entry[0]))
		var s string
		for _, outPattern := range strings.Fields(entry[1]) {
			switch len(outPattern) {
			case 2, 3, 4, 7:
				count++
				fallthrough
			default:
				s += deducedMap[util.SortString(outPattern)]
			}
		}
		totalOutput += util.MustAtoi(s)
	}

	return fmt.Sprintf("8.1: %d\n8.2: %d\n", count, totalOutput), nil
}
