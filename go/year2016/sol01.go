package year2016

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type direction int

const (
	north direction = iota
	east
	south
	west
)

func Sol01(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	// position is the x and y coordinates representing the current position.
	position := [2]int{0, 0}
	direction := north
	visited := set.New(position)

	// Number of blocks between the initial position and the position which
	// is visited twice. -1 is a sentinel value to indicate that no location
	// has been visited twice yet.
	visitedTwice := -1

	for _, instruction := range strings.Split(lines[0], ", ") {
		switch instruction[0] {
		case 'L':
			direction = util.Mod(direction-1, 4)
		case 'R':
			direction = util.Mod(direction+1, 4)
		default:
			panic("invalid turn: '" + string(instruction[0]) + "'")
		}

		distance := util.MustAtoi(instruction[1:])
		for i := 0; i < distance; i++ {
			switch direction {
			case north:
				position[1]++
			case east:
				position[0]++
			case south:
				position[1]--
			case west:
				position[0]--
			default:
				panic(fmt.Sprintf("invalid direction: %d", direction))
			}
			if visitedTwice == -1 {
				if visited.Contains(position) {
					visitedTwice = util.Abs(position[0]) + util.Abs(position[1])
				} else {
					visited.Add(position)
				}
			}
		}
	}

	blocks := util.Abs(position[0]) + util.Abs(position[1])
	fmt.Printf("1.1: %d\n1.2: %d\n", blocks, visitedTwice)
	return nil
}
