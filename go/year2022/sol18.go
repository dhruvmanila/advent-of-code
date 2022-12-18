package year2022

import (
	"fmt"
	"math"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geom"
	"github.com/dhruvmanila/advent-of-code/go/pkg/queue"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

// lavaDroplet represents a single lava droplet.
type lavaDroplet struct {
	// points is a set of location in a 3D grid representing a 1x1x1 cube.
	// All of the points combined represents the droplet.
	points set.Set[geom.Point3D[int]]

	// bbox is the bounding box encompassing all the points.
	bbox *geom.BoundingBox3D
}

// SurfaceArea returns the total surface area and the exterior surface area
// of the lava droplet.
func (d *lavaDroplet) SurfaceArea() (totalSurfaceArea, exteriorSurfaceArea int) {
	// edgePoints is a set of points located on the edge of the droplet.
	// This will include the interior edge as well where the air pockets
	// are formed. A point is considered to be on the edge if atleast one
	// of the surface is exposed.
	edgePoints := set.New[geom.Point3D[int]]()

	d.points.ForEach(func(p geom.Point3D[int]) {
		exposed := false
		for _, neighbor := range p.Neighbors() {
			if !d.points.Contains(neighbor) {
				totalSurfaceArea++
				exposed = true
			}
		}
		if exposed {
			edgePoints.Add(p)
		}
	})

	visited := set.New[geom.Point3D[int]]()
	queue := queue.New(geom.Point3D[int]{X: d.bbox.MinX, Y: d.bbox.MinY, Z: d.bbox.MinZ})

	// Use BFS starting from the top left corner of the bounding box and go
	// through each point upto the edge of the droplet. Now, as there's no
	// way to enter the air pocket which is in the interior part of the droplet,
	// the search won't go there.
	for {
		p, ok := queue.Dequeue()
		if !ok {
			break // empty queue
		}
		if visited.Contains(p) {
			continue
		}
		visited.Add(p)
		for _, neighbor := range p.Neighbors() {
			if !d.bbox.Contains(neighbor.X, neighbor.Y, neighbor.Z) {
				continue
			}
			if edgePoints.Contains(neighbor) {
				exteriorSurfaceArea++
			} else {
				queue.Enqueue(neighbor)
			}
		}
	}

	return totalSurfaceArea, exteriorSurfaceArea
}

// surfaceArea returns the total surface area of the given set of points.
func surfaceArea(points set.Set[geom.Point3D[int]]) int {
	surfaceArea := 0
	points.ForEach(func(p geom.Point3D[int]) {
		for _, neighbor := range p.Neighbors() {
			if !points.Contains(neighbor) {
				surfaceArea++
			}
		}
	})
	return surfaceArea
}

// SurfaceAreaFloodFill returns the total surface area and the exterior surface
// area of the laval droplet. This uses the flood fill algorithm to find out
// the areas.
func (d *lavaDroplet) SurfaceAreaFloodFill() (totalSurfaceArea, exteriorSurfaceArea int) {
	allPoints := set.NewWithSize[geom.Point3D[int]](
		(d.bbox.MaxX - d.bbox.MinX + 1) * (d.bbox.MaxY - d.bbox.MinY + 1) * (d.bbox.MaxZ - d.bbox.MinZ + 1),
	)
	for x := d.bbox.MinX; x <= d.bbox.MaxX; x++ {
		for y := d.bbox.MinY; y <= d.bbox.MaxY; y++ {
			for z := d.bbox.MinZ; z <= d.bbox.MaxZ; z++ {
				allPoints.Add(geom.Point3D[int]{X: x, Y: y, Z: z})
			}
		}
	}

	remaining := allPoints.Difference(d.points)
	queue := queue.New(geom.Point3D[int]{X: d.bbox.MinX, Y: d.bbox.MinY, Z: d.bbox.MinZ})

	for {
		p, ok := queue.Dequeue()
		if !ok {
			break // empty queue
		}
		if remaining.Contains(p) {
			remaining.Remove(p)
		} else {
			continue
		}
		for _, neighbor := range p.Neighbors() {
			queue.Enqueue(neighbor)
		}
	}

	totalSurfaceArea = surfaceArea(d.points)
	return totalSurfaceArea, totalSurfaceArea - surfaceArea(remaining)
}

func parseDropletPoints(lines []string) (*lavaDroplet, error) {
	points := set.NewWithSize[geom.Point3D[int]](len(lines))
	minx, maxx := math.MaxInt, math.MinInt
	miny, maxy := math.MaxInt, math.MinInt
	minz, maxz := math.MaxInt, math.MinInt

	var x, y, z int
	for idx, line := range lines {
		_, err := fmt.Sscanf(line, "%d,%d,%d", &x, &y, &z)
		if err != nil {
			return nil, fmt.Errorf("line %d: %q: %w", idx, line, err)
		}
		minx, maxx = util.Min(minx, x), util.Max(maxx, x)
		miny, maxy = util.Min(miny, y), util.Max(maxy, y)
		minz, maxz = util.Min(minz, z), util.Max(maxz, z)
		points.Add(geom.Point3D[int]{X: x, Y: y, Z: z})
	}

	return &lavaDroplet{
		points: points,
		bbox:   geom.NewBoundingBox3D(minx-1, maxx+1, miny-1, maxy+1, minz-1, maxz+1),
	}, nil
}

func Sol18(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	d, err := parseDropletPoints(lines)
	if err != nil {
		return "", err
	}

	totalSurfaceArea, exteriorSurfaceArea := d.SurfaceArea()
	// totalSurfaceArea, exteriorSurfaceArea := d.SurfaceAreaFloodFill()

	return fmt.Sprintf("18.1: %d\n18.2: %d\n", totalSurfaceArea, exteriorSurfaceArea), nil
}
