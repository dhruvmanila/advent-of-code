package year2020

import (
	"fmt"

	"github.com/dhruvmanila/advent-of-code/go/pkg/iterator"
	"github.com/dhruvmanila/advent-of-code/go/pkg/queue"
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
	it := iterator.NewString(expr)
	for it.Next() {
		switch it.Value() {
		case " ":
			continue
		case "+":
			op = opAdd
		case "*":
			op = opMult
		case "(":
			subExpr := ""
			openParen := 0
		Loop:
			for it.Next() {
				switch it.Value() {
				case "(":
					openParen++
				case ")":
					if openParen == 0 {
						break Loop
					}
					openParen--
				}
				subExpr += it.Value()
			}
			result = evaluateOp(result, evaluate(subExpr), op)
		default: // number
			result = evaluateOp(result, util.MustAtoi(it.Value()), op)
		}
	}
	return result
}

func evaluateAdvance(expr []byte) int {
	// Shunting-yard algorithm
	output := queue.New()
	operator := stack.New()
	for _, c := range expr {
		switch c {
		case ' ':
			continue
		case '+', '*':
			for !operator.IsEmpty() {
				top := operator.Peek().(byte)
				if top == '(' || top < c {
					break
				}
				output.Enqueue(operator.Pop())
			}
			fallthrough
		case '(':
			operator.Push(c)
		case ')':
			for {
				if operator.IsEmpty() {
					panic("mismatched parenthesis")
				}
				top := operator.Pop().(byte)
				if top == '(' { // discard the left parenthesis
					break
				}
				output.Enqueue(top)
			}
		case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9':
			output.Enqueue(c)
		default:
			panic("invalid char: " + string(c))
		}
	}
	// Add rest of the operators onto the ouput stack.
	for !operator.IsEmpty() {
		top := operator.Pop().(byte)
		if top == '(' {
			panic("mismatched parenthesis")
		}
		output.Enqueue(top)
	}

	numbers := stack.New()
	for !output.IsEmpty() {
		switch c := output.Dequeue().(byte); c {
		case '+', '*':
			n1 := numbers.Pop().(int)
			n2 := numbers.Pop().(int)
			numbers.Push(evaluateOp(n1, n2, c))
		default:
			numbers.Push(int(c - '0'))
		}
	}
	return numbers.Pop().(int)
}

func Sol18(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	result1, result2 := 0, 0
	for _, line := range lines {
		result1 += evaluate(line)
		result2 += evaluateAdvance([]byte(line))
	}

	fmt.Printf("18.1: %d\n18.2: %d\n", result1, result2)
	return nil
}
