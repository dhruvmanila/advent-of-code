package year2021

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type deterministicDice struct {
	// next is the number on the next roll.
	next int
	// rolls is the number of rolls done by the dice.
	rolls int
}

func newDeterministicDice() *deterministicDice {
	return &deterministicDice{
		next:  0,
		rolls: 0,
	}
}

func (d *deterministicDice) roll() int {
	d.next = d.next%100 + 1
	d.rolls++
	return d.next
}

// playerId is an id assigned to all the new players starting from 0. This will
// be useful to distinguish between multiple players.
var playerId = -1

type player struct {
	// pos is the current position of the player on the board.
	pos int
	// score is the total score of the player.
	score int
	// start is the start position of the player.
	start int
	// id is a unique number assigned to each player.
	id int
}

func newPlayer(start int) *player {
	playerId++
	return &player{
		pos:   start,
		start: start,
		score: 0,
		id:    playerId,
	}
}

func (p *player) move(steps int) {
	p.pos = (p.pos+steps-1)%10 + 1
	p.score += p.pos
}

func practiceGame(p1, p2 player) int {
	var loser player
	dice := newDeterministicDice()

	for {
		p1.move(dice.roll() + dice.roll() + dice.roll())
		if p1.score >= 1000 {
			loser = p2
			break
		}

		p2.move(dice.roll() + dice.roll() + dice.roll())
		if p2.score >= 1000 {
			loser = p1
			break
		}
	}

	return loser.score * dice.rolls
}

// quantumRolls is a map from the sum of combination of three rolls possible
// for a quantum dice to their frequency.
var quantumRolls = map[int]int{
	3: 1, // {1, 1, 1}
	4: 3, // {1, 1, 2}
	5: 6, // {1, 2, 2}, {1, 1, 3}
	6: 7, // {1, 2, 3}, {2, 2, 2}
	7: 6, // {1, 3, 3}, {2, 2, 3}
	8: 3, // {2, 3, 3}
	9: 1, // {3, 3, 3}
}

func realGame(p1, p2 player) int {
	memo := make(map[[2]player]*counter.Counter)
	var loop func(p, other player) *counter.Counter

	// Here, p represents the currently playing player while other is waiting
	// for its turn.
	loop = func(p, other player) *counter.Counter {
		// Base case: One of the player have score equal to or greater than 21.
		switch {
		case p.score >= 21:
			return counter.NewWith(p.id)
		case other.score >= 21:
			return counter.NewWith(other.id)
		}

		key := [2]player{p, other}
		if value, ok := memo[key]; ok {
			return value
		}

		c := counter.New()
		for steps, freq := range quantumRolls {
			// We cannot update the original struct as we still have other
			// rolls to play. This is creating a copy of the struct and works
			// only because the struct contains only primitive values.
			np := p
			np.move(steps)

			// Now it's other's turn.
			rc := loop(other, np)
			c.IncrementBy(p.id, rc.Get(p.id)*freq)
			c.IncrementBy(other.id, rc.Get(other.id)*freq)
		}

		memo[key] = c
		return c
	}

	wins := loop(p1, p2)
	return util.IntMax(wins.Get(p1.id), wins.Get(p2.id))
}

func Sol21(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	p1 := newPlayer(util.Atoi(lines[0][28:]))
	p2 := newPlayer(util.Atoi(lines[1][28:]))

	// We don't want to mutate the player information.
	practiceGameOutput := practiceGame(*p1, *p2)
	winCount := realGame(*p1, *p2)

	fmt.Printf("21.1: %d\n21.2: %d\n", practiceGameOutput, winCount)
	return nil
}
