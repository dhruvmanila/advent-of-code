package year2020

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func validMessages(rules map[string]string, messages []string) int {
	var genRegex func(string) string
	genRegex = func(k string) (regex string) {
		rule := rules[k]
		if strings.HasPrefix(rule, "\"") {
			regex = strings.Trim(rule, "\"")
		} else {
			for _, token := range strings.Fields(rule) {
				switch token {
				case "|":
					regex += "|"
				default:
					regex += genRegex(token)
				}
			}
		}
		return fmt.Sprintf("(?:%s)", regex)
	}

	rule0Regex := regexp.MustCompile("^" + genRegex("0") + "$")
	count := 0
	for _, message := range messages {
		if rule0Regex.MatchString(message) {
			count++
		}
	}
	return count
}

func Sol19(input string) error {
	sections, err := util.ReadSections(input)
	if err != nil {
		return err
	}

	rules := make(map[string]string, len(sections[0]))
	for _, line := range sections[0] {
		data := strings.Split(line, ": ")
		if len(data) != 2 {
			panic("invalid rule: " + line)
		}
		rules[data[0]] = data[1]
	}
	messages := sections[1]

	fmt.Printf("19.1: %d\n19.2: %d\n", validMessages(rules, messages), 0)
	return nil
}
