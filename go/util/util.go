package util

import (
	"bufio"
	"os"
)

func Sum(arr []int) int {
	total := 0
	for _, n := range arr {
		total += n
	}
	return total
}

func SumN(n int) int {
	return n * (n + 1) / 2
}

func MinMax(arr []int) (int, int) {
	min := arr[0]
	max := arr[0]
	for _, val := range arr {
		if min > val {
			min = val
		}
		if max < val {
			max = val
		}
	}
	return min, max
}

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
