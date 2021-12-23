package year2021

import (
	"fmt"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/counter"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type memoKey struct {
	pair  string
	steps int
}

// polymer
type polymer struct {
	template string
	rules    map[string]string
	c        *counter.Counter
}

func newPolymer(template string, rules map[string]string) *polymer {
	c := counter.New()
	for _, letter := range template {
		c.Increment(string(letter))
	}
	return &polymer{
		template: template,
		rules:    rules,
		c:        c,
	}
}

func (p *polymer) process(steps int) {
	var recursiveProcess func(string, int) *counter.Counter
	memo := make(map[memoKey]*counter.Counter)

	recursiveProcess = func(pair string, steps int) *counter.Counter {
		// Base case: No steps remaining.
		if steps == 0 {
			return counter.New()
		}

		// Check if the function has been called with the exact arguments
		// and return the cache value if it exists.
		key := memoKey{pair, steps}
		if value, ok := memo[key]; ok {
			return value
		}

		element := p.rules[pair]

		// We only need to add the new element to the frequency map as we've
		// already added all the characters from the template when creating
		// the polymer.
		c := counter.New()
		c.Increment(element)

		// Recursion case:
		//
		// We take the element which needs to be inserted and make split the
		// original pair into two:
		//   1. first character from the pair + element
		//   2. element + second character from the pair
		//
		// Example: for pair "NN" and rule "NN -> C", the recursive pairs
		// will be "NC" and "CN".
		c.Update(recursiveProcess(string(pair[0])+element, steps-1))
		c.Update(recursiveProcess(element+string(pair[1]), steps-1))

		memo[key] = c
		return c
	}

	for i := 0; i < len(p.template)-1; i++ {
		p.c.Update(recursiveProcess(p.template[i:i+2], steps))
	}
}

func (p *polymer) reset() {
	p.c = counter.New()
	for _, letter := range p.template {
		p.c.Increment(string(letter))
	}
}

// diff returns the difference between quantity of the most common element and
// quantity of the least common element.
func (p *polymer) diff() int {
	return p.c.Get(p.c.MostCommon()) - p.c.Get(p.c.LeastCommon())
}

func parseInsertionRules(lines []string) map[string]string {
	rules := make(map[string]string)
	for _, line := range lines {
		s := strings.Split(line, " -> ")
		rules[s[0]] = s[1]
	}
	return rules
}

func Sol14(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	p := newPolymer(lines[0], parseInsertionRules(lines[2:]))
	p.process(10)
	fmt.Printf("14.1: %d\n", p.diff())

	p.reset()
	p.process(40)
	fmt.Printf("14.2: %d\n", p.diff())
	return nil
}
