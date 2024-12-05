package year2022

import (
	"bytes"
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/iterator"
)

const (
	oneTrillion = 1_000_000_000_000
	rightEdge   = 1      // 0b0000001
	leftEdge    = 1 << 6 // 0b1000000
)

type rockShape []int

func (r rockShape) String() string {
	lines := make([]string, len(r))
	for i, line := range r {
		lines[i] = renderDigits(fmt.Sprintf("%07b", line))
	}
	return strings.Join(lines, "\n")
}

var rockShapes = []rockShape{
	// line
	{
		0b0011110,
	},
	// plus
	{
		0b0001000,
		0b0011100,
		0b0001000,
	},
	// flipped L
	{
		0b0000100,
		0b0000100,
		0b0011100,
	},
	// column
	{
		0b0010000,
		0b0010000,
		0b0010000,
		0b0010000,
	},
	// square
	{
		0b0011000,
		0b0011000,
	},
}

type verticalChamber struct {
	rockPile []int

	jets  *iterator.Cycle[byte]
	rocks *iterator.Cycle[rockShape]
}

// NewVerticalChamber returns a new vertical chamber with the given jets.
func NewVerticalChamber(jets []byte) *verticalChamber {
	return &verticalChamber{
		rockPile: []int{},
		jets:     iterator.NewCycle(jets),
		rocks:    iterator.NewCycle(rockShapes),
	}
}

// nextJet returns the index and the next jet direction.
func (v *verticalChamber) nextJet() byte {
	v.jets.Next()
	return v.jets.Value()
}

// nextRock returns the index and the next rock shape.
func (v *verticalChamber) nextRock() rockShape {
	v.rocks.Next()
	return v.rocks.Value()
}

func (v *verticalChamber) canShift(level int, rock rockShape) bool {
	for r, line := range rock {
		switch v.jets.Value() {
		case '>':
			if (line&rightEdge != 0) || (line>>1&v.rockPile[level+r] != 0) {
				return false
			}
		case '<':
			if (line&leftEdge != 0) || (line<<1&v.rockPile[level+r] != 0) {
				return false
			}
		}
	}
	return true
}

func (v *verticalChamber) canFall(level int, rock rockShape) bool {
	for r, line := range rock {
		if (level+r >= len(v.rockPile)-1) || (line&v.rockPile[level+r+1] != 0) {
			return false
		}
	}
	return true
}

// JetIdx returns the index of the current jet.
func (v *verticalChamber) JetIdx() int { return v.jets.Index() }

// RockIdx returns the index of the current rock.
func (v *verticalChamber) RockIdx() int { return v.rocks.Index() }

// drop drops the next rock taking into consideration the next set of jets.
func (v *verticalChamber) DropRock() {
	nextRock := v.nextRock()
	rock := make(rockShape, len(nextRock))
	copy(rock, nextRock)

	// Add the new empty rows on top of the chamber. The rock's bottom edge
	// is 3 units above the highest rock in the room.
	v.rockPile = append(make([]int, 3+len(rock)), v.rockPile...)

	for level := 0; level < len(v.rockPile); level++ {
		direction := v.nextJet()

		if v.canShift(level, rock) {
			for r := range rock {
				switch direction {
				case '>':
					rock[r] >>= 1
				case '<':
					rock[r] <<= 1
				}
			}
		}

		if !v.canFall(level, rock) {
			for r, line := range rock {
				v.rockPile[level+r] |= line
			}
			for v.rockPile[0] == 0 {
				v.rockPile = v.rockPile[1:]
			}
			break
		}
	}
}

// Height returns the height of the rock pile.
func (v *verticalChamber) Height() int {
	return len(v.rockPile)
}

func (v *verticalChamber) String() string {
	var s string
	for _, line := range v.rockPile {
		s += "|" + renderDigits(fmt.Sprintf("%07b", line)) + "|\n"
	}
	s += "+-------+"
	return s
}

func renderDigits(digits string) string {
	var s string
	for _, digit := range digits {
		switch digit {
		case '0':
			s += "."
		case '1':
			s += "#"
		}
	}
	return s
}

type cacheEntry struct {
	step   int
	height int
}

func Sol17(input string) (string, error) {
	jets := bytes.TrimRight([]byte(input), "\n")

	var height1, height2 int
	cache := make(map[[2]int]cacheEntry)

	room := NewVerticalChamber(jets)
	for i := 0; i < oneTrillion; i++ {
		if i == 2022 {
			height1 = room.Height()
		}
		key := [2]int{room.RockIdx(), room.JetIdx()}
		if v, ok := cache[key]; ok {
			remaining, lastSeen := oneTrillion-i, i-v.step
			if remaining%lastSeen == 0 {
				height2 = room.Height() + remaining/lastSeen*(room.Height()-v.height)
				// Let's compute the first part height if the cycle was
				// seen before the 2022nd rock. This is the case for my
				// test input.
				if i < 2022 {
					for ; i < 2022; i++ {
						room.DropRock()
					}
					height1 = room.Height()
				}
				break
			}
		}
		cache[key] = cacheEntry{step: i, height: room.Height()}
		room.DropRock()
	}

	return fmt.Sprintf("17.1: %d\n17.2: %d\n", height1, height2), nil
}
