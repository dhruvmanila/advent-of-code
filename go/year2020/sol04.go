package year2020

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/set"
)

// requiredFields is a list of all the required fields for a passport.
var requiredFields = []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

// validEyeColor is a set of valid eye colors for the "ecl" field in a passport.
var validEyeColor = set.New()

var (
	hexColorRegex = regexp.MustCompile(`^#[0-9a-f]{6}$`)
	heightRegex   = regexp.MustCompile(`(\d+)(cm|in)`)
)

// validRange is a map from the group name and the valid range for its value.
// The range is inclusive.
var validRange = map[string][2]int{
	"byr": {1920, 2002},
	"iyr": {2010, 2020},
	"eyr": {2020, 2030},
	"cm":  {150, 193},
	"in":  {59, 76},
}

func init() {
	validEyeColor.Add("amb", "blu", "brn", "gry", "grn", "hzl", "oth")
}

// passport is a map from the field name to its value.
type passport map[string]string

// newPassportFromString is used to contruct a passport object using the given lines.
func newPassportFromString(lines string) passport {
	passportInfo := make(passport)
	for _, line := range strings.Split(lines, "\n") {
		for _, pair := range strings.Fields(line) {
			info := strings.Split(pair, ":")
			passportInfo[info[0]] = info[1]
		}
	}
	return passportInfo
}

// containRequiredFields is used to check whether all the required fields are
// present in the passport or not.
func (p passport) containRequiredFields() bool {
	for _, field := range requiredFields {
		if _, exist := p[field]; !exist {
			return false
		}
	}
	return true
}

// validateFields is used to validate the information in each field for the
// passport.
func (p passport) validateFields() bool {
	for field, value := range p {
		switch field {
		case "byr", "iyr", "eyr":
			year, _ := strconv.Atoi(value)
			r := validRange[field]
			if year < r[0] || year > r[1] {
				return false
			}
		case "hgt":
			matches := heightRegex.FindStringSubmatch(value)
			if len(matches) != 3 {
				return false
			}
			height, _ := strconv.Atoi(matches[1])
			r := validRange[matches[2]]
			if height < r[0] || height > r[1] {
				return false
			}
		case "hcl":
			if !hexColorRegex.MatchString(value) {
				return false
			}
		case "ecl":
			if !validEyeColor.Contains(value) {
				return false
			}
		case "pid":
			if len(value) != 9 {
				return false
			}
		}
	}
	return true
}

func Sol4(input string) error {
	content, err := os.ReadFile(input)
	if err != nil {
		return err
	}

	var allFieldsPresent, validValues int
	// Every passport is separated by a blank line.
	for _, passportLines := range strings.Split(string(content), "\n\n") {
		p := newPassportFromString(passportLines)
		if !p.containRequiredFields() {
			continue
		}
		allFieldsPresent++
		if !p.validateFields() {
			continue
		}
		validValues++
	}

	fmt.Printf("4.1: %d\n4.2: %d\n", allFieldsPresent, validValues)
	return nil
}
