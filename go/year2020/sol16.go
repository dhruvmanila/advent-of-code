package year2020

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var ticketRuleRegex = regexp.MustCompile(
	`([^:]+):\s*(\d+)-(\d+)\s*or\s*(\d+)-(\d+)`,
)

type intRange [2]int

func (r intRange) contains(n int) bool {
	return r[0] <= n && n < r[1]
}

type ticketInfo struct {
	rules  map[string][2]intRange
	ticket []int
	nearby [][]int
}

func newTicketInfo(sections [][]string) *ticketInfo {
	info := new(ticketInfo)
	info.rules = make(map[string][2]intRange)

	for _, line := range sections[0] {
		matches := ticketRuleRegex.FindStringSubmatch(line)
		if len(matches) != 6 {
			panic("invalid rule: '" + line + "'")
		}
		info.rules[matches[1]] = [2]intRange{
			[2]int{util.MustAtoi(matches[2]), util.MustAtoi(matches[3]) + 1},
			[2]int{util.MustAtoi(matches[4]), util.MustAtoi(matches[5]) + 1},
		}
	}

	for _, s := range strings.Split(sections[1][1], ",") {
		info.ticket = append(info.ticket, util.MustAtoi(s))
	}

	for _, line := range sections[2][1:] {
		ticket := make([]int, strings.Count(line, ",")+1)
		for i, s := range strings.Split(line, ",") {
			ticket[i] = util.MustAtoi(s)
		}
		info.nearby = append(info.nearby, ticket)
	}

	return info
}

func (i *ticketInfo) errorRate() int {
	rate := 0
	for _, ticket := range i.nearby {
	Loop:
		for _, value := range ticket {
			for _, ranges := range i.rules {
				if ranges[0].contains(value) || ranges[1].contains(value) {
					continue Loop
				}
			}
			rate += value
		}
	}
	return rate
}

func (i *ticketInfo) validNearbyTickets() (valid [][]int) {
TicketLoop:
	for _, ticket := range i.nearby {
	ValueLoop:
		for _, value := range ticket {
			for _, ranges := range i.rules {
				if ranges[0].contains(value) || ranges[1].contains(value) {
					continue ValueLoop
				}
			}
			continue TicketLoop
		}
		valid = append(valid, ticket)
	}
	return valid
}

func (i *ticketInfo) ruleOrder() []string {
	validTickets := i.validNearbyTickets()

	possibleFields := make(map[int]set.Set[string], len(i.rules))
	for col := 0; col < len(i.rules); col++ {
		possibleFields[col] = set.New[string]()
	Rule:
		for field, ranges := range i.rules {
			for _, ticket := range validTickets {
				if !(ranges[0].contains(ticket[col]) || ranges[1].contains(ticket[col])) {
					continue Rule
				}
			}
			possibleFields[col].Add(field)
		}
	}

	order := make([]string, len(i.rules))
	for len(possibleFields) != 0 {
		for col, possible := range possibleFields {
			if possible.Len() == 1 {
				field := possible.Pop()
				order[col] = field
				delete(possibleFields, col)
				for _, otherPossible := range possibleFields {
					otherPossible.Remove(field)
				}
				break
			}
		}
	}

	return order
}

func Sol16(input string) error {
	sections, err := util.ReadSections(input)
	if err != nil {
		return err
	}

	info := newTicketInfo(sections)

	departure := 1
	order := info.ruleOrder()
	for col, field := range order {
		if strings.HasPrefix(field, "departure") {
			departure *= info.ticket[col]
		}
	}

	fmt.Printf("16.1: %d\n16.2: %d\n", info.errorRate(), departure)
	return nil
}
