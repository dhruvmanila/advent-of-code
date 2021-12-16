package year2021

import (
	"container/heap"
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/queue"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var direction2 = [][]int{
	{0, -1},
	{1, 0},
	{0, 1},
	{-1, 0},
}

// graph represents a weighted undirected graph.
type graph struct {
	// nodes is a matrix representing the node position (row, col) and the
	// weight which is indexed at the position.
	nodes [][]int

	// rows and cols are the total number of rows and columns in the graph matrix.
	rows int
	cols int
}

func newGraph(nodes [][]int) *graph {
	return &graph{
		nodes: nodes,
		rows:  len(nodes),
		cols:  len(nodes[0]),
	}
}

// at is used to get the weight of the node at a given position p.
func (g *graph) at(p position) int {
	return g.nodes[p.row][p.col]
}

// from is used to get the position of all the nodes in g that can be reached
// directly from the node at a given position p.
func (g *graph) from(p position) []position {
	var edges []position
	for _, d := range direction2 {
		r, c := p.row+d[1], p.col+d[0]
		if r < 0 || c < 0 || r >= g.rows || c >= g.cols {
			continue
		}
		edges = append(edges, position{row: r, col: c})
	}
	return edges
}

// lowestTotalRisk is used to calculate the lowest total risk taken by a path
// from s to t node in a graph g.
//
// This is using the Dijkstra's algorithm for finding the shortest path.
func lowestTotalRisk(s, t position, g *graph) int {
	visited := make(map[position]bool)
	risk := map[position]int{s: 0}

	pq := queue.PriorityQueue{{Value: s, Priority: 0}}
	for !pq.IsEmpty() {
		item := heap.Pop(&pq).(*queue.Item)
		p := item.Value.(position)
		for _, to := range g.from(p) {
			if visited[to] {
				continue
			}
			joint := risk[p] + g.at(to)
			if v, ok := risk[to]; !ok || joint < v {
				heap.Push(&pq, &queue.Item{Value: to, Priority: joint})
				risk[to] = joint
			}
		}
		visited[p] = true
	}

	return risk[t]
}

func constructGraph(lines []string) *graph {
	nodes := make([][]int, len(lines))
	for y, line := range lines {
		row := make([]int, len(line))
		for x, d := range line {
			row[x] = int(d - '0')
		}
		nodes[y] = row
	}
	return newGraph(nodes)
}

func constructGraphV2(lines []string) *graph {
	nodes := make([][]int, len(lines)*5)
	for i := range nodes {
		nodes[i] = make([]int, len(lines[0])*5)
	}
	for y, line := range lines {
		for x, d := range line {
			n := int(d - '0')
			for dy := 0; dy < 5; dy++ {
				for dx := 0; dx < 5; dx++ {
					val := (n + dx + dy) % 9
					if val == 0 {
						val = 9
					}
					row, col := dy*len(lines)+y, dx*len(line)+x
					nodes[row][col] = val
				}
			}
		}
	}
	return newGraph(nodes)
}

func Sol15(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	from := position{0, 0}
	to := position{len(lines[0]) - 1, len(lines) - 1}
	to2 := position{len(lines[0])*5 - 1, len(lines)*5 - 1}

	fmt.Printf(
		"15.1: %d\n15.2: %d\n",
		lowestTotalRisk(from, to, constructGraph(lines)),
		lowestTotalRisk(from, to2, constructGraphV2(lines)),
	)
	return nil
}
