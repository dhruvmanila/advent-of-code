package year2022

import (
	"container/heap"
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/geom"
	"github.com/dhruvmanila/advent-of-code/go/pkg/matrix"
	"github.com/dhruvmanila/advent-of-code/go/pkg/queue"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

// hikingNode is the node in the A* search algorithm.
type hikingNode struct {
	point geom.Point2D[int]
	dist  int
}

// Cost returns the cost of this node, f(n) = g(n) + h(n)
//
// The cost of this node is the distance travelled (g) plus the estimated
// cost to get to the target (h). Here, the manhattan distance to the target
// is being used as the estimated cost.
func (h *hikingNode) Cost(target geom.Point2D[int]) int {
	return h.dist + util.Abs(h.point.X-target.X) - util.Abs(h.point.Y-target.Y)
}

// heightMap represents the height map of the surrounding.
type heightMap struct {
	height *matrix.Dense[rune]
	// sources are the location of all the points with the lowest elevation ('a').
	sources []geom.Point2D[int]
	start   geom.Point2D[int]
	end     geom.Point2D[int]
}

// from returns all the points which can be reached from the given point p.
// The move can only be done one step in either of the four direction which
// are inside the map.
func (m *heightMap) from(p geom.Point2D[int]) []geom.Point2D[int] {
	var points []geom.Point2D[int]
	for _, pt := range util.CardinalDirection(p.Y, p.X, m.height.Rows, m.height.Cols) {
		// Filter out the points whose height is higher than the current
		// point by atleast 2. The lower elevation can be much higher.
		if m.height.At(pt[0], pt[1])-m.height.At(p.Y, p.X) <= 1 {
			points = append(points, geom.Point2D[int]{X: pt[1], Y: pt[0]})
		}
	}
	return points
}

// shortestHikingDistance returns the shortest distance from either the
// start point or from one of the sources to the end.
func (m *heightMap) shortestHikingDistance(fromLowestElevation bool) int {
	distance := make(map[geom.Point2D[int]]int)

	var sources []geom.Point2D[int]
	if fromLowestElevation {
		sources = m.sources
	} else {
		sources = append(sources, m.start)
	}

	pq := make(queue.PriorityQueue, 0, len(m.sources))
	for _, source := range sources {
		start := &hikingNode{point: source, dist: 0}
		distance[start.point] = 0
		pq = append(pq, &queue.Item{Value: start, Priority: 0})
	}

	for !pq.IsEmpty() {
		item := heap.Pop(&pq).(*queue.Item)
		node := item.Value.(*hikingNode)
		if node.point.Equal(m.end) {
			break
		}
		for _, to := range m.from(node.point) {
			dist := node.dist + 1
			if v, ok := distance[to]; !ok || dist < v {
				heap.Push(&pq, &queue.Item{
					Value:    &hikingNode{point: to, dist: dist},
					Priority: node.Cost(m.end),
				})
				distance[to] = dist
			}
		}
	}

	if dist, ok := distance[m.end]; ok {
		return dist
	}

	panic("no path found")
}

// shortestHikingDistance1 returns the shortest hiking distance from start
// to end using the A* search algorithm.
func (m *heightMap) shortestHikingDistance1() int {
	return m.shortestHikingDistance(false)
}

// shortestHikingDistance2 returns the shortest hiking distance from a
// starting point at the lowest elevation ('a') to the end.
func (m *heightMap) shortestHikingDistance2() int {
	return m.shortestHikingDistance(true)
}

// shortestHikingDistanceBFS returns the shortest hiking distance from
// start to end using Breadth-first search algorithm.
func (m *heightMap) shortestHikingDistanceBFS() int {
	q := queue.New(&hikingNode{point: m.start, dist: 0})
	visited := set.New(m.start)

	for !q.IsEmpty() {
		node, _ := q.Dequeue()
		if node.point.Equal(m.end) {
			return node.dist
		}
		for _, to := range m.from(node.point) {
			if visited.Contains(to) {
				continue
			}
			q.Enqueue(&hikingNode{point: to, dist: node.dist + 1})
			visited.Add(to)
		}
	}

	panic("no path found")
}

func parseHeightMap(lines []string) *heightMap {
	var start, end geom.Point2D[int]
	var sources []geom.Point2D[int]

	heights := make([]rune, 0, len(lines))
	for y, line := range lines {
		for x, char := range line {
			switch char {
			case 'S':
				start = geom.Point2D[int]{X: x, Y: y}
				char = 'a'
				fallthrough
			case 'a':
				sources = append(sources, geom.Point2D[int]{X: x, Y: y})
			case 'E':
				end = geom.Point2D[int]{X: x, Y: y}
				char = 'z'
			}
			heights = append(heights, char)
		}
	}

	return &heightMap{
		height:  matrix.NewDense(len(lines), len(lines[0]), heights),
		sources: sources,
		start:   start,
		end:     end,
	}
}

func Sol12(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	m := parseHeightMap(lines)

	return fmt.Sprintf(
		"12.1: %d\n12.2: %d\n", m.shortestHikingDistance1(), m.shortestHikingDistance2(),
	), nil
}
