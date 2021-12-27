package year2021

import "fmt"

// point contains information for a point with x and y coordinates.
type point struct {
	x int
	y int
}

func (p *point) String() string {
	return fmt.Sprintf("(%d, %d)", p.x, p.y)
}

// position contains information regarding a specific position in the matrix.
type position struct {
	row int
	col int
}

func (p *position) String() string {
	return fmt.Sprintf("(%d, %d)", p.row, p.col)
}
