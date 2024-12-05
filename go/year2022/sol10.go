package year2022

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/matrix"
	"github.com/dhruvmanila/advent-of-code/go/pkg/ocr"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

const (
	pixelOn      = '#' // '\u2588'
	pixelOff     = '.'
	screenWidth  = 40
	screenHeight = 6
	pixelCount   = screenHeight * screenWidth
)

func Sol10(input string) (string, error) {
	instructions := util.ReadLines(input)

	registerX := 1
	currentCycle := 0
	totalSignal := 0

	screen := matrix.NewDense[rune](screenHeight, screenWidth, nil)

	var cycles, value int
	for idx, instruction := range instructions {
		fields := strings.Fields(instruction)
		if len(fields) < 1 {
			return "", fmt.Errorf("line %d: %q: invalid instruction", idx, instruction)
		}
		switch fields[0] {
		case "noop":
			cycles, value = 1, 0
		case "addx":
			cycles, value = 2, util.MustAtoi(fields[1])
		default:
			return "", fmt.Errorf("line %d: %q: invalid instruction", idx, instruction)
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

	lines := make([]string, 0, screen.Rows)
	for r := 0; r < screen.Rows; r++ {
		lines = append(lines, string(screen.RawRowView(r)))
	}
	letters, err := ocr.ConvertSlice6(lines)
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("10.1: %d\n10.2: %s\n", totalSignal, letters), nil
}
