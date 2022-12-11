package year2016

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/ocr"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type pixelType int

const (
	OFF pixelType = iota
	ON
)

type display struct {
	pixels [][]pixelType
	rows   int
	cols   int
}

func newDisplay(rows, cols int) *display {
	pixels := make([][]pixelType, rows)
	for i := range pixels {
		pixels[i] = make([]pixelType, cols)
	}
	return &display{
		pixels: pixels,
		rows:   rows,
		cols:   cols,
	}
}

func (d *display) executeInstruction(instruction string) {
	parts := strings.SplitN(instruction, " ", 2)
	switch parts[0] {
	case "rect":
		var rows, cols int
		fmt.Sscanf(parts[1], "%dx%d", &cols, &rows)
		for r := 0; r < rows; r++ {
			for c := 0; c < cols; c++ {
				d.pixels[r][c] = ON
			}
		}
	case "rotate":
		subparts := strings.SplitN(parts[1], " ", 2)
		switch subparts[0] {
		case "row":
			var row, shift int
			fmt.Sscanf(subparts[1], "y=%d by %d", &row, &shift)
			newRow := make([]pixelType, d.cols)
			for i, pixel := range d.pixels[row] {
				newRow[(i+shift)%d.cols] = pixel
			}
			d.pixels[row] = newRow
		case "column":
			var col, shift int
			fmt.Sscanf(subparts[1], "x=%d by %d", &col, &shift)
			newCol := make([]pixelType, d.rows)
			for i := 0; i < d.rows; i++ {
				newCol[(i+shift)%d.rows] = d.pixels[i][col]
			}
			for i := 0; i < d.rows; i++ {
				d.pixels[i][col] = newCol[i]
			}
		}
	}
}

func (d *display) onCount() int {
	count := 0
	for _, row := range d.pixels {
		for _, pixel := range row {
			if pixel == ON {
				count++
			}
		}
	}
	return count
}

func (d *display) String() string {
	lines := make([]string, d.rows)
	for i, row := range d.pixels {
		line := ""
		for _, pixel := range row {
			switch pixel {
			case ON:
				line += "#"
			case OFF:
				line += "."
			}
		}
		lines[i] = line
	}
	return strings.Join(lines, "\n")
}

func Sol08(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	d := newDisplay(6, 50)
	for _, line := range lines {
		d.executeInstruction(line)
	}

	code, err := ocr.Convert6(d.String())
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("8.1: %d\n8.2: %s\n", d.onCount(), code), nil
}
