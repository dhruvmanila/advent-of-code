package year2021

import (
	"bytes"
	"fmt"
	"os"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/ocr"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

// foldInstruction contains information regarding a single fold.
type foldInstruction struct {
	// direction is the direction to which to fold to. 'x' and 'y' are the
	// possible values for folding the paper left and up respectively.
	direction rune

	// value is the line on the paper where the fold occurs.
	value int
}

type paper struct {
	// dots is a set of coordinates representing the presence of a dot at that
	// coordinate.
	dots map[point]struct{}

	// rows and columns are the total number of rows and columns in the paper
	// respectively.
	rows    int
	columns int
}

// newPaper is used to construct a paper object from the given set of lines.
// Each line is a comma separated integers representing the x and y coordinate
// where the dot is present.
func newPaper(lines []string) *paper {
	p := paper{
		dots:    make(map[point]struct{}),
		rows:    0,
		columns: 0,
	}
	var rows, columns int
	for _, line := range lines {
		xy := strings.Split(line, ",")
		x, y := util.MustAtoi(xy[0]), util.MustAtoi(xy[1])
		rows, columns = util.Max(rows, y), util.Max(columns, x)
		p.dots[point{x, y}] = struct{}{}
	}
	p.rows = rows + 1
	p.columns = columns + 1
	return &p
}

// fold is used to fold the paper according to the given fold instruction.
func (p *paper) fold(how foldInstruction) {
	switch how.direction {
	case 'x':
		for c := range p.dots {
			// Puzzle states that dots will never appear exactly on a fold line, so
			// we will only consider the dots which lies outside the fold line.
			if c.x > how.value {
				delta := c.x - how.value
				p.dots[point{how.value - delta, c.y}] = struct{}{}
				delete(p.dots, point{c.x, c.y})
			}
		}
		p.columns = how.value
	case 'y':
		for c := range p.dots {
			if c.y > how.value {
				delta := c.y - how.value
				p.dots[point{c.x, how.value - delta}] = struct{}{}
				delete(p.dots, point{c.x, c.y})
			}
		}
		p.rows = how.value
	default:
		panic("invalid fold direction")
	}
}

// dotCount is used to get the number of dots on the paper.
func (p *paper) dotCount() int {
	return len(p.dots)
}

// String is used for presenting the paper.
func (p *paper) String() string {
	lines := make([]string, p.rows)
	for y := 0; y < p.rows; y++ {
		line := ""
		for x := 0; x < p.columns; x++ {
			if _, exist := p.dots[point{x, y}]; exist {
				line += "#"
			} else {
				line += "."
			}
		}
		lines[y] = line
	}
	return strings.Join(lines, "\n")
}

// parseFoldInstructions is used to parse the fold instructions mentioned in
// the given set of lines. The lines are of the form:
//
//	`fold along <direction>=<value>`
func parseFoldInstructions(lines []string) []foldInstruction {
	instructions := make([]foldInstruction, len(lines))
	for i, line := range lines {
		instructions[i] = foldInstruction{
			direction: rune(line[11]),
			value:     util.MustAtoi(line[13:]),
		}
	}
	return instructions
}

func Sol13(input string) (string, error) {
	content, err := os.ReadFile(input)
	if err != nil {
		return "", err
	}
	content = bytes.Trim(content, "\n")

	data := strings.Split(string(content), "\n\n")
	p := newPaper(strings.Split(data[0], "\n"))
	instructions := parseFoldInstructions(strings.Split(data[1], "\n"))

	p.fold(instructions[0])
	count := p.dotCount()
	for _, instruction := range instructions[1:] {
		p.fold(instruction)
	}

	code, err := ocr.Convert6(p.String())
	if err != nil {
		return "", err
	}

	return fmt.Sprintf("13.1: %d\n13.2: %s\n", count, code), nil
}
