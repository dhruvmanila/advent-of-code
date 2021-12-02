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
	if err = scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}
