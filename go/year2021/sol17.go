package year2021

import (
	"bytes"
	"fmt"
	"os"
	"regexp"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geom"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var targetAreaRegex = regexp.MustCompile(
	`^target area: x=(\d+)\.\.(\d+), y=(-?\d+)\.\.(-?\d+)$`,
)

type probe struct {
	point

	vx int
	vy int
}

func newProbe(vx, vy int) *probe {
	return &probe{
		point: point{x: 0, y: 0},
		vx:    vx,
		vy:    vy,
	}
}

func (p *probe) isOutside(target *geom.BoundingBox2D) bool {
	switch {
	case p.vx < 0 && p.x < target.MinX:
		fallthrough
	case p.vx > 0 && p.x > target.MaxX:
		fallthrough
	case p.vy < 0 && p.y < target.MinY:
		return true
	}
	return false
}

func (p *probe) launch(target *geom.BoundingBox2D) (maxHeight int, reached bool) {
	for !p.isOutside(target) {
		p.x += p.vx
		p.y += p.vy
		maxHeight = util.IntMax(maxHeight, p.y)
		if target.Contains(p.x, p.y) {
			reached = true
			break
		}
		switch {
		case p.vx > 0:
			p.vx--
		case p.vx < 0:
			p.vx++
		}
		p.vy--
	}
	return maxHeight, reached
}

func Sol17(input string) error {
	content, err := os.ReadFile(input)
	if err != nil {
		return err
	}
	content = bytes.Trim(content, "\n")

	matches := targetAreaRegex.FindStringSubmatch(string(content))
	if len(matches) != 5 {
		return fmt.Errorf("invalid match: %s", content)
	}

	minx := util.MustAtoi(matches[1])
	maxx := util.MustAtoi(matches[2])
	miny := util.MustAtoi(matches[3])
	maxy := util.MustAtoi(matches[4])
	target := geom.NewBoundingBox2D(minx, maxx, miny, maxy)

	maxHeight := 0
	count := 0
	// Velocity in x direction cannot be negative otherwise we would be
	// moving away from the target. It cannot be 0 as well which would just
	// move the probe in a vertical direction.
	for vx := 1; vx <= maxx; vx++ {
		for vy := miny; vy <= util.AbsInt(miny); vy++ {
			height, reached := newProbe(vx, vy).launch(target)
			if reached {
				maxHeight = util.IntMax(maxHeight, height)
				count++
			}
		}
	}

	fmt.Printf("17.1: %d\n17.2: %d\n", maxHeight, count)
	return nil
}
