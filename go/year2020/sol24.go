package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/iterator"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type hexDirection uint8

const (
	east hexDirection = iota
	southEast
	southWest
	west
	northWest
	northEast
)

// hexDirectionMap is a map from a string, representing the hex direction,
// to the respective hexDirection.
var hexDirectionMap = map[string]hexDirection{
	"e":  east,
	"se": southEast,
	"sw": southWest,
	"w":  west,
	"nw": northWest,
	"ne": northEast,
}

// hexDirectionOffsets is an array representing the offsets for a specific
// direction from the referenced hex.
var hexDirectionOffsets = [6]hex{
	// The order is maintained as defined by the hexDirection constants.
	newHex(1, 0),  // east
	newHex(0, 1),  // southEast
	newHex(-1, 1), // southWest
	newHex(-1, 0), // west
	newHex(0, -1), // northWest
	newHex(1, -1), // northEast
}

// hex describes a regular hexagon with represented using the Axial Coordinate
// System, sometimes called "trapezoidal" or "oblique" or "skewed".
//
// For additional reference on these coordinate systems:
// http://www.redblobgames.com/grids/hexagons/#coordinates
type hex struct {
	q int // x axis
	r int // y axis
}

// newHex creates a new hex using the axial interface.
func newHex(q, r int) hex {
	return hex{q: q, r: r}
}

// neighbor returns the neighboring hex for the receiver hex h at the given
// direction.
func (h *hex) neighbor(direction hexDirection) hex {
	offset := hexDirectionOffsets[direction]
	return newHex(h.q+offset.q, h.r+offset.r)
}

// allNeighbors returns all the neighboring hexes for the receiver hex h.
func (h *hex) allNeighbors() []hex {
	neighbors := make([]hex, 0, 6)
	for _, direction := range hexDirectionMap {
		neighbors = append(neighbors, h.neighbor(direction))
	}
	return neighbors
}

func (h *hex) String() string {
	return fmt.Sprintf("(%d, %d)", h.q, h.r)
}

func getBlackTiles(instructions []string) set.Set[hex] {
	blackTiles := set.New[hex]()
	for _, instruction := range instructions {
		position := newHex(0, 0)
		it := iterator.New([]byte(instruction))
		for it.Next() {
			d := string(it.Value())
			if d == "s" || d == "n" {
				it.Next()
				d += string(it.Value())
			}
			position = position.neighbor(hexDirectionMap[d])
		}
		if blackTiles.Contains(position) {
			blackTiles.Remove(position)
		} else {
			blackTiles.Add(position)
		}
	}
	return blackTiles
}

func runArtExhibit(blackTiles set.Set[hex], days int) int {
	for ; days > 0; days-- {
		newBlackTiles := set.New[hex]()
		whiteTiles := set.New[hex]()
		blackTiles.ForEach(func(tile hex) {
			blackCount := 0
			for _, neighbor := range tile.allNeighbors() {
				if blackTiles.Contains(neighbor) {
					blackCount++
				} else {
					whiteTiles.Add(neighbor)
				}
			}
			switch blackCount {
			case 1, 2:
				newBlackTiles.Add(tile)
			}
		})
		whiteTiles.ForEach(func(tile hex) {
			blackCount := 0
			for _, neighbor := range tile.allNeighbors() {
				if blackTiles.Contains(neighbor) {
					blackCount++
				}
			}
			if blackCount == 2 {
				newBlackTiles.Add(tile)
			}
		})
		blackTiles = newBlackTiles
	}
	return blackTiles.Len()
}

func Sol24(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	blackTiles := getBlackTiles(lines)
	count1 := blackTiles.Len()
	count2 := runArtExhibit(blackTiles, 100)

	return fmt.Sprintf("24.1: %d\n24.2: %d\n", count1, count2), nil
}
