package year2021

import (
	"errors"
	"fmt"
	"regexp"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geometry"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var stepRegex = regexp.MustCompile(
	`^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$`,
)

// rebootStep contains information about a single reboot step.
type rebootStep struct {
	state  bool
	cuboid *geometry.BoundingBox3D
}

// filterSteps is used to filter the steps to include only the ones which
// fall within the range as needed for part one.
func filterSteps(steps []*rebootStep) []*rebootStep {
	var filtered []*rebootStep
	for _, step := range steps {
		// Is this more readable than an if statement?
		switch {
		case step.cuboid.MinX < -50 || step.cuboid.MaxX > 50:
			fallthrough
		case step.cuboid.MinY < -50 || step.cuboid.MaxY > 50:
			fallthrough
		case step.cuboid.MinZ < -50 || step.cuboid.MaxZ > 50:
			continue
		}
		filtered = append(filtered, step)
	}
	return filtered
}

func reboot(rebootSteps []*rebootStep) int {
	var steps []*rebootStep

	for _, rs := range rebootSteps {
		for _, s := range steps {
			// Inclusion-Exclusion principle to obtain a union of two sets:
			//
			//   |A ∪ B| = |A| + |B| - |A ∩ B|
			//
			// If they intersect, then the intersect cuboid will be added with
			// its state flipped w.r.t. the reboot step (rs).
			if intersect := rs.cuboid.Intersection(s.cuboid); intersect != nil {
				// We're modifying the slice while iterating which is ok here
				// as range cannot see the updated slice. This would lead to
				// an infinite loop if using the three part for loop.
				steps = append(steps, &rebootStep{
					state:  !s.state,
					cuboid: intersect,
				})
			}
		}
		if rs.state {
			steps = append(steps, rs)
		}
	}

	total := 0
	for _, s := range steps {
		sign := 1
		if !s.state {
			sign = -1
		}
		total += s.cuboid.Volume() * sign
	}
	return total
}

func parseSteps(lines []string) ([]*rebootStep, error) {
	steps := make([]*rebootStep, len(lines))
	for i, line := range lines {
		matches := stepRegex.FindStringSubmatch(line)
		if len(matches) != 8 {
			return nil, errors.New("invalid reboot step")
		}
		steps[i] = &rebootStep{
			state: matches[1] == "on",
			cuboid: geometry.NewBoundingBox3D(
				util.Atoi(matches[2]),
				util.Atoi(matches[3]),
				util.Atoi(matches[4]),
				util.Atoi(matches[5]),
				util.Atoi(matches[6]),
				util.Atoi(matches[7]),
			),
		}
	}
	return steps, nil
}

func Sol22(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	steps, err := parseSteps(lines)
	if err != nil {
		return err
	}

	fmt.Printf("22.1: %d\n22.2: %d\n", reboot(filterSteps(steps)), reboot(steps))
	return nil
}
