package util

import (
	"bufio"
	"os"
	"sort"
	"strconv"
)

// Sum is used to add all the integers in a given array.
func Sum(arr []int) int {
	total := 0
	for _, n := range arr {
		total += n
	}
	return total
}

// SumN is used to sum 1 to n integers.
func SumN(n int) int {
	return n * (n + 1) / 2
}

// MinMax is used to find the minimum and maximum value in the given array of
// integers.
func MinMax(arr []int) (int, int) {
	min, max := arr[0], arr[0]
	for _, val := range arr {
		min = IntMin(min, val)
		max = IntMax(max, val)
	}
	return min, max
}

// IntMax returns the larger of x or y integer.
func IntMax(x, y int) int {
	if x > y {
		return x
	}
	return y
}

// IntMin returns the smaller of x or y integer.
func IntMin(x, y int) int {
	if x < y {
		return x
	}
	return y
}

// Atoi is similar to strconv.Atoi except this function will panic if the
// conversion fails.
func Atoi(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		panic(err)
	}
	return i
}

// SortString is used to sort the individual characters in the given string.
func SortString(s string) string {
	ss := []rune(s)
	sort.Slice(ss, func(i, j int) bool {
		return ss[i] < ss[j]
	})
	return string(ss)
}

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
