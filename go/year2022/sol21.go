package year2022

import (
	"fmt"
	"regexp"

	"github.com/dhruvmanila/advent-of-code/go/pkg/operator"
	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

var exprRegex = regexp.MustCompile(
	`(?P<name>\w{4}): (?:(?P<number>\d+)|(?P<left>\w{4}) (?P<op>[+\-*/]) (?P<right>\w{4}))`,
)

var operators = map[string]func(left, right int) int{
	"+": operator.Add[int],
	"-": operator.Sub[int],
	"*": operator.Mul[int],
	"/": operator.Div[int],
}

type monkeyExpression struct {
	name string

	// left and right are the monkey names whose values are used to complete
	// the current monkey job.
	left  string
	right string

	// parent is the name of the monkey which will use this monkey's value.
	parent string

	// value held by the monkey. Use the Value method to get the monkey value.
	value  int
	solved bool

	// job is the arithmetic job the monkey needs to complete to get
	// the value.
	job func(left, right int) int
	op  string

	out chan int
}

// Value returns the number value held by this monkey. It will complete the
// job is needed.
func (m *monkeyExpression) Value(monkeys map[string]*monkeyExpression) int {
	if !m.solved {
		m.value = m.job(monkeys[m.left].Value(monkeys), monkeys[m.right].Value(monkeys))
		m.solved = true
	}
	return m.value
}

func (m *monkeyExpression) Start(monkeys map[string]*monkeyExpression) {
	go func() {
		if m.job != nil {
			m.value = m.job(<-monkeys[m.left].out, <-monkeys[m.right].out)
		}
		m.out <- m.value
	}()
}

func humnNum(monkeys map[string]*monkeyExpression) int {
	// path is the path from humn to root (excluding).
	path := stack.New[string]()
	current := "humn"
	for current != "root" {
		path.Push(current)
		current = monkeys[current].parent
	}

	var final int
	last, _ := path.Peek()
	if m := monkeys["root"]; m.left != last {
		final = monkeys[m.left].Value(monkeys)
	} else {
		final = monkeys[m.right].Value(monkeys)
	}

	for !path.IsEmpty() {
		current, _ = path.Pop()
		last, ok := path.Peek()
		if !ok {
			// Here, the current monkey is "humn" and so there's no other
			// monkeys remaining in the stack. This means the final value
			// is the one yelled by "humn".
			break
		}
		if m := monkeys[current]; m.left != last {
			final = inverseEval(monkeys[m.left].Value(monkeys), final, m.op, true)
		} else {
			final = inverseEval(monkeys[m.right].Value(monkeys), final, m.op, false)
		}
	}

	return final
}

// inverseEval evaluates the given equation in an inverse manner such that
// x op a = b is evaluated as x = b rop a, returning the value of x.
// isLeft indicates whether a is on the left of op or not.
func inverseEval(a, b int, op string, isLeft bool) int {
	var result int
	switch op {
	case "+": // x + a = b
		result = b - a
	case "-":
		if isLeft { // a - x = b
			result = a - b
		} else { // x - a = b
			result = b + a
		}
	case "*": // x * a = b
		result = b / a
	case "/":
		if isLeft { // a / x = b
			result = a / b
		} else { // x / a = b
			result = b * a
		}
	}
	return result
}

func parseExpressions(lines []string) (map[string]*monkeyExpression, error) {
	monkeys := make(map[string]*monkeyExpression, len(lines))

	for idx, line := range lines {
		matches := exprRegex.FindStringSubmatch(line)
		if len(matches) != exprRegex.NumSubexp()+1 {
			return nil, fmt.Errorf("line %d: %q: invalid monkey expression", idx, line)
		}
		var m *monkeyExpression
		switch n := matches[exprRegex.SubexpIndex("number")]; n {
		case "":
			op := matches[exprRegex.SubexpIndex("op")]
			opfunc, ok := operators[op]
			if !ok {
				return nil, fmt.Errorf("line %d: %q: invalid operator", idx, line)
			}
			m = &monkeyExpression{
				name:  matches[exprRegex.SubexpIndex("name")],
				out:   make(chan int, 1),
				left:  matches[exprRegex.SubexpIndex("left")],
				right: matches[exprRegex.SubexpIndex("right")],
				job:   opfunc,
				op:    op,
			}
		default:
			number := util.MustAtoi(n)
			m = &monkeyExpression{
				name:   matches[exprRegex.SubexpIndex("name")],
				value:  number,
				solved: true,
				out:    make(chan int, 1),
			}
		}
		monkeys[m.name] = m
	}

	// Link the monkeys
	for _, m := range monkeys {
		if !m.solved {
			monkeys[m.left].parent = m.name
			monkeys[m.right].parent = m.name
		}
	}

	return monkeys, nil
}

func Sol21(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	monkeys, err := parseExpressions(lines)
	if err != nil {
		return "", err
	}

	// Finding out the root number using goroutines.
	// for _, m := range monkeys {
	// 	m.Start(monkeys)
	// }
	// rootNum := <-monkeys["root"].out

	// Finding out the root number using recursion.
	rootNum := monkeys["root"].Value(monkeys)

	return fmt.Sprintf("21.1: %d\n21.2: %d\n", rootNum, humnNum(monkeys)), nil
}
