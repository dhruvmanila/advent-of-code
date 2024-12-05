package year2022

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

const (
	unknown = iota
	rock
	paper
	scissors
)

const (
	lose = iota * 3
	draw
	win
)

func getShape(letter string) int {
	switch letter {
	case "A", "X":
		return rock
	case "B", "Y":
		return paper
	case "C", "Z":
		return scissors
	default:
		panic("invalid shape letter: " + letter)
	}
}

func getOutcome(letter string) int {
	switch letter {
	case "X":
		return lose
	case "Y":
		return draw
	case "Z":
		return win
	default:
		panic("invalid outcome letter: " + letter)
	}
}

// getScore1 returns the score as per the rules of the first part of the puzzle.
func getScore1(player, opponent int) int {
	switch player - opponent {
	case 0:
		return draw + player
	case paper - rock, rock - scissors:
		return win + player
	default:
		return lose + player
	}
}

// getScore2 returns the score as per the rules of the second part of the puzzle.
func getScore2(opponent int, outcome int) int {
	var choice int
	// The order of rock, paper and scissors is in a way that the next
	// shape is the one that beats the current shape. We can use this
	// to our advantage to choose the shape in order to get the desired
	// outcome.
	switch outcome {
	case lose:
		// To lose, we need to choose the previous shape circuling back
		// to the last shape (scissors) if the opponent's shape is the
		// first shape (rock).
		choice = opponent - 1
		if choice == unknown {
			choice = scissors
		}
	case draw:
		choice = opponent
	case win:
		// To win, we need to choose the next shape circuling back to
		// the first shape (rock) if the opponent's shape is the last
		// shape (scissors).
		if opponent == scissors {
			choice = rock
		} else {
			choice = opponent + 1
		}
	}
	return outcome + choice
}

func Sol02(input string) (string, error) {
	lines := util.ReadLines(input)

	score1, score2 := 0, 0
	for idx, line := range lines {
		first, second, found := strings.Cut(line, " ")
		if !found {
			return "", fmt.Errorf("line %d: invalid input: %q", idx, line)
		}
		player, opponent := getShape(second), getShape(first)
		score1 += getScore1(player, opponent)
		score2 += getScore2(opponent, getOutcome(second))
	}

	return fmt.Sprintf("2.1: %d\n2.2: %d\n", score1, score2), nil
}
