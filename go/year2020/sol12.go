package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

type navInstruction struct {
	action byte
	value  int
}

func handleInstructionsV1(instructions []navInstruction) int {
	directions := [][2]int{{1, 0}, {0, 1}, {-1, 0}, {0, -1}}
	x, y := 0, 0
	curdir := 0
	for _, instruction := range instructions {
		switch instruction.action {
		case 'N':
			y += instruction.value
		case 'S':
			y -= instruction.value
		case 'E':
			x += instruction.value
		case 'W':
			x -= instruction.value
		case 'L':
			curdir = (curdir + instruction.value/90) % 4
		case 'R':
			curdir = (curdir + instruction.value*3/90) % 4
		case 'F':
			x += directions[curdir][0] * instruction.value
			y += directions[curdir][1] * instruction.value
		}
	}
	return util.AbsInt(x) + util.AbsInt(y)
}

func handleInstructionsV2(instructions []navInstruction) int {
	x, y, wx, wy := 0, 0, 10, 1
	for _, instruction := range instructions {
		switch instruction.action {
		case 'N':
			wy += instruction.value
		case 'S':
			wy -= instruction.value
		case 'E':
			wx += instruction.value
		case 'W':
			wx -= instruction.value
		case 'L':
			for i := 0; i < instruction.value/90; i++ {
				wx, wy = -wy, wx
			}
		case 'R':
			for i := 0; i < instruction.value/90; i++ {
				wx, wy = wy, -wx
			}
		case 'F':
			x += wx * instruction.value
			y += wy * instruction.value
		}
	}
	return util.AbsInt(x) + util.AbsInt(y)
}

func Sol12(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	instructions := make([]navInstruction, len(lines))
	for i, line := range lines {
		instructions[i] = navInstruction{
			action: line[0],
			value:  util.MustAtoi(line[1:]),
		}
	}

	fmt.Printf(
		"12.1: %d\n12.2: %d\n",
		handleInstructionsV1(instructions),
		handleInstructionsV2(instructions),
	)
	return nil
}
