package year2021

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

type command struct {
	direction string
	units     int
}

func parseCommands(lines []string) ([]*command, error) {
	cmds := make([]*command, len(lines))
	for i, line := range lines {
		// format: "{direction} {units}"
		s := strings.Split(line, " ")
		cmds[i] = &command{
			direction: s[0],
			units:     util.Atoi(s[1]),
		}
	}
	return cmds, nil
}

func Sol2(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	cmds, err := parseCommands(lines)
	if err != nil {
		return err
	}

	// hpos (horizontal position) calculation remains the same.
	// depth1 and depth2 is used for the first and second problem respectively.
	var aim, hpos, depth1, depth2 int
	for _, cmd := range cmds {
		switch cmd.direction {
		case "forward":
			hpos += cmd.units
			depth2 += aim * cmd.units
		case "down":
			aim += cmd.units
			depth1 += cmd.units
		case "up":
			aim -= cmd.units
			depth1 -= cmd.units
		default:
			return fmt.Errorf("command: unknown direction: %s", cmd.direction)
		}
	}

	fmt.Printf("2.1: %d\n2.2: %d\n", hpos*depth1, hpos*depth2)
	return nil
}
