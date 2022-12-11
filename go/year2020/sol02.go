package year2020

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

var passwordRegex = regexp.MustCompile(`(\d+)-(\d+) ([a-z]): ([a-z]+)`)

type password struct {
	// min and max are the lowest and highest number of times a given letter
	// must appear for the password to be valid.
	min int
	max int

	// pos1 and pos2 are the two positions in the password.
	pos1 int
	pos2 int

	// letter is the given letter which must appear in a password at least
	// min time and at most max times.
	letter string

	// value is the actual password string.
	value string
}

// isOldValid checks for the validity of the password according to the password
// policy rules from the first part of the puzzle.
func (p *password) isOldValid() bool {
	count := strings.Count(p.value, p.letter)
	return (p.min <= count) && (count <= p.max)
}

// isValid checks for the validity of the password according to the password
// policy rules from the second part of the puzzle.
func (p *password) isValid() bool {
	occurences := 0
	if string(p.value[p.pos1-1]) == p.letter {
		occurences++
	}
	if string(p.value[p.pos2-1]) == p.letter {
		occurences++
	}
	// Only one of the position must contain the letter.
	return occurences == 1
}

func parsePassword(lines []string) ([]*password, error) {
	passwords := make([]*password, len(lines))
	for i, line := range lines {
		matches := passwordRegex.FindStringSubmatch(line)
		// The line itself + 4 matched groups
		if len(matches) != 5 {
			return nil, fmt.Errorf("invalid line: %s", line)
		}
		num1, num2 := util.MustAtoi(matches[1]), util.MustAtoi(matches[2])
		passwords[i] = &password{
			min:    num1,
			max:    num2,
			pos1:   num1,
			pos2:   num2,
			letter: matches[3],
			value:  matches[4],
		}
	}
	return passwords, nil
}

func Sol02(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	passwords, err := parsePassword(lines)
	if err != nil {
		return "", err
	}

	var count1, count2 int
	for _, p := range passwords {
		if p.isOldValid() {
			count1++
		}
		if p.isValid() {
			count2++
		}
	}

	return fmt.Sprintf("2.1: %d\n2.2: %d\n", count1, count2), nil
}
