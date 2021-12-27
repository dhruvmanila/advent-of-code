package year2020

import (
	"errors"
	"fmt"
	"regexp"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

// bagMap is a mapping from bag color to another map which contains information
// regarding which bag can be contained in the outer bag along with the quantity.
type bagMap map[string]map[string]int

var (
	ruleRegex = regexp.MustCompile(`(^[a-z ]+) bags contain (.*)\.$`)
	bagRegex  = regexp.MustCompile(`^(\d+) ([a-z ]+) bags?$`)
)

// newBagMapFromRules is used to construct a new bagMap from a set of lines
// which contains all the rules.
func newBagMapFromRules(lines []string) (bagMap, error) {
	m := make(bagMap)
	for _, line := range lines {
		content := make(map[string]int)
		ruleMatches := ruleRegex.FindStringSubmatch(line)
		if len(ruleMatches) != 3 {
			return nil, errors.New("newBagMapFromRules: invalid rule")
		}
		if ruleMatches[2] != "no other bags" {
			for _, s := range strings.Split(ruleMatches[2], ", ") {
				bagMatches := bagRegex.FindStringSubmatch(s)
				if len(bagMatches) != 3 {
					return nil, errors.New("newBagMapFromRules: invalid bag content")
				}
				content[bagMatches[2]] = util.MustAtoi(bagMatches[1])
			}
		}
		m[ruleMatches[1]] = content
	}
	return m, nil
}

// canContain is used to check if the origin bag can contain the target bag
// as per the parsed rules.
func (m bagMap) canContain(origin, target string) bool {
	for child := range m[origin] {
		if child == target || m.canContain(child, target) {
			return true
		}
	}
	return false
}

// childCount is used to find out all the containing bags within the origin bag.
func (m bagMap) childCount(origin string) int {
	count := 0
	for child, quantity := range m[origin] {
		count += quantity
		count += m.childCount(child) * quantity
	}
	return count
}

func Sol07(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	bm, err := newBagMapFromRules(lines)
	if err != nil {
		return err
	}

	count := 0
	for origin := range bm {
		if bm.canContain(origin, "shiny gold") {
			count++
		}
	}

	fmt.Printf("7.1: %d\n7.2: %d\n", count, bm.childCount("shiny gold"))
	return nil
}
