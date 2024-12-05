package year2016

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

var keypadPosition1 = map[[2]int]string{
	// y, x from top left corner
	{0, 0}: "1",
	{0, 1}: "2",
	{0, 2}: "3",
	{1, 0}: "4",
	{1, 1}: "5",
	{1, 2}: "6",
	{2, 0}: "7",
	{2, 1}: "8",
	{2, 2}: "9",
}

var keypadPosition2 = map[[2]int]string{
	// y, x from top left corner
	{0, 2}: "1",
	{1, 1}: "2",
	{1, 2}: "3",
	{1, 3}: "4",
	{2, 0}: "5",
	{2, 1}: "6",
	{2, 2}: "7",
	{2, 3}: "8",
	{2, 4}: "9",
	{3, 1}: "A",
	{3, 2}: "B",
	{3, 3}: "C",
	{4, 2}: "D",
}

func extractBathroomCode(keypad map[[2]int]string, position [2]int, instructions []string) string {
	code := ""
	nextPosition := position
	for _, instructionLine := range instructions {
		for _, instruction := range instructionLine {
			switch instruction {
			case 'D':
				nextPosition[0]++
			case 'L':
				nextPosition[1]--
			case 'R':
				nextPosition[1]++
			case 'U':
				nextPosition[0]--
			}
			if _, ok := keypad[nextPosition]; ok {
				position = nextPosition
			} else {
				nextPosition = position
			}
		}
		code += keypad[position]
	}
	return code
}

func Sol02(input string) (string, error) {
	lines := util.ReadLines(input)

	return fmt.Sprintf(
		"2.1: %s\n2.2: %s\n",
		extractBathroomCode(keypadPosition1, [2]int{1, 1}, lines),
		extractBathroomCode(keypadPosition2, [2]int{2, 0}, lines),
	), nil
}
