package year2021

import (
	"errors"
	"fmt"
	"regexp"
	"strconv"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

var lineSegmentRegex = regexp.MustCompile(`(\d+),(\d+) -> (\d+),(\d+)`)

// point contains information for a point with x and y coordinates.
type point struct {
	x int
	y int
}

type orientation int

const (
	horizontal orientation = iota + 1
	vertical
	diagonal
)

// lineSegment contains information regarding the two endpoints of a line segment.
type lineSegment struct {
	start       point
	end         point
	orientation orientation
}

// newLineSegment is used to construct a lineSegment with the given set of
// points. The order needs to be maintained where the first two argument
// represents the x and y coordinates of the first point and the other two are
// for the second point.
func newLineSegment(start, end point) *lineSegment {
	ls := &lineSegment{start: start, end: end}
	ls.orientation = ls.getOrientation()
	ls.normalizeDirection()
	return ls
}

// getOrientation is used to define the orientation of the line segment.
// Valid orientations are: horizontal, vertical, diagonal.
func (ls *lineSegment) getOrientation() orientation {
	slope, defined := ls.slope()
	if defined {
		switch slope {
		case 0:
			return horizontal
		case 1, -1:
			return diagonal
		default:
			panic(fmt.Sprintf("Invalid slope for line segment: %d", slope))
		}
	} else {
		return vertical
	}
}

// normalizeDirection is used to normalize the direction of a line segment
// from start point to end point for the respective orientation.
//
// Direction for all the orientation:
// - horizontal: left to right
// - vertical, diagonal: bottom to top
func (ls *lineSegment) normalizeDirection() {
	switch ls.orientation {
	case horizontal:
		if ls.start.x > ls.end.x {
			ls.start, ls.end = ls.end, ls.start
		}
	case vertical, diagonal:
		if ls.start.y > ls.end.y {
			ls.start, ls.end = ls.end, ls.start
		}
	}
}

// allPoints is used to get all the points which is on the line segment including
// the endpoints. The points returned are at a unit distance.
func (ls *lineSegment) allPoints() []point {
	var points []point
	switch ls.orientation {
	case horizontal:
		// Include the endpoints as well.
		for x := ls.start.x; x <= ls.end.x; x++ {
			points = append(points, point{x, ls.start.y})
		}
	case vertical:
		// Include the endpoints as well.
		for y := ls.start.y; y <= ls.end.y; y++ {
			points = append(points, point{ls.start.x, y})
		}
	case diagonal:
		slope, _ := ls.slope()
		for i := 0; i <= ls.end.y-ls.start.y; i++ {
			points = append(points, point{ls.start.x + (slope * i), ls.start.y + i})
		}
	}
	return points
}

// slope is used to calculate the slope of a line segment.
//
// Assumption: The line segment is either horizontal, vertical or diagonal and
// thus, an integer is used to represent the value instead of a float.
func (ls *lineSegment) slope() (int, bool) {
	dx := ls.end.x - ls.start.x
	dy := ls.end.y - ls.start.y
	if dx == 0 || dy == 0 {
		// If dx == 0, then the slope is 0. However, if the lines are vertical,
		// then the slope will be undefined.
		return 0, dx != 0
	}
	return dy / dx, true
}

// pointCounter is a map from point to an int representing the number of times
// the point was added.
type pointCounter map[point]int

// add is used to add multiple points to the pointCounter. If the point exists,
// then increase the count, otherwise add the point with a default count of 1.
func (pc pointCounter) add(pts ...point) {
	for _, p := range pts {
		if _, exist := pc[p]; exist {
			pc[p]++
		} else {
			pc[p] = 1
		}
	}
}

// where is used to return the number of points where the count is equal to or
// greater than the given value.
func (pc pointCounter) where(c int) int {
	quant := 0
	for _, count := range pc {
		if count >= c {
			quant++
		}
	}
	return quant
}

// parseLines will convert the given lines to the respective lineSegment object.
// The structure of the line is parsed using the lineSegmentRegex which if of
// the form: "1,2 -> 3,4"
func parseLines(lines []string) ([]*lineSegment, error) {
	lineSegments := make([]*lineSegment, len(lines))
	for i, line := range lines {
		matches := lineSegmentRegex.FindStringSubmatch(line)
		if len(matches) != 5 {
			return nil, errors.New("regexp: invalid match")
		}
		// Convert all the matches from string to an integer. The error will be
		// returned for the last invalid conversion.
		x1, err := strconv.Atoi(matches[1])
		y1, err := strconv.Atoi(matches[2])
		x2, err := strconv.Atoi(matches[3])
		y2, err := strconv.Atoi(matches[4])
		if err != nil {
			return nil, err
		}
		lineSegments[i] = newLineSegment(point{x1, y1}, point{x2, y2})
	}
	return lineSegments, nil
}

func Sol5(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	lineSegments, err := parseLines(lines)
	if err != nil {
		return err
	}

	// counter1 and counter2 represents the counter for the first and second
	// part of the puzzle respectively.
	counter1, counter2 := make(pointCounter), make(pointCounter)

	for _, ls := range lineSegments {
		pts := ls.allPoints()
		switch ls.orientation {
		case horizontal, vertical:
			counter1.add(pts...)
			counter2.add(pts...)
		case diagonal:
			counter2.add(pts...)
		}
	}

	fmt.Printf("5.1: %d\n5.2: %d\n", counter1.where(2), counter2.where(2))
	return nil
}
