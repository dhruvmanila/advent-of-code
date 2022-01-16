package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/queue"
	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type player struct {
	id   uint8
	deck queue.Queue[int]
}

func newPlayer(playerId uint8, cards []int) *player {
	deck := queue.New[int]()
	for _, card := range cards {
		deck.Enqueue(card)
	}
	return &player{
		id:   playerId,
		deck: *deck,
	}
}

func (p *player) String() string {
	return fmt.Sprintf("&player{id:%v deck:%v}", p.id, p.deck)
}

func playCombat(p1, p2 *player) *player {
	for !(p1.deck.IsEmpty() || p2.deck.IsEmpty()) {
		c1, _ := p1.deck.Dequeue()
		c2, _ := p2.deck.Dequeue()
		if c1 > c2 {
			p1.deck.Enqueue(c1, c2)
		} else {
			p2.deck.Enqueue(c2, c1)
		}
	}
	if p1.deck.IsEmpty() {
		return p2
	} else {
		return p1
	}
}

func playRecursiveCombat(p1, p2 *player) *player {
	// play is the recursive function which will contain the logic of playing
	// the combat game recursively. This will avoid the infinite loop by keeping
	// a set of seen game state for every game.
	var play func(p1, p2 *player) *player
	play = func(p1, p2 *player) *player {
		seen := set.New[string]()
		for !(p1.deck.IsEmpty() || p2.deck.IsEmpty()) {
			// We will use the string representation of each player which
			// contains the id and card values in order as the game state.
			state := p1.String() + p2.String()
			if seen.Contains(state) {
				return p1
			}
			seen.Add(state)
			// Begin the round by drawing the top card from each player's deck.
			c1, _ := p1.deck.Dequeue()
			c2, _ := p2.deck.Dequeue()
			var p1wins bool
			if p1.deck.Len() < c1 || p2.deck.Len() < c2 {
				p1wins = c1 > c2
			} else {
				d1, d2 := make(queue.Queue[int], c1), make(queue.Queue[int], c2)
				copy(d1, p1.deck[:c1])
				copy(d2, p2.deck[:c2])
				// Recursive call for the sub-game
				p1wins = play(
					&player{id: p1.id, deck: d1},
					&player{id: p2.id, deck: d2},
				).id == p1.id
			}
			if p1wins {
				p1.deck.Enqueue(c1, c2)
			} else {
				p2.deck.Enqueue(c2, c1)
			}
		}
		if p1.deck.IsEmpty() {
			return p2
		} else {
			return p1
		}
	}
	return play(p1, p2)
}

func calculateScore(p *player) int {
	score := 0
	multiplier := p.deck.Len()
	for {
		if card, ok := p.deck.Dequeue(); ok {
			score += card * multiplier
		} else {
			break
		}
		multiplier--
	}
	return score
}

func parseCards(sections [][]string) ([]int, []int) {
	decks := make([][]int, 0, len(sections))
	for _, section := range sections {
		cards := make([]int, len(section)-1)
		for i, line := range section[1:] {
			cards[i] = util.MustAtoi(line)
		}
		decks = append(decks, cards)
	}
	return decks[0], decks[1]
}

func Sol22(input string) error {
	sections, err := util.ReadSections(input)
	if err != nil {
		return err
	}

	cards1, cards2 := parseCards(sections)
	score1 := calculateScore(
		playCombat(
			newPlayer(1, cards1),
			newPlayer(2, cards2),
		),
	)
	score2 := calculateScore(
		playRecursiveCombat(
			newPlayer(1, cards1),
			newPlayer(2, cards2),
		),
	)

	fmt.Printf("22.1: %d\n22.2: %d\n", score1, score2)
	return nil
}
