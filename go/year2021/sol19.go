package year2021

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

const (
	xAxis = iota
	yAxis
	zAxis
)

type axisInfo struct {
	axis int
	sign int
	diff int
}

type Scanner struct {
	id      int
	beacons [][3]int
}

func (s *Scanner) edgesTo(other *Scanner, axis int) *axisInfo {
	for _, otherAxis := range []int{xAxis, yAxis, zAxis} {
		for _, sign := range []int{-1, 1} {
			diffCounter := counter.New[int]()
			for _, srcBeacon := range s.beacons {
				for _, otherBeacon := range other.beacons {
					diffCounter.Increment(srcBeacon[axis] - otherBeacon[otherAxis]*sign)
				}
			}
			item := diffCounter.MostCommon()
			if diffCounter.Get(item) >= 12 {
				return &axisInfo{axis: otherAxis, sign: sign, diff: item}
			}
		}
	}
	return nil
}

func (s *Scanner) xEdgesTo(other *Scanner) *axisInfo {
	return s.edgesTo(other, xAxis)
}

func (s *Scanner) yEdgesTo(other *Scanner) *axisInfo {
	return s.edgesTo(other, yAxis)
}

func (s *Scanner) zEdgesTo(other *Scanner) *axisInfo {
	return s.edgesTo(other, zAxis)
}

// parseSections parses the input into a map of scanner id to scanner.
func parseSections(sections [][]string) map[int]*Scanner {
	scannersById := make(map[int]*Scanner, len(sections))
	for _, section := range sections {
		var id int
		fmt.Sscanf(section[0], "--- scanner %d ---", &id)
		beacons := make([][3]int, 0, len(section)-1)
		for _, line := range section[1:] {
			var x, y, z int
			fmt.Sscanf(line, "%d,%d,%d", &x, &y, &z)
			beacons = append(beacons, [3]int{x, y, z})
		}
		scannersById[id] = &Scanner{id: id, beacons: beacons}
	}
	return scannersById
}

func compute(scannersById map[int]*Scanner) (int, int) {
	// scannerIds is a set of all the scanner ids.
	scannerIds := set.NewWithSize[int](len(scannersById))
	for id := range scannersById {
		scannerIds.Add(id)
	}

	// beacons is a set of all the beacon positions.
	beacons := set.NewFromSlice(scannersById[0].beacons)

	scannerPositions := make([][3]int, 0, len(scannersById))
	scannerPositions = append(scannerPositions, [3]int{0, 0, 0})

	scanners := stack.New[*Scanner]()
	scanners.Push(scannersById[0])
	scannerIds.Remove(0)

	for !scanners.IsEmpty() {
		scanner, _ := scanners.Pop()

		xEdges := make(map[int]*axisInfo)
		scannerIds.ForEach(func(id int) {
			if info := scanner.xEdgesTo(scannersById[id]); info != nil {
				xEdges[id] = info
			}
		})

		yEdges := make(map[int]*axisInfo)
		for id := range xEdges {
			if info := scanner.yEdgesTo(scannersById[id]); info != nil {
				yEdges[id] = info
			}
		}

		zEdges := make(map[int]*axisInfo)
		for id := range xEdges {
			if info := scanner.zEdgesTo(scannersById[id]); info != nil {
				zEdges[id] = info
			}
		}

		for id := range xEdges {
			dx, dy, dz := xEdges[id].diff, yEdges[id].diff, zEdges[id].diff
			scannerPositions = append(scannerPositions, [3]int{dx, dy, dz})

			nextScanner := scannersById[id]
			normalizedBeacons := make([][3]int, 0, len(nextScanner.beacons))
			for _, beacon := range nextScanner.beacons {
				normalizedBeacons = append(normalizedBeacons, [3]int{
					beacon[xEdges[id].axis]*xEdges[id].sign + dx,
					beacon[yEdges[id].axis]*yEdges[id].sign + dy,
					beacon[zEdges[id].axis]*zEdges[id].sign + dz,
				})
			}
			nextScanner.beacons = normalizedBeacons

			beacons.Add(nextScanner.beacons...)
			scanners.Push(nextScanner)
			scannerIds.Remove(id)
		}
	}

	maxDistance := 0
	for idx, s1 := range scannerPositions {
		for _, s2 := range scannerPositions[idx+1:] {
			maxDistance = util.Max(
				util.Abs(s1[xAxis]-s2[xAxis])+
					util.Abs(s1[yAxis]-s2[yAxis])+
					util.Abs(s1[zAxis]-s2[zAxis]),
				maxDistance,
			)
		}
	}

	return beacons.Len(), maxDistance
}

func Sol19(input string) (string, error) {
	sections, err := util.ReadSections(input)
	if err != nil {
		return "", err
	}

	scannersById := parseSections(sections)
	beacons, maxDistance := compute(scannersById)

	return fmt.Sprintf("19.1: %d\n19.2: %d\n", beacons, maxDistance), nil
}
