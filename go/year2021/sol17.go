package year2021

import (
	"bytes"
	"fmt"
	"os"
	"regexp"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

var targetAreaRegex = regexp.MustCompile(
	`^target area: x=(\d+)\.\.(\d+), y=(-?\d+)\.\.(-?\d+)$`,
)

type boundingBox struct {
	minx int
	maxx int
	miny int
	maxy int
}

func newBoundingBox(minx, maxx, miny, maxy int) *boundingBox {
	return &boundingBox{
		minx: minx,
		maxx: maxx,
		miny: miny,
		maxy: maxy,
	}
}

func (bb *boundingBox) contains(p point) bool {
	return bb.minx <= p.x && p.x <= bb.maxx && bb.miny <= p.y && p.y <= bb.maxy
}

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

func (p *probe) isOutside(target *boundingBox) bool {
	switch {
	case p.vx < 0 && p.x < target.minx:
		fallthrough
	case p.vx > 0 && p.x > target.maxx:
		fallthrough
	case p.vy < 0 && p.y < target.miny:
		return true
	}
	return false
}

func (p *probe) launch(target *boundingBox) (maxHeight int, reached bool) {
	for !p.isOutside(target) {
		p.x += p.vx
		p.y += p.vy
		maxHeight = util.IntMax(maxHeight, p.y)
		if target.contains(p.point) {
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

	bb := newBoundingBox(
		util.Atoi(matches[1]),
		util.Atoi(matches[2]),
		util.Atoi(matches[3]),
		util.Atoi(matches[4]),
	)

	maxHeight := 0
	count := 0
	for vx := -500; vx <= 500; vx++ {
		for vy := -500; vy <= 500; vy++ {
			height, reached := newProbe(vx, vy).launch(bb)
			if reached {
				maxHeight = util.IntMax(maxHeight, height)
				count++
			}
		}
	}

	fmt.Printf("17.1: %d\n17.2: %d\n", maxHeight, count)
	return nil
}
