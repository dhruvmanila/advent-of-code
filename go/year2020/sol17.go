package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

func getNeighbours3D(e interface{}) [][3]int {
	cube := e.([3]int)
	neighbours := make([][3]int, 0, 26)
	for dx := -1; dx <= 1; dx++ {
		for dy := -1; dy <= 1; dy++ {
			for dz := -1; dz <= 1; dz++ {
				if dx == 0 && dy == 0 && dz == 0 {
					continue
				}
				neighbours = append(neighbours, [3]int{
					cube[0] + dx,
					cube[1] + dy,
					cube[2] + dz,
				})
			}
		}
	}
	return neighbours
}

func getNeighbours4D(e interface{}) [][4]int {
	cube := e.([4]int)
	neighbours := make([][4]int, 0, 80)
	for dx := -1; dx <= 1; dx++ {
		for dy := -1; dy <= 1; dy++ {
			for dz := -1; dz <= 1; dz++ {
				for dw := -1; dw <= 1; dw++ {
					if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
						continue
					}
					neighbours = append(neighbours, [4]int{
						cube[0] + dx,
						cube[1] + dy,
						cube[2] + dz,
						cube[3] + dw,
					})
				}
			}
		}
	}
	return neighbours
}

func executeCycle(activeCubes *set.Set, n, dimensions int) int {
	if n == 0 {
		return activeCubes.Len()
	}
	nextActiveCubes := set.New()
	inactiveActiveNeighbours := counter.New()
	activeCubes.ForEach(func(cube interface{}) {
		activeNeighbours := 0
		switch dimensions {
		case 3:
			for _, neighbour := range getNeighbours3D(cube) {
				if activeCubes.Contains(neighbour) {
					activeNeighbours++
				} else {
					inactiveActiveNeighbours.Increment(neighbour)
				}
			}
		case 4:
			for _, neighbour := range getNeighbours4D(cube) {
				if activeCubes.Contains(neighbour) {
					activeNeighbours++
				} else {
					inactiveActiveNeighbours.Increment(neighbour)
				}
			}
		}
		switch activeNeighbours {
		case 2, 3:
			nextActiveCubes.Add(cube)
		}
	})
	inactiveActiveNeighbours.ForEach(func(item interface{}, count int) {
		if count == 3 {
			nextActiveCubes.Add(item)
		}
	})
	return executeCycle(nextActiveCubes, n-1, dimensions)
}

func parseInitialCubes(state []string, dimensions int) *set.Set {
	activeCubes := set.New()
	for y, line := range state {
		for x, char := range line {
			if char == '#' {
				switch dimensions {
				case 3:
					activeCubes.Add([3]int{x, y, 0})
				case 4:
					activeCubes.Add([4]int{x, y, 0, 0})
				}
			}
		}
	}
	return activeCubes
}

func Sol17(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	cubes3D := parseInitialCubes(lines, 3)
	cubes4D := parseInitialCubes(lines, 4)
	fmt.Printf("17.1: %d\n17.2: %d\n", executeCycle(cubes3D, 6, 3), executeCycle(cubes4D, 6, 4))
	return nil
}
