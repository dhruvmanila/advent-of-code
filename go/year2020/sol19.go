package year2020

import (
	"fmt"
	"regexp"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/util"
)

func validMessages(rules map[string]string, messages []string) (count1, count2 int) {
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
		return fmt.Sprintf("(%s)", regex)
	}

	rule0Regex := regexp.MustCompile("^" + genRegex("0") + "$")
	rule42Regex := regexp.MustCompile("^" + genRegex("42"))
	rule31Regex := regexp.MustCompile("^" + genRegex("31"))

	for _, message := range messages {
		if rule0Regex.MatchString(message) {
			count1++
		}
		var pos, count42, count31 int
		for match := rule42Regex.FindStringIndex(message); match != nil; {
			count42++
			pos, match = pos+match[1], rule42Regex.FindStringIndex(message[pos+match[1]:])
		}
		for match := rule31Regex.FindStringIndex(message[pos:]); match != nil; {
			count31++
			pos, match = pos+match[1], rule31Regex.FindStringIndex(message[pos+match[1]:])
		}
		if pos == len(message) && 0 < count31 && count31 < count42 {
			count2++
		}
	}
	return count1, count2
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

	count1, count2 := validMessages(rules, messages)
	fmt.Printf("19.1: %d\n19.2: %d\n", count1, count2)
	return nil
}
