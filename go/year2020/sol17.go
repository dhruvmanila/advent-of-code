package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

func getNeighbours3D(cube [3]int) [][3]int {
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

func getNeighbours4D(cube [4]int) [][4]int {
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

func executeCycle3D(activeCubes set.Set[[3]int], n int) int {
	if n == 0 {
		return activeCubes.Len()
	}
	nextActiveCubes := set.New[[3]int]()
	inactiveActiveNeighbours := counter.New[[3]int]()
	activeCubes.ForEach(func(cube [3]int) {
		activeNeighbours := 0
		for _, neighbour := range getNeighbours3D(cube) {
			if activeCubes.Contains(neighbour) {
				activeNeighbours++
			} else {
				inactiveActiveNeighbours.Increment(neighbour)
			}
		}
		switch activeNeighbours {
		case 2, 3:
			nextActiveCubes.Add(cube)
		}
	})
	inactiveActiveNeighbours.ForEach(func(item [3]int, count int) {
		if count == 3 {
			nextActiveCubes.Add(item)
		}
	})
	return executeCycle3D(nextActiveCubes, n-1)
}

func executeCycle4D(activeCubes set.Set[[4]int], n int) int {
	if n == 0 {
		return activeCubes.Len()
	}
	nextActiveCubes := set.New[[4]int]()
	inactiveActiveNeighbours := counter.New[[4]int]()
	activeCubes.ForEach(func(cube [4]int) {
		activeNeighbours := 0
		for _, neighbour := range getNeighbours4D(cube) {
			if activeCubes.Contains(neighbour) {
				activeNeighbours++
			} else {
				inactiveActiveNeighbours.Increment(neighbour)
			}
		}
		switch activeNeighbours {
		case 2, 3:
			nextActiveCubes.Add(cube)
		}
	})
	inactiveActiveNeighbours.ForEach(func(item [4]int, count int) {
		if count == 3 {
			nextActiveCubes.Add(item)
		}
	})
	return executeCycle4D(nextActiveCubes, n-1)
}

func parseInitialCubes3D(state []string) set.Set[[3]int] {
	activeCubes := set.New[[3]int]()
	for y, line := range state {
		for x, char := range line {
			if char == '#' {
				activeCubes.Add([3]int{x, y, 0})
			}
		}
	}
	return activeCubes
}

func parseInitialCubes4D(state []string) set.Set[[4]int] {
	activeCubes := set.New[[4]int]()
	for y, line := range state {
		for x, char := range line {
			if char == '#' {
				activeCubes.Add([4]int{x, y, 0})
			}
		}
	}
	return activeCubes
}

func Sol17(input string) (string, error) {
	lines := util.ReadLines(input)

	count1 := executeCycle3D(parseInitialCubes3D(lines), 6)
	count2 := executeCycle4D(parseInitialCubes4D(lines), 6)

	return fmt.Sprintf("17.1: %d\n17.2: %d\n", count1, count2), nil
}
