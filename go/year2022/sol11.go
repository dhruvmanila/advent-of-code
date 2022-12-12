package year2022

import (
	"fmt"
	"os"
	"regexp"
	"sort"
	"strings"

	"github.com/MakeNowJust/heredoc"

	"github.com/dhruvmanila/advent-of-code/go/pkg/queue"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var notesRegex = regexp.MustCompile(
	heredoc.Doc(`
Monkey (?P<id>\d+):
  Starting items: (?P<items>[0-9, ]+)
  Operation: new = old (?P<operator>[*+]) (?P<operand>old|\d+)
  Test: divisible by (?P<mod>\d+)
    If true: throw to monkey (?P<targetTrue>\d+)
    If false: throw to monkey (?P<targetFalse>\d+)`,
	),
)

// operationFunc is the operation function for the monkey. It takes in the
// old worry level and return the new worry level after performing certain
// operation on it.
type operationFunc func(worry int) int

// opAdd returns the operation function which increases the worry level
// by adding the given operand.
func opAdd(operand int) operationFunc {
	return func(worry int) int {
		return worry + operand
	}
}

// opMul returns the operation function which increases the worry level
// by multiplying the given operand.
func opMul(operand int) operationFunc {
	return func(worry int) int {
		return worry * operand
	}
}

// opSquare returns the operation function which increases the worry level
// by multiplying it by itself thus squaring it.
func opSquare() operationFunc {
	return func(worry int) int {
		return worry * worry
	}
}

// throwItem represents an item a monkey throws to another monkey.
type throwItem struct {
	// monkeyId is the target monkey to throw this item to.
	monkeyId int
	// worryLevel is the worry level of the item being thrown.
	worryLevel int
}

// monkey represents the monkey in the throwing game.
type monkey struct {
	id          int
	items       *queue.Queue[int]
	operation   operationFunc
	mod         int
	targetTrue  int
	targetFalse int

	relief        int
	inspected     int
	originalItems []int
}

// SetNoRelief sets the relief to 1, thus removing the reduction of worry
// levels.
func (m *monkey) SetNoRelief() {
	m.relief = 1
}

// Turn does a turn for the monkey. It returns the items to be thrown to
// other monkeys.
func (m *monkey) Turn() []*throwItem {
	items := make([]*throwItem, 0, m.items.Len())

	for {
		worryLevel, ok := m.items.Dequeue()
		if !ok {
			break
		}
		worryLevel = m.operation(worryLevel) / m.relief

		var nextMonkey int
		if worryLevel%m.mod == 0 {
			nextMonkey = m.targetTrue
		} else {
			nextMonkey = m.targetFalse
		}

		items = append(items, &throwItem{
			monkeyId:   nextMonkey,
			worryLevel: worryLevel,
		})
		m.inspected++
	}

	return items
}

// Reset will reset the monkey to its original state.
func (m *monkey) Reset() {
	m.inspected = 0
	m.items = queue.New(m.originalItems...)
}

// parseNotes parses the notes creating all the monkeys.
// The returned error will only be if the parsing fails.
func parseNotes(notes []string) ([]*monkey, error) {
	monkeys := make([]*monkey, 0, len(notes))

	for idx, note := range notes {
		matches := notesRegex.FindStringSubmatch(note)
		if len(matches) != 8 {
			return nil, fmt.Errorf("line %d: %q: failed to match note", idx, note)
		}

		var items []int
		for _, s := range strings.Split(matches[notesRegex.SubexpIndex("items")], ", ") {
			items = append(items, util.MustAtoi(s))
		}

		var opfunc operationFunc
		switch operand := matches[notesRegex.SubexpIndex("operand")]; operand {
		case "old":
			// This assumes that when the operand is "old", the operator is
			// going to be "*", thus generating the square operation function.
			opfunc = opSquare()
		default:
			num := util.MustAtoi(operand)
			switch matches[notesRegex.SubexpIndex("operator")] {
			case "*":
				opfunc = opMul(num)
			case "+":
				opfunc = opAdd(num)
			}
		}

		monkeys = append(monkeys, &monkey{
			id:            util.MustAtoi(matches[notesRegex.SubexpIndex("id")]),
			items:         queue.New(items...),
			operation:     opfunc,
			mod:           util.MustAtoi(matches[notesRegex.SubexpIndex("mod")]),
			targetTrue:    util.MustAtoi(matches[notesRegex.SubexpIndex("targetTrue")]),
			targetFalse:   util.MustAtoi(matches[notesRegex.SubexpIndex("targetFalse")]),
			relief:        3,
			originalItems: items,
		})
	}

	return monkeys, nil
}

// watchStuffSlingingSimianShenanigans will simulate the monkeys throwing
// around items upto the given number of rounds and return the monkey
// business value.
func watchStuffSlingingSimianShenanigans(monkeys []*monkey, rounds int) int {
	lcm := 1
	for _, m := range monkeys {
		lcm *= m.mod
	}

	for i := 0; i < rounds; i++ {
		for _, m := range monkeys {
			for _, t := range m.Turn() {
				monkeys[t.monkeyId].items.Enqueue(t.worryLevel % lcm)
			}
		}
	}

	sort.Slice(monkeys, func(i, j int) bool {
		return monkeys[i].inspected > monkeys[j].inspected
	})

	return monkeys[0].inspected * monkeys[1].inspected
}

func Sol11(input string) (string, error) {
	content, err := os.ReadFile(input)
	if err != nil {
		return "", err
	}
	notes := strings.Split(string(content), "\n\n")

	monkeys, err := parseNotes(notes)
	if err != nil {
		return "", err
	}

	monkeyBusiness1 := watchStuffSlingingSimianShenanigans(monkeys, 20)

	// The sorting is done in-place, so now the monkeys are in decreasing
	// order of the number of items inspected. We need to sort it back to
	// the original order.
	sort.Slice(monkeys, func(i, j int) bool {
		return monkeys[i].id < monkeys[j].id
	})

	// Reset all the monkeys for another set of rounds. Also, now the worry
	// levels are not to be divided by the relief, so set it to 1.
	for _, m := range monkeys {
		m.Reset()
		m.SetNoRelief()
	}

	monkeyBusiness2 := watchStuffSlingingSimianShenanigans(monkeys, 10000)

	return fmt.Sprintf("11.1: %d\n11.2: %d\n", monkeyBusiness1, monkeyBusiness2), nil
}
