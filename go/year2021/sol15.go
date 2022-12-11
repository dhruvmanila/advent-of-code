package year2021

import (
	"container/heap"
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/queue"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

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
	dirs := util.CardinalDirection(p.row, p.col, g.rows, g.cols)
	edges := make([]position, len(dirs))
	for _, d := range dirs {
		edges = append(edges, position{d[0], d[1]})
	}
	return edges
}

// renderPath renders the given path on the grid by highlighting that position
// using ANSII escape sequence.
func (g *graph) renderPath(path []position) {
	grid := make([][]string, len(g.nodes))
	for y, row := range g.nodes {
		grid[y] = make([]string, len(row))
		for x, n := range row {
			grid[y][x] = fmt.Sprint(n)
		}
	}
	for _, p := range path {
		grid[p.row][p.col] = fmt.Sprintf("\033[7m%s\033[0m", grid[p.row][p.col])
	}
	for _, row := range grid {
		for _, c := range row {
			fmt.Print(c)
		}
		fmt.Println()
	}
	fmt.Println()
}

// lowestTotalRiskPath is used to calculate the lowest total risk taken by a path
// from s to t node in a graph g.
//
// This is using the Dijkstra's algorithm for finding the shortest path.
func lowestTotalRiskPath(s, t position, g *graph) int {
	visited := make(map[position]bool)
	risk := map[position]int{s: 0}
	prev := make(map[position]position)

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
				prev[to] = p
			}
		}
		visited[p] = true
	}

	path := []position{t}
	for p := prev[t]; p != s; p = prev[p] {
		path = append([]position{p}, path...)
	}
	path = append([]position{s}, path...)
	// g.renderPath(path)

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
					row, col := dy*len(lines)+y, dx*len(line)+x
					nodes[row][col] = (n+dx+dy-1)%9 + 1
				}
			}
		}
	}
	return newGraph(nodes)
}

func Sol15(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	from := position{0, 0}
	to1 := position{len(lines[0]) - 1, len(lines) - 1}
	to2 := position{len(lines[0])*5 - 1, len(lines)*5 - 1}

	return fmt.Sprintf(
		"15.1: %d\n15.2: %d\n",
		lowestTotalRiskPath(from, to1, constructGraph(lines)),
		lowestTotalRiskPath(from, to2, constructGraphV2(lines)),
	), nil
}
