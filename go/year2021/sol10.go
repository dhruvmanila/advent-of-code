package year2021

import (
	"fmt"
	"sort"

	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var legalPairs = map[byte]byte{
	'(': ')',
	'[': ']',
	'{': '}',
	'<': '>',
}

var illegalCharPoints = map[byte]int{
	')': 3,
	']': 57,
	'}': 1197,
	'>': 25137,
}

var completionCharPoints = map[byte]int{
	')': 1,
	']': 2,
	'}': 3,
	'>': 4,
}

func closeChunks(s *stack.Stack[byte]) []byte {
	closed := make([]byte, 0, s.Len())
	for {
		elem, ok := s.Pop()
		if !ok {
			break
		}
		closed = append(closed, legalPairs[elem])
	}
	return closed
}

func calculateCompletionScore(s []byte) int {
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
		s := stack.New[byte]()
		for _, char := range []byte(line) {
			switch char {
			case ')', ']', '}', '>':
				last, _ := s.Pop()
				if legalPairs[last] != char {
					syntaxErrorScore += illegalCharPoints[char]
					continue Line
				}
			default:
				s.Push(char)
			}
		}
		completionScores = append(
			completionScores,
			calculateCompletionScore(closeChunks(s)),
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
