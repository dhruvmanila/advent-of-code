package year2021

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

// sentinel is used to represent an empty number node.
const sentinel = -1

type number struct {
	// value is an integer contained in this node. To represent an empty node,
	// use the constant `sentinel` for the value.
	value int

	// left and right are the left and right child of this node respectively.
	left  *number
	right *number
}

func newNumber(v int) *number {
	return &number{
		value: v,
		left:  nil,
		right: nil,
	}
}

// Add returns a new number resulting from adding n and other.
func (n *number) Add(other *number) *number {
	return &number{
		value: sentinel,
		left:  n,
		right: other,
	}
}

// Reduce returns the reduced snailfish number.
func (n *number) Reduce() *number {
	for {
		if n.explode() || n.split() {
			continue
		}
		break
	}
	return n
}

// explode is used to perform the explode operation on a snailfish number. It
// returns true if an exploding pair was found (performing the explode operation),
// false otherwise.
func (n *number) explode() bool {
	var visit func(*number, int) bool

	// prev points to the previous number node for the current node which is
	// basically the first number node to the left of the exploding pair.
	var prev *number

	// toAdd is a number which needs to be added to the next number node which
	// is to add it to the first number node to the right of the exploding pair.
	toAdd := sentinel

	visit = func(node *number, depth int) bool {
		if node.value != sentinel { // number node
			prev = node
			return false
		} else if depth+1 == 5 { // empty node and reached depth limit
			if prev != nil {
				prev.value += node.left.value
			}
			toAdd = node.right.value
			// Replace the exploding pair with the regular number 0.
			node.left = nil
			node.right = nil
			node.value = 0
			return true
		}

		// found indicates whether an exploding pair exists in the left subtree.
		// If it does and the pair's right value is yet to be added, then add
		// it to the left most node in the right subtree.
		found := visit(node.left, depth+1)
		if found {
			if toAdd != sentinel {
				node.right.addLeftmost(toAdd)
				toAdd = sentinel
			}
			return true
		} else { // search for exploding pair in the right subtree
			return visit(node.right, depth+1)
		}
	}

	return visit(n, 0)
}

// addLeftmost is used to add the given integer i to the left most node from
// node n. This is used internally for the explode step.
func (n *number) addLeftmost(i int) {
	if n.value != sentinel {
		n.value += i
		return
	}
	n.left.addLeftmost(i)
}

// split is used to perform the split operation on a snailfish number. It
// returns true if a split was performed, false otherwise.
func (n *number) split() bool {
	if n.value != sentinel {
		if n.value > 9 {
			n.left = newNumber(n.value / 2)
			n.right = newNumber((n.value + 1) / 2)
			n.value = sentinel
			return true
		}
		return false
	}
	return n.left.split() || n.right.split()
}

// Magnitude is used to calculate the Magnitude of a snailfish number.
func (n *number) Magnitude() int {
	if n.value != sentinel {
		return n.value
	}
	return 3*n.left.Magnitude() + 2*n.right.Magnitude()
}

// String returns a string representation of a snailfish number.
func (n *number) String() string {
	if n.value == sentinel {
		return fmt.Sprintf("[%s,%s]", n.left, n.right)
	}
	return fmt.Sprint(n.value)
}

// dump is used to get a formatted dump of the tree in node. This is mainly
// used for debugging purposes. Each token is separated by a newline without
// any commas. A dot (.) is used to represent the depth of the current pair.
func dump(node *number) string {
	var format func(*number, int) string

	format = func(node *number, depth int) string {
		if node == nil {
			return ""
		}
		prefix := strings.Repeat(". ", depth)
		if node.value == sentinel {
			children := format(node.left, depth+1) + format(node.right, depth+1)
			return prefix + "[\n" + children + prefix + "]\n"
		} else {
			return prefix + fmt.Sprint(node.value) + "\n"
		}
	}

	return format(node, 0)
}

func parseNumber(line string) *number {
	nodes := stack.New[*number]()
	for _, char := range line {
		switch char {
		case '[', ',':
			continue
		case ']':
			// Order matters here: first pop should be the right node and the
			// second should be the left node.
			right, _ := nodes.Pop()
			left, _ := nodes.Pop()
			nodes.Push(&number{value: sentinel, right: right, left: left})
		default:
			nodes.Push(newNumber(int(char - '0')))
		}
	}
	result, ok := nodes.Pop()
	if !ok {
		panic("parse error")
	}
	return result
}

func Sol18(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	// As we're using pointers and also mutating the values in place, we need
	// to parse the input separately for part one and two. Maybe find a way
	// so that this can be avoided?

	total := parseNumber(lines[0])
	for _, line := range lines[1:] {
		total = total.Add(parseNumber(line)).Reduce()
	}

	maxMagnitude := 0
	for i := 0; i < len(lines); i++ {
		for j := 0; j < len(lines); j++ {
			if i == j {
				continue
			}
			maxMagnitude = util.Max(
				maxMagnitude,
				parseNumber(lines[i]).
					Add(parseNumber(lines[j])).
					Reduce().
					Magnitude(),
			)
		}
	}

	return fmt.Sprintf("18.1: %d\n18.2: %d\n", total.Magnitude(), maxMagnitude), nil
}
