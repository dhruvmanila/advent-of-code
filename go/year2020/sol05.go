package year2020

import (
	"fmt"
	"sort"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

type boardingPass string

func (bp boardingPass) findRow() int {
	return bisectionSearch(bp[:7], 0, 127, 'F')
}

func (bp boardingPass) findColumn() int {
	return bisectionSearch(bp[7:], 0, 7, 'L')
}

func (bp boardingPass) seatID() int {
	return bp.findRow()*8 + bp.findColumn()
}

func bisectionSearch(chars boardingPass, lo, hi int, hiChar rune) int {
	for _, char := range chars {
		mid := (hi - lo + 1) / 2
		if char == hiChar {
			hi -= mid
		} else {
			lo += mid
		}
	}
	return lo // both lo and hi are the same number
}

func Sol05(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	seatIds := make([]int, len(lines))
	for i, line := range lines {
		seatIds[i] = boardingPass(line).seatID()
	}
	sort.Ints(seatIds)

	var missingSeatId int
	for i := 0; i < len(seatIds)-1; i++ {
		if seatIds[i+1]-seatIds[i] == 2 {
			missingSeatId = seatIds[i] + 1
			break
		}
	}

	fmt.Printf("5.1: %d\n5.2: %d\n", seatIds[len(seatIds)-1], missingSeatId)
	return nil
}
