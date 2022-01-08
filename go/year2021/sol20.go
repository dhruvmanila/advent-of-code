package year2021

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geom"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type image struct {
	// pixels is a set of position containing all the light pixels.
	pixels *set.Set

	// bbox represents a bounding box containing the input image without the
	// infinite region.
	bbox *geom.BoundingBox2D

	// inf represents all the pixels in the infinite region.
	inf rune
}

func newImage(lines []string) *image {
	pixels := set.New()
	var minx, maxx, miny, maxy int
	for row, line := range lines {
		for col, pixel := range line {
			if pixel == '#' {
				pixels.Add(position{row, col})
				minx = util.IntMin(minx, col)
				maxx = util.IntMax(maxx, col)
				miny = util.IntMin(miny, row)
				maxy = util.IntMax(maxy, row)
			}
		}
	}
	return &image{
		pixels: pixels,
		bbox:   geom.NewBoundingBox2D(minx, maxx, miny, maxy),
		inf:    '.',
	}
}

// apply is used to apply the algorithm on the image a number of times. Note
// that this will update the image after applying the algorithm.
func (i *image) apply(algorithm string, times int) {
	if times == 0 {
		return
	}

	// By default, all the infinite pixels are black, so when applying the
	// algorithm we would be replacing the dark pixels with the pixel present
	// in the zeroth position (binary 000000000). So, if the algorithm contains
	// a light pixel and a dark pixel at the start and end, then the infinite
	// region of the image will be flickering on and off.
	if algorithm[0] == '#' && algorithm[len(algorithm)-1] == '.' {
		if times%2 == 0 {
			i.inf = '.'
		} else {
			i.inf = '#'
		}
	}

	// As the algorithm, needs to change all the pixels simultaneously, we will
	// apply the changes in the new image from referencing the old image.
	newImage := set.New()
	var minx, maxx, miny, maxy int

	// Loop over all the pixels in the bounding box including 2 extra rows and
	// columns from all 4 sides.
	for row := i.bbox.MinY - 2; row <= i.bbox.MaxY+2; row++ {
		for col := i.bbox.MinX - 2; col <= i.bbox.MaxX+2; col++ {
			idx := 0
			bit := 8
			for dy := -1; dy <= 1; dy++ {
				for dx := -1; dx <= 1; dx++ {
					y, x := row+dy, col+dx
					// If the position is in the infinite region, then turn on
					// the bit as per the infinite pixel.
					if !i.bbox.Contains(x, y) {
						if i.inf == '#' {
							idx += 1 << bit
						}
						// Here, it's gurranteed that the position is within
						// the image bounds.
					} else if i.pixels.Contains(position{y, x}) {
						idx += 1 << bit
					}
					bit--
				}
			}
			if algorithm[idx] == '#' {
				newImage.Add(position{row, col})
				minx = util.IntMin(minx, col)
				maxx = util.IntMax(maxx, col)
				miny = util.IntMin(miny, row)
				maxy = util.IntMax(maxy, row)
			}
		}
	}

	// Update the image to reflect the new image formed after applying the
	// algorithm once.
	i.pixels = newImage
	i.bbox.MinX = minx
	i.bbox.MaxX = maxx
	i.bbox.MinY = miny
	i.bbox.MaxY = maxy

	i.apply(algorithm, times-1)
}

func (i *image) String() string {
	var s string
	for row := i.bbox.MinY - 2; row <= i.bbox.MaxY+2; row++ {
		for col := i.bbox.MinX - 2; col <= i.bbox.MaxX+2; col++ {
			if i.pixels.Contains(position{row, col}) {
				s += "#"
			} else {
				s += "."
			}
		}
		s += "\n"
	}
	return s
}

func Sol20(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	// algorithm is the image enhancement algorithm string.
	algorithm := lines[0]

	// image is a set of position where all the light pixels exists.
	image := newImage(lines[2:])

	image.apply(algorithm, 2)
	fmt.Printf("20.1: %d\n", image.pixels.Len())

	image.apply(algorithm, 48)
	fmt.Printf("20.2: %d\n", image.pixels.Len())

	return nil
}
