package year2020

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/combinations"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var instructionRegex = regexp.MustCompile(
	`^(?:mask\s*=\s*([01X]+)|mem\[(\d+)\]\s*=\s*(\d+))$`,
)

type memory map[int]int

func (m memory) sum() int {
	total := 0
	for _, v := range m {
		total += v
	}
	return total
}

func runV1(program []string) int {
	// setMask is a mask to set bits, using bit-wise OR (number | bitmask); any
	// 1 in the bitmask sets that bit in the output.
	//
	// clearMask is a mask to clear bits, using bit-wise AND (number &
	// bitmask); any 0 in the bitmask clears that bit.
	var setMask, clearMask int
	mem := make(memory)
	for _, instruction := range program {
		matches := instructionRegex.FindStringSubmatch(instruction)
		switch {
		case len(matches) != 4:
			panic("invalid instruction: '" + instruction + "'")
		case matches[1] != "": // mask instruction
			setMask = util.MustBtoi(strings.ReplaceAll(matches[1], "X", "0"))
			clearMask = util.MustBtoi(strings.ReplaceAll(matches[1], "X", "1"))
		default: // memory instruction
			addr, val := util.MustAtoi(matches[2]), util.MustAtoi(matches[3])
			mem[addr] = (val & clearMask) | setMask
		}
	}
	return mem.sum()
}

func runV2(program []string) int {
	mem := make(memory)
	// setMask is same as from V1 while clearMask is a mask to clear the 'X'
	// bits to 0.
	var setMask, clearMask int
	var floatingBits []int

	for _, instruction := range program {
		matches := instructionRegex.FindStringSubmatch(instruction)
		switch {
		case len(matches) != 4:
			panic("invalid instruction: '" + instruction + "'")
		case matches[1] != "": // mask instruction
			// floatingBits is an array of integers where every number has only
			// a single one bit which is the position of one of the X. Then, we
			// can get all the possible combinations of the floating bits by
			// doing a OR operation to the initial address where all Xs are zero.
			floatingBits = make([]int, 0, strings.Count(matches[1], "X"))
			for i, bit := range []byte(matches[1]) {
				if bit == 'X' {
					floatingBits = append(floatingBits, 1<<(35-i))
				}
			}
			setMask = util.MustBtoi(strings.ReplaceAll(matches[1], "X", "0"))
			// All zeroes are replaced with ones first and then all the Xs are
			// replaced with zeroes. This will convert the given string to be
			// full of ones except for wherever the Xs were.
			clearMask = util.MustBtoi(
				strings.ReplaceAll(strings.ReplaceAll(matches[1], "0", "1"), "X", "0"),
			)
		default: // memory instruction
			addr, val := util.MustAtoi(matches[2]), util.MustAtoi(matches[3])
			// The initial possible address where all the Xs are zero.
			addr = (addr & clearMask) | setMask
			mem[addr] = val
			for _, comb := range combinations.AllInt(floatingBits) {
				nextAddr := addr
				for _, c := range comb {
					nextAddr |= c
				}
				mem[nextAddr] = val
			}
		}
	}

	return mem.sum()
}

func Sol14(input string) error {
	program, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	fmt.Printf("14.1: %d\n14.2: %d\n", runV1(program), runV2(program))
	return nil
}
