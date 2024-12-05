package year2021

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type seafloor struct {
	// east and south is a set of position where the east-facing and
	// south-facing sea cucumbers are present respectively.
	east  set.Set[position]
	south set.Set[position]

	// rows and cols are the total number of rows and columns on the seafloor.
	rows int
	cols int
}

func newSeafloor(lines []string) *seafloor {
	east := set.New[position]()
	south := set.New[position]()
	for row, line := range lines {
		for col, char := range line {
			switch char {
			case '>':
				east.Add(position{row, col})
			case 'v':
				south.Add(position{row, col})
			}
		}
	}
	return &seafloor{
		east:  east,
		south: south,
		rows:  len(lines),
		cols:  len(lines[0]),
	}
}

func (sf *seafloor) move() bool {
	moved := false
	newEast := set.NewWithSize[position](sf.east.Len())
	newSouth := set.NewWithSize[position](sf.south.Len())

	sf.east.ForEach(func(pos position) {
		newPos := position{pos.row, (pos.col + 1) % sf.cols}
		switch {
		case sf.east.Contains(newPos):
			fallthrough
		case sf.south.Contains(newPos):
			newEast.Add(pos)
		default:
			newEast.Add(newPos)
			moved = true
		}
	})

	sf.south.ForEach(func(pos position) {
		newPos := position{(pos.row + 1) % sf.rows, pos.col}
		switch {
		case newEast.Contains(newPos):
			fallthrough
		case sf.south.Contains(newPos):
			newSouth.Add(pos)
		default:
			newSouth.Add(newPos)
			moved = true
		}
	})

	sf.east = newEast
	sf.south = newSouth
	return moved
}

func (sf *seafloor) String() string {
	var s string
	for r := 0; r < sf.rows; r++ {
		for c := 0; c < sf.cols; c++ {
			p := position{r, c}
			switch {
			case sf.east.Contains(p):
				s += ">"
			case sf.south.Contains(p):
				s += "v"
			default:
				s += "."
			}
		}
		s += "\n"
	}
	return s
}

func Sol25(input string) (string, error) {
	lines := util.ReadLines(input)

	sf := newSeafloor(lines)
	var steps int
	for steps = 1; sf.move(); steps++ {
	}

	return fmt.Sprintf("25.1: %d\n", steps), nil
}
