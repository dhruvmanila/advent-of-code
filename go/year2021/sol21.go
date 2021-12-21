package year2021

import (
	"fmt"

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

func (p *player) reset() {
	p.pos = p.start
	p.score = 0
}

func practiceGame(p1, p2 *player) int {
	var loser *player
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

// quantumRolls contains the sum of all the possible rolls for a quantum dice
// and their frequency.
var quantumRolls = [][]int{
	{3, 1},
	{4, 3},
	{5, 6},
	{6, 7},
	{7, 6},
	{8, 3},
	{9, 1},
}

func realGame(p1, p2 player) int {
	memo := make(map[[2]player]map[int]int)

	var loop func(p, other player) map[int]int
	loop = func(p, other player) map[int]int {
		if p.score >= 21 {
			return map[int]int{p.id: 1, other.id: 0}
		} else if other.score >= 21 {
			return map[int]int{p.id: 0, other.id: 1}
		} else if value, ok := memo[[2]player{p, other}]; ok {
			return value
		}

		count := map[int]int{p.id: 0, other.id: 0}
		for _, roll := range quantumRolls {
			steps, freq := roll[0], roll[1]

			// We cannot update the original struct as we still have other
			// rolls to play. This is creating a copy of the struct and works
			// because the struct only contains primitive values.
			np := p
			np.move(steps)

			rc := loop(other, np)
			count[p.id] += rc[p.id] * freq
			count[other.id] += rc[other.id] * freq
		}

		memo[[2]player{p, other}] = count
		return count
	}

	wins := loop(p1, p2)
	return util.IntMax(wins[p1.id], wins[p2.id])
}

func Sol21(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	p1 := newPlayer(util.Atoi(lines[0][28:]))
	p2 := newPlayer(util.Atoi(lines[1][28:]))

	practiceGameOutput := practiceGame(p1, p2)
	p1.reset()
	p2.reset()
	winCount := realGame(*p1, *p2)

	fmt.Printf("21.1: %d\n21.2: %d\n", practiceGameOutput, winCount)
	return nil
}
