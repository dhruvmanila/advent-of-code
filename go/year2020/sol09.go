package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func isValid(num int, preamble []int) bool {
	set := make(map[int]struct{})
	for _, n := range preamble {
		set[n] = struct{}{}
	}

	for _, n := range preamble {
		if _, exist := set[num-n]; exist {
			return true
		}
	}
	return false
}

func encryptionWeakness(invalidNum int, numbers []int) int {
	var min, max int
Loop:
	for start, num1 := range numbers {
		total := num1
		min, max = num1, num1
		for _, num2 := range numbers[start+1:] {
			total += num2
			min = util.IntMin(min, num2)
			max = util.IntMax(max, num2)
			if total == invalidNum {
				break Loop
			}
		}
	}
	return min + max
}

func Sol9(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	numbers := make([]int, len(lines))
	for i, line := range lines {
		numbers[i] = util.Atoi(line)
	}

	var invalidNum int
	for i := 0; i < len(numbers)-25; i++ {
		if !isValid(numbers[i+25], numbers[i:i+25]) {
			invalidNum = numbers[i+25]
		}
	}

	fmt.Printf("9.1: %d\n9.2: %d\n", invalidNum, encryptionWeakness(invalidNum, numbers))
	return nil
}
