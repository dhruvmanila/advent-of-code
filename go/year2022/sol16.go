package year2022

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

var valveRegex = regexp.MustCompile(
	`^Valve (?P<name>[A-Z]{2}) has flow rate=(?P<rate>\d+); ` +
		`tunnels? leads? to valves? (?P<valves>[A-Z, ]+)$`,
)

type valve struct {
	name   string
	rate   int
	closed bool
	valves []string
}

func (v *valve) String() string {
	return fmt.Sprintf("Valve{name:%s rate:%d closed:%v valves:%v}", v.name, v.rate, v.closed, v.valves)
}

func parseValves(lines []string) (map[string]*valve, error) {
	valves := make(map[string]*valve, len(lines))
	for idx, line := range lines {
		matches := valveRegex.FindStringSubmatch(line)
		if len(matches) != valveRegex.NumSubexp()+1 {
			return nil, fmt.Errorf("line %d: %q: invalid scan", idx, line)
		}

		var toValves []string
		for _, v := range strings.Split(matches[valveRegex.SubexpIndex("valves")], ", ") {
			toValves = append(toValves, v)
		}

		name := matches[valveRegex.SubexpIndex("name")]
		valves[name] = &valve{
			name:   name,
			rate:   util.MustAtoi(matches[valveRegex.SubexpIndex("rate")]),
			closed: true,
			valves: toValves,
		}
	}
	return valves, nil
}

func maxPressure(valves map[string]*valve) int {
	return 0
}

func Sol16(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	valves, err := parseValves(lines)
	if err != nil {
		return "", err
	}

	for _, from := range valves {
		for _, to := range from.valves {
			fmt.Printf("%s -> %s\n", from.name, to)
		}
	}

	return fmt.Sprintf("16.1: %d\n16.2: %d\n", maxPressure(valves), 0), nil
}
