package year2022

import (
	"fmt"
	"regexp"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geom"
	"github.com/dhruvmanila/advent-of-code/go/pkg/geom/direction"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var pathRegex = regexp.MustCompile(`(L|R)|(\d+)`)

var facingValue = map[direction.Type]int{
	direction.Right: 0,
	direction.Down:  1,
	direction.Left:  2,
	direction.Up:    3,
}

// intRange represents a [min, max] range.
type intRange struct {
	min, max int
}

type strangelyShapedBoard struct {
	tiles map[geom.Point2D[int]]rune

	// xedges is an array of ranges specifying the min amd max x coordinate for
	// every y coordinate.
	xedges []intRange

	// yedges is an array of ranges specifying the min and max y coordinate for
	// every x coordinate.
	yedges []intRange

	facing direction.Type
	pos    geom.Point2D[int]
}

func NewStrangelyShapedBoard(boardMap []string) *strangelyShapedBoard {
	maxx, maxy := 0, len(boardMap)
	for _, row := range boardMap {
		maxx = util.Max(maxx, len(row))
	}

	xedges := make([]intRange, maxy)
	for i := range xedges {
		xedges[i].min = maxx
	}
	yedges := make([]intRange, maxx)
	for i := range yedges {
		yedges[i].min = maxy
	}

	tiles := make(map[geom.Point2D[int]]rune)

	for y, row := range boardMap {
		for x, char := range row {
			switch char {
			case '.', '#':
				xedges[y].min = util.Min(xedges[y].min, x)
				xedges[y].max = util.Max(xedges[y].max, x)
				yedges[x].min = util.Min(yedges[x].min, y)
				yedges[x].max = util.Max(yedges[x].max, y)
				tiles[geom.Point2D[int]{X: x, Y: y}] = char
			}
		}
	}

	return &strangelyShapedBoard{
		tiles:  tiles,
		xedges: xedges,
		yedges: yedges,
		facing: direction.Right,
		pos:    geom.Point2D[int]{X: xedges[0].min, Y: 0},
	}
}

func (b *strangelyShapedBoard) Move(steps []string) {
	for _, s := range steps {
		switch s {
		case "R":
			b.facing = b.facing.Clockwise()
		case "L":
			b.facing = b.facing.CounterClockwise()
		default:
			count := util.MustAtoi(s)
			delta := b.facing.Delta()
			for n := 0; n < count; n++ {
				nextPos := b.pos.Add(delta)
				if _, ok := b.tiles[nextPos]; !ok {
					switch b.facing {
					case direction.Right:
						nextPos.X = b.xedges[nextPos.Y].min
					case direction.Down:
						nextPos.Y = b.yedges[nextPos.X].min
					case direction.Left:
						nextPos.X = b.xedges[nextPos.Y].max
					case direction.Up:
						nextPos.Y = b.yedges[nextPos.X].max
					}
				}
				if b.tiles[nextPos] == '#' {
					break
				}
				b.pos = nextPos
			}
		}
	}
}

// Password returns the final password.
func (b *strangelyShapedBoard) Password() int {
	return 1000*(b.pos.Y+1) + 4*(b.pos.X+1) + facingValue[b.facing]
}

func Sol22(input string) (string, error) {
	sections := util.ReadSections(input)

	board := NewStrangelyShapedBoard(sections[0])
	steps := pathRegex.FindAllString(sections[1][0], -1)
	board.Move(steps)

	return fmt.Sprintf("22.1: %d\n22.2: %d\n", board.Password(), 0), nil
}
