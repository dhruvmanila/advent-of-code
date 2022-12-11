package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

func isValid(num int, preamble []int) bool {
	ns := set.New[int]()
	for _, n := range preamble {
		ns.Add(n)
	}

	for _, n := range preamble {
		if ns.Contains(num - n) {
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
			min = util.Min(min, num2)
			max = util.Max(max, num2)
			if total == invalidNum {
				break Loop
			}
		}
	}
	return min + max
}

func Sol09(input string) (string, error) {
	numbers, err := util.ReadLinesAsInt(input)
	if err != nil {
		return "", err
	}

	var invalidNum int
	for i := 0; i < len(numbers)-25; i++ {
		if !isValid(numbers[i+25], numbers[i:i+25]) {
			invalidNum = numbers[i+25]
		}
	}

	return fmt.Sprintf("9.1: %d\n9.2: %d\n", invalidNum, encryptionWeakness(invalidNum, numbers)), nil
}
