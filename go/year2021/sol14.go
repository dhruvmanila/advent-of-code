package year2021

import (
	"fmt"
	"math"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

// letterFrequency is a map representing the frequency of letters.
type letterFrequency map[string]int

// add is used to add a letter to the map. If the letter exists, increment
// the count, otherwise add it with count 1.
func (lf letterFrequency) add(letter string) {
	if _, ok := lf[letter]; ok {
		lf[letter]++
	} else {
		lf[letter] = 1
	}
}

// update is used to update lf with other. For existing letters, the count
// is added to lf entry.
func (lf letterFrequency) update(other letterFrequency) {
	for letter, quant := range other {
		if _, ok := lf[letter]; ok {
			lf[letter] += quant
		} else {
			lf[letter] = quant
		}
	}
}

// mostCommon is used to get the count of the most common letter.
func (lf letterFrequency) mostCommon() int {
	max := math.MinInt
	for _, quant := range lf {
		max = util.IntMax(max, quant)
	}
	return max
}

// leastCommon is used to get the count of the least common letter.
func (lf letterFrequency) leastCommon() int {
	min := math.MaxInt
	for _, quant := range lf {
		min = util.IntMin(min, quant)
	}
	return min
}

type memoKey struct {
	pair  string
	steps int
}

// polymer
type polymer struct {
	template string
	rules    map[string]string
	freq     letterFrequency
}

func newPolymer(template string, rules map[string]string) *polymer {
	freq := make(letterFrequency)
	for _, letter := range template {
		freq.add(string(letter))
	}
	return &polymer{
		template: template,
		rules:    rules,
		freq:     freq,
	}
}

func (p *polymer) process(steps int) {
	var recursiveProcess func(string, int) letterFrequency
	memo := make(map[memoKey]letterFrequency)

	recursiveProcess = func(pair string, steps int) letterFrequency {
		// Base case: No steps remaining.
		if steps == 0 {
			return letterFrequency{}
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
		freq := make(letterFrequency)
		freq.add(element)

		// Recursion case:
		//
		// We take the element which needs to be inserted and make split the
		// original pair into two:
		//   1. first character from the pair + element
		//   2. element + second character from the pair
		//
		// Example: for pair "NN" and rule "NN -> C", the recursive pairs
		// will be "NC" and "CN".
		freq.update(recursiveProcess(string(pair[0])+element, steps-1))
		freq.update(recursiveProcess(element+string(pair[1]), steps-1))

		memo[key] = freq
		return freq
	}

	for i := 0; i < len(p.template)-1; i++ {
		p.freq.update(recursiveProcess(p.template[i:i+2], steps))
	}
}

func (p *polymer) reset() {
	p.freq = make(letterFrequency)
	for _, letter := range p.template {
		p.freq.add(string(letter))
	}
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
	fmt.Printf("14.1: %d\n", p.freq.mostCommon()-p.freq.leastCommon())

	p.reset()
	p.process(40)
	fmt.Printf("14.2: %d\n", p.freq.mostCommon()-p.freq.leastCommon())
	return nil
}
