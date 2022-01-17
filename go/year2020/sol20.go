package year2020

import (
	"fmt"
	"math"

	"github.com/dhruvmanila/advent-of-code/go/pkg/matrix"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

const size = 10

var monster = [3]string{
	"                  # ",
	"#    ##    ##    ###",
	" #  #  #  #  #  #   ",
}

type imageTile struct {
	id    int
	image [][]byte // image[row][col]
}

// rotate returns a tile after rotating it once in the clockwise direction.
// This function will not mutate the original image and instead, return a new
// tile with the rotated image. In other words, this function is pure.
func (t *imageTile) rotate() *imageTile {
	size := len(t.image)
	rotated := make([][]byte, size)
	for i := 0; i < size; i++ {
		rotated[i] = make([]byte, size)
	}
	for row := 0; row < size; row++ {
		for col := 0; col < size; col++ {
			rotated[row][col] = t.image[size-1-col][row]
		}
	}
	return &imageTile{id: t.id, image: rotated}
}

// flip will flip the image on the vertical axis. This function will not mutate
// the original image and instead, return a new tile with the flipped image.
// In other words, this function is pure.
func (t *imageTile) flip() *imageTile {
	size := len(t.image)
	flipped := make([][]byte, size)
	for i := 0; i < size; i++ {
		flipped[i] = make([]byte, size)
	}
	for row := 0; row < size; row++ {
		for col := 0; col < size/2; col++ {
			flipped[row][col], flipped[row][size-1-col] = t.image[row][size-1-col], t.image[row][col]
		}
	}
	return &imageTile{id: t.id, image: flipped}
}

// topMatch returns true if the top row of tile t matches the bottom row of
// other tile, false otherwise.
func (t *imageTile) topMatch(other *imageTile) bool {
	size := len(t.image)
	for i, cell := range t.image[0] {
		if cell != other.image[size-1][i] {
			return false
		}
	}
	return true
}

// leftMatch returns true if the leftmost column of tile t matches the rightmost
// column of other tile, false otherwise.
func (t *imageTile) leftMatch(other *imageTile) bool {
	size := len(t.image)
	for i, row := range t.image {
		if row[0] != other.image[i][size-1] {
			return false
		}
	}
	return true
}

func (t *imageTile) String() string {
	var s string
	for _, row := range t.image {
		for _, char := range row {
			s += string(char)
		}
		s += "\n"
	}
	return s
}

func search(tiles []*imageTile) *matrix.Dense[*imageTile] {
	// gridSize is the size of the main image. It is calculated based on the
	// total possible tiles which includes all the rotations and flips.
	gridSize := int(math.Sqrt(float64(len(tiles) / 8)))

	// grid is the main image matrix of (gridSize x gridSize).
	grid := matrix.NewDense[*imageTile](gridSize, gridSize, nil)

	// visited is a set of image ids which have been visited.
	visited := set.New[int]()

	// Core loop which runs the backtracking algorithm to assemble the image.
	// row and col are zero-based index values for the main image where (0, 0)
	// points to the top left corner and (gridSize-1, gridSize-1) is the bottom
	// right corner.
	//
	// This returns a boolean value indicating whether we have found the
	// solution or not.
	var loop func(row, col int) bool
	loop = func(row, col int) bool {
		// There's no need to check whether the col is equal to the grid size
		// because we're going in left-to-right, top-to-bottom manner. So, the
		// order for a 2x2 will be (0, 0), (0, 1), (1, 0), (1, 1) and then this
		// will be called with (2, 0).
		if row == gridSize {
			return true
		}
		for _, tile := range tiles {
			if !visited.Contains(tile.id) {
				// If we're not on the first row, then check if the top row of
				// the current tile matches the bottom row of the tile right
				// above the current position.
				if row > 0 && !tile.topMatch(grid.At(row-1, col)) {
					continue
				}
				// If we're not on the first column, then check if the leftmost
				// column of the current tile matches the rightmost column of
				// the tile left to the current position.
				if col > 0 && !tile.leftMatch(grid.At(row, col-1)) {
					continue
				}
				// We found a possible tile for the current position.
				grid.Set(row, col, tile)
				visited.Add(tile.id)

				var finished bool
				if col == gridSize-1 {
					finished = loop(row+1, 0)
				} else {
					finished = loop(row, col+1)
				}
				if finished {
					return true
				}

				// We're backtracking, so remove the visited tile.
				visited.Remove(tile.id)
			}
		}
		return false
	}

	// Start the search loop from top left corner going left-to-right,
	// top-to-bottom.
	loop(0, 0)

	return grid
}

// removeFrames will remove all the edges (top, bottom, left, right) from each
// individual image in the grid.
func removeFrames(grid *matrix.Dense[*imageTile]) *imageTile {
	imageSize := (grid.Rows * size) - (grid.Rows * 2)
	image := make([][]byte, 0, imageSize)
	for row := 0; row < grid.Rows*size; row++ {
		// Skip the top and bottom row from every image in the grid.
		if row%size == 0 || row%size == size-1 {
			continue
		}
		imageRow := make([]byte, 0, imageSize)
		for col := 0; col < grid.Cols*size; col++ {
			// Skip the leftmost and rightmost column from every image in the grid.
			if col%size == 0 || col%size == size-1 {
				continue
			}
			imageRow = append(imageRow, grid.At(row/size, col/size).image[row%size][col%size])
		}
		image = append(image, imageRow)
	}
	return &imageTile{image: image}
}

// computeRoughness is used to return the habitat's water roughness by finding
// all the sea monsters in the given water image.
func computeRoughness(image *imageTile) int {
	var found bool
	imageSize := len(image.image)
	monsterHeight, monsterWidth := len(monster), len(monster[0])
	// Try every possible rotations and flips for the image to find the correct
	// orientation which contains the sea monsters.
MainLoop:
	for rotations := 0; rotations < 4; rotations++ {
		image = image.rotate()
		for flips := 0; flips < 2; flips++ {
			image = image.flip()
			for row := 0; row+monsterHeight-1 < imageSize; row++ {
			ColumnLoop:
				for col := 0; col+monsterWidth-1 < imageSize; col++ {
					// Consider a monster with its top-left corner at (row, col).
					for dy := 0; dy < monsterHeight; dy++ {
						for dx := 0; dx < monsterWidth; dx++ {
							if monster[dy][dx] == '#' && image.image[row+dy][col+dx] != '#' {
								// This position is not part of the monster
								// starting at (row, col).
								continue ColumnLoop
							}
						}
					}
					// We found a monster with its top-left corner at (row, col).
					found = true
					for dy := 0; dy < monsterHeight; dy++ {
						for dx := 0; dx < monsterWidth; dx++ {
							if monster[dy][dx] == '#' {
								image.image[row+dy][col+dx] = 'O'
							}
						}
					}
				}
			}
			// We found the sea monsters for this orientation.
			if found {
				break MainLoop
			}
		}
	}

	roughness := 0
	for y, row := range image.image {
		for x, char := range row {
			if char == '#' {
				roughness++
			}
			// Convert the image to only contain the sea monsters.
			if char != 'O' {
				image.image[y][x] = '.'
			}
		}
	}
	// fmt.Println(image)
	return roughness
}

func Sol20(input string) error {
	sections, err := util.ReadSections(input)
	if err != nil {
		return err
	}

	// tiles contains all the tiles with the four rotations and two flips.
	tiles := make([]*imageTile, 0, len(sections)*8) // N * 8
	for _, section := range sections {
		image := make([][]byte, size)
		for i, row := range section[1:] {
			image[i] = []byte(row)
		}
		tile := &imageTile{
			id:    util.MustAtoi(section[0][5:9]),
			image: image,
		}
		for f := 0; f < 2; f++ {
			for r := 0; r < 4; r++ {
				tiles = append(tiles, tile)
				tile = tile.rotate()
			}
			tile = tile.flip()
		}
	}

	// grid is a matrix containing the entire image including the edges/frames.
	grid := search(tiles)

	// product is the product of ids of the four corner images.
	product := grid.At(0, 0).id *
		grid.At(0, grid.Cols-1).id *
		grid.At(grid.Rows-1, 0).id *
		grid.At(grid.Rows-1, grid.Cols-1).id

	// Remove the frames from the grid and form an image as a string slice.
	// We could use matrix.Matrix again but comparing and slicing operation
	// on a string will be much easier.
	image := removeFrames(grid)

	fmt.Printf("20.1: %d\n20.2: %d\n", product, computeRoughness(image))
	return nil
}
