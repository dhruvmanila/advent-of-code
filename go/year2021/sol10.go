package year2021

import (
	"container/list"
	"errors"
	"fmt"
	"sort"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

var legalPairs = map[rune]rune{
	'(': ')',
	'[': ']',
	'{': '}',
	'<': '>',
}

var illegalCharPoints = map[rune]int{
	')': 3,
	']': 57,
	'}': 1197,
	'>': 25137,
}

var completionCharPoints = map[rune]int{
	')': 1,
	']': 2,
	'}': 3,
	'>': 4,
}

var errStackEmpty = errors.New("stack is empty")

type chunkStack struct {
	stack *list.List
}

func newChunkStack() *chunkStack {
	return &chunkStack{stack: list.New()}
}

func (c *chunkStack) push(chunk rune) {
	c.stack.PushFront(chunk)
}

func (c *chunkStack) pop() (rune, error) {
	if element := c.stack.Front(); element != nil {
		c.stack.Remove(element)
		return element.Value.(rune), nil
	}
	return '\x00', errStackEmpty
}

func (c *chunkStack) closeChunks() string {
	var closed string
	for {
		elem, err := c.pop()
		if errors.Is(err, errStackEmpty) {
			break
		}
		closed += string(legalPairs[elem])
	}
	return closed
}

func calculateCompletionScore(s string) int {
	score := 0
	for _, char := range s {
		score *= 5
		score += completionCharPoints[char]
	}
	return score
}

func Sol10(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	var syntaxErrorScore int
	var completionScores []int

Line:
	for _, line := range lines {
		stack := newChunkStack()
		for _, char := range line {
			switch char {
			case ')', ']', '}', '>':
				last, err := stack.pop()
				if err != nil {
					return err
				}
				if legalPairs[last] != char {
					syntaxErrorScore += illegalCharPoints[char]
					continue Line
				}
			default:
				stack.push(char)
			}
		}
		completionScores = append(
			completionScores,
			calculateCompletionScore(stack.closeChunks()),
		)
	}

	sort.Ints(completionScores)
	fmt.Printf(
		"10.1: %d\n10.2: %d\n",
		syntaxErrorScore,
		completionScores[len(completionScores)/2],
	)
	return nil
}
