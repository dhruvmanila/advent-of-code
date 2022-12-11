package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/iterator"
	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

const (
	opAdd  byte = '+' // Addition (43) has higher precedence than multiplication (42).
	opMult byte = '*'
)

func evaluateOp(a, b int, op byte) int {
	switch op {
	case opAdd:
		return a + b
	case opMult:
		return a * b
	default:
		panic("invalid operator: " + string(op))
	}
}

func evaluate(expr string) int {
	result := 0
	op := opAdd
	it := iterator.New([]byte(expr))
	for it.Next() {
		switch it.Value() {
		case ' ':
			continue
		case '+':
			op = opAdd
		case '*':
			op = opMult
		case '(':
			subExpr := ""
			openParen := 0
		Loop:
			for it.Next() {
				switch it.Value() {
				case '(':
					openParen++
				case ')':
					if openParen == 0 {
						break Loop
					}
					openParen--
				}
				subExpr += string(it.Value())
			}
			result = evaluateOp(result, evaluate(subExpr), op)
		default: // digit
			result = evaluateOp(result, int(it.Value()-'0'), op)
		}
	}
	return result
}

func evaluateAdvance(expr []byte) int {
	// Shunting-yard algorithm
	// https://en.wikipedia.org/wiki/Shunting-yard_algorithm
	output := stack.New[int]()
	operator := stack.New[byte]()

	// evalOutput is helper function to evaluate the two numbers on top of the
	// output stack as per the given op.
	evalOutput := func(op byte) {
		n1, _ := output.Pop()
		n2, _ := output.Pop()
		output.Push(evaluateOp(n1, n2, op))
	}

	for _, c := range expr {
		switch c {
		case ' ':
			continue
		case '+', '*':
			for {
				if top, ok := operator.Peek(); !ok || top == '(' || top < c {
					break
				}
				top, _ := operator.Pop()
				evalOutput(top)
			}
			fallthrough
		case '(':
			operator.Push(c)
		case ')':
			for {
				top, ok := operator.Pop()
				if !ok {
					panic("mismatched parenthesis")
				}
				if top == '(' { // discard the left parenthesis
					break
				}
				evalOutput(top)
			}
		case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9':
			output.Push(int(c - '0'))
		default:
			panic("invalid char: " + string(c))
		}
	}
	// Add rest of the operators onto the ouput stack.
	for {
		top, ok := operator.Pop()
		if !ok {
			break
		}
		if top == '(' {
			panic("mismatched parenthesis")
		}
		evalOutput(top)
	}
	result, _ := output.Pop()
	return result
}

func Sol18(input string) (string, error) {
	lines, err := util.ReadLines(input)
	if err != nil {
		return "", err
	}

	result1, result2 := 0, 0
	for _, line := range lines {
		result1 += evaluate(line)
		result2 += evaluateAdvance([]byte(line))
	}

	return fmt.Sprintf("18.1: %d\n18.2: %d\n", result1, result2), nil
}
