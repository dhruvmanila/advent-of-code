package year2021

import (
	"errors"
	"fmt"
	"regexp"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geom"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var stepRegex = regexp.MustCompile(
	`^(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)$`,
)

// rebootStep contains information about a single reboot step.
type rebootStep struct {
	state  bool
	cuboid *geom.BoundingBox3D
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

// Algorithm explanation:
//
// This is done in 2D but the same logic applies in 3D. There are two cases to
// help understand the algorithm:
//
// 1. A (on) + B (on) + C (on)
//
//              B(on)
//             ┌───────────────────┐
//             │+                  │
//      A(on)  │                   │
//     ┌───────┼────────┐          │
//     │+      │D    + +│          │
//     │       │     -  │          │
//     │       │        │          │
//     │  ┌────┼────────┼──────┐   │
//     │  │E   │F  + + +│G  + +│   │
//     │  │    │ + - - -│     -│   │
//     │  │+ + └────────┼──────┼───┘
//     │  │-            │      │
//     └──┼─────────────┘      │
//        │                    │
//        │                    │
//   C(on)│+                   │
//        └────────────────────┘
//
// Here, D, E, F and G are the cubes at an intersection and plus (+) and minus (-)
// represent that the region is being added or removed.
//
// 1. Cube A is added as it is on and is represented by a plus (+) in all the
//    cubes in A which are A, D, E, G.
// 2. Cube B is added as it is on and is represented by a plus (+) in all the
//    cubes in B which are B, E, F, G. Here, D and F have been added twice, so
//    we need to remove them. This is done using the inclusion-exlusion principle
//    by adding the intersection cube (D+F) of A and B with an opposite state
//    w.r.t. the cube A.
// 3. Cube C is added as it is on and is represented by a plus (+) in all the
//    cubes in C which are C, E, F, G. Here, E, F and G have been added twice.
//    We will loop over all the cubes available before this step which includes
//    A, B and D+F (intersection cube between A and B) and use the same principle.
//    1. E and F will be subtracted (opposite state of A) as E+F is the
//       intersection cube between C and A.
//    2. F and G will be subtracted (opposite state of B) as F+G is the
//       intersection cube between C and B.
//    3. Now, this is the special case where C intersects with the intersection
//       cube D+F which, if you remember, has an opposite state w.r.t. cube B
//       meaning that the intersection cube is to be subtracted. So, cube F,
//       which is an intersection between C and D+F, will be added (opposite
//       state of intersection cubee D+F).
//
// Cancelling out all the plus (+) and minus (-), what we get is all the region
// being added only once which is what we want.
//
// 2. A (on) + B (on) + C (off)
//
//              B(on)
//             ┌───────────────────┐
//             │+                  │
//      A(on)  │                   │
//     ┌───────┼────────┐          │
//     │+      │D    + +│          │
//     │       │       -│          │
//     │       │        │          │
//     │  ┌────┼────────┼──────┐   │
//     │  │E   │F  - + +│G    +│   │
//     │  │    │   + - -│     -│   │
//     │  │+   └────────┼──────┼───┘
//     │  │-            │      │
//     └──┼─────────────┘      │
//        │                    │
//        │                    │
//  C(off)│                    │
//        └────────────────────┘
//
// First two steps where A and B are being added is the same as before.
// Here, C should not be added as it is an off cube, but we still need to
// perform the intersection and add opposite state step for all the previous
// cubes. After performing the same steps as 3.1, 3.2 and 3.3 from before, we
// are left with the above state of plus (+) and minus (-). After cancelling
// them all out, what we get is the region which is all on.
//
// Summary:
// 1. A(on) + B(on)  = |A| + |B| - |A ∩ B| (inclusion-exlusion principle)
// 2. A(on) + B(off) = |A|       - |A ∩ B|
func reboot(rebootSteps []*rebootStep) int {
	var steps []*rebootStep

	for _, rs := range rebootSteps {
		for _, s := range steps {
			// If they intersect, then the intersect cuboid will be added with
			// its state flipped w.r.t. the step (s).
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
			cuboid: geom.NewBoundingBox3D(
				util.MustAtoi(matches[2]),
				util.MustAtoi(matches[3]),
				util.MustAtoi(matches[4]),
				util.MustAtoi(matches[5]),
				util.MustAtoi(matches[6]),
				util.MustAtoi(matches[7]),
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
