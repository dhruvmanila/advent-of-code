package year2021

import (
	"fmt"
	"sort"

	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
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

func closeChunks(s *stack.Stack) string {
	var closed string
	for {
		elem, ok := s.Pop().(rune)
		if !ok {
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
		s := stack.New()
		for _, char := range line {
			switch char {
			case ')', ']', '}', '>':
				last := s.Pop().(rune)
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
