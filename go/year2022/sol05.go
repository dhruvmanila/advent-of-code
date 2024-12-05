package year2022

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

func createStacks(data []string, stacklen int) []*stack.Stack[byte] {
	stacks := make([]*stack.Stack[byte], 0, stacklen)

	for sid := 0; sid < stacklen; sid++ {
		s := stack.New[byte]()
		// Index where the crates are for this specific stack (1, 5, 9, ...)
		// This assumes that the trailing whitespaces are not removed from
		// the input file.
		crateIdx := (sid * 4) + 1
		// Loop over the stack data in reverse order skipping the line where
		// the stack ids are mentioned.
		for i := len(data) - 2; i >= 0; i-- {
			crate := data[i][crateIdx]
			if crate == ' ' {
				break // there are no more crates
			}
			s.Push(data[i][crateIdx])
		}
		stacks = append(stacks, s)
	}

	return stacks
}

func topCrates(stacks []*stack.Stack[byte]) string {
	var crates string
	for _, s := range stacks {
		crate, ok := s.Peek()
		if !ok {
			panic("empty stack")
		}
		crates += string(crate)
	}
	return crates
}

func Sol05(input string) (string, error) {
	sections := util.ReadSections(input)

	// There are two sections separated by a blank line where the first
	// section contains the stack data and the second one contains the
	// rearrangement procedure.
	stackdata, instructions := sections[0], sections[1]

	// The last line contains the stack IDs which will provide us with
	// the number of stacks present in the data.
	stacklen := len(strings.Fields(stackdata[len(stackdata)-1]))

	// Create two stacks for the first and second part of the puzzle. This
	// is because the stacks are going to be updated in place.
	stacks1, stacks2 := createStacks(stackdata, stacklen), createStacks(stackdata, stacklen)

	var quantity, from, to int
	for idx, instruction := range instructions {
		_, err := fmt.Sscanf(instruction, "move %d from %d to %d", &quantity, &from, &to)
		if err != nil {
			return "", fmt.Errorf("line %d: %q: %w", idx, instruction, err)
		}

		// Part 1: This is just a simple pop "from" crate and push "to" crate.
		for n := 0; n < quantity; n++ {
			crate, ok := stacks1[from-1].Pop()
			if !ok {
				continue // empty stack
			}
			stacks1[to-1].Push(crate)
		}

		// Part 2: First, collect all the crates "from" stack and push "to"
		// crate in reverse order so as to maintain the "from" stack order.
		crates := make([]byte, 0, quantity)
		for n := 0; n < quantity; n++ {
			crate, ok := stacks2[from-1].Pop()
			if !ok {
				continue // empty stack
			}
			crates = append(crates, crate)
		}
		for n := quantity - 1; n >= 0; n-- {
			stacks2[to-1].Push(crates[n])
		}
	}

	return fmt.Sprintf("5.1: %s\n5.2: %s\n", topCrates(stacks1), topCrates(stacks2)), nil
}
