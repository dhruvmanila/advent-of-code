package year2021

import (
	"errors"
	"fmt"
	"regexp"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var lineSegmentRegex = regexp.MustCompile(`(\d+),(\d+) -> (\d+),(\d+)`)

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
		x1 := util.MustAtoi(matches[1])
		y1 := util.MustAtoi(matches[2])
		x2 := util.MustAtoi(matches[3])
		y2 := util.MustAtoi(matches[4])
		lineSegments[i] = newLineSegment(point{x1, y1}, point{x2, y2})
	}
	return lineSegments, nil
}

func Sol05(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	lineSegments, err := parseLines(lines)
	if err != nil {
		return "", err
	}

	// counter1 and counter2 represents the counter for the first and second
	// part of the puzzle respectively.
	counter1, counter2 := counter.New[point](), counter.New[point]()

	for _, ls := range lineSegments {
		for _, p := range ls.allPoints() {
			switch ls.orientation {
			case horizontal, vertical:
				counter1.Increment(p)
				counter2.Increment(p)
			case diagonal:
				counter2.Increment(p)
			}
		}
	}

	var count1, count2 int
	counter1.ForEach(func(_ point, count int) {
		if count >= 2 {
			count1++
		}
	})
	counter2.ForEach(func(_ point, count int) {
		if count >= 2 {
			count2++
		}
	})

	return fmt.Sprintf("5.1: %d\n5.2: %d\n", count1, count2), nil
}
