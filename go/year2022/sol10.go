package year2022

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/matrix"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

const (
	pixelOn      = '\u2588'
	pixelOff     = ' '
	screenWidth  = 40
	screenHeight = 6
	pixelCount   = screenHeight * screenWidth
)

func Sol10(input string) error {
	instructions, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	registerX := 1
	currentCycle := 0
	totalSignal := 0

	screen := matrix.NewDense[rune](screenHeight, screenWidth, nil)

	var cycles, value int
	for _, instruction := range instructions {
		fields := strings.Fields(instruction)
		switch fields[0] {
		case "noop":
			cycles, value = 1, 0
		case "addx":
			cycles, value = 2, util.MustAtoi(fields[1])
		default:
			panic("invalid instruction: " + instruction)
		}

		for c := 0; c < cycles; c++ {
			crtCol, crtRow := currentCycle%screenWidth, currentCycle/screenWidth
			if registerX-1 <= crtCol && crtCol <= registerX+1 {
				screen.Set(crtRow, crtCol, pixelOn)
			} else {
				screen.Set(crtRow, crtCol, pixelOff)
			}

			currentCycle++
			if currentCycle%40 == 20 {
				totalSignal += currentCycle * registerX
			}
		}

		registerX += value
		if currentCycle == pixelCount {
			break
		}
	}

	fmt.Printf("10.1: %d\n10.2:\n", totalSignal)

	for r := 0; r < screen.Rows; r++ {
		for _, pixel := range screen.RawRowView(r) {
			fmt.Printf("%s", string(pixel))
		}
		fmt.Println()
	}

	return nil
}
