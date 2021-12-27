package year2021

import (
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"

	"github.com/dhruvmanila/advent-of-code/go/pkg/stack"
	"github.com/dhruvmanila/advent-of-code/go/util"
)

type alu struct {
	instructions []string
	vars         map[string]int
}

func newAlu(instructions []string) *alu {
	return &alu{
		instructions: instructions,
		vars: map[string]int{
			"w": 0,
			"x": 0,
			"y": 0,
			"z": 0,
		},
	}
}

func (a *alu) run(input int) {
	digits := util.Digits(input)
	for _, instruction := range a.instructions {
		fields := strings.Fields(instruction)
		opcode, result := fields[0], fields[1]

		var num int
		if len(fields) == 3 {
			i, err := strconv.Atoi(fields[2])
			if err == nil {
				num = i
			} else {
				num = a.vars[fields[2]]
			}
		}

		switch opcode {
		case "inp":
			a.vars[result] = <-digits
		case "add":
			a.vars[result] += num
		case "mul":
			a.vars[result] *= num
		case "div":
			a.vars[result] /= num
		case "mod":
			a.vars[result] %= num
		case "eql":
			if a.vars[result] == num {
				a.vars[result] = 1
			} else {
				a.vars[result] = 0
			}
		default:
			panic("invalid opcode: " + opcode)
		}
	}
}

// reset will reset all the variables back to 0.
func (a *alu) reset() {
	for v := range a.vars {
		a.vars[v] = 0
	}
}

// z is used as a stack storing a bunch of small numbers at once by treating
// it as a big base-26 number. So, (z * 26) means z.push() and (z % 26) means
// z.pop().
//
// Digit 1
//
//   w₁ <-
//   z = 26z + w₁ + 2
//
// Digit 2
//
//   w₂ <-
//   z = 26z + w₂ + 16
//
// Digit 3
//
//   w₃ <-
//   z = 26z + w₃ + 9
//
// Digit 4
//
//   w₄ <-
//   z = 26z + w₄
//
// Digit 5
//
//   w₅ <-
//   z = z/26 => (z % 26) - 8 == w₅
//            => w₄ - 8 == w₅
//
// Digit 6
//
//   w₆ <-
//   z = 26z + w₆ + 12
//
// Digit 7
//
//   w₇ <-
//   z = z/26 => (z % 26) - 16 == w₇
//            => w₆ + 12 - 16 == w₇
//            => w₆ - 4 == w₇
//
// Digit 8
//
//   w₈ <-
//   z = z/26 => (z % 26) - 4 == w₈
//            => w₃ + 9 - 4 == w₈
//            => w₃ + 5 == w₈
//
// Digit 9
//
//   w₉ <-
//   z = 26z + w₉ + 3
//
// Digit 10
//
//   w₁₀ <-
//   z = z/26 => (z % 26) - 3 == w₁₀
//            => w₉ + 3 - 3 == w₁₀
//            => w₉ == w₁₀
//
// Digit 11
//
//   w₁₁ <-
//   z = 26z + w₁₁ + 9
//
// Digit 12
//
//   w₁₂ <-
//   z = z/26 => (z % 26) - 7 == w₁₂
//            => w₁₁ + 9 - 7 == w₁₂
//            => w₁₁ + 2 == w₁₂
//
// Digit 13
//
//   w₁₃ <-
//   z = z/26 => (z % 26) - 15 == w₁₃
//            => w₂ + 16 - 15 == w₁₃
//            => w₂ + 1 == w₁₃
//
// Digit 14
//
//   w₁₄ <-
//   z = z/26 => (z % 26) - 7 == w₁₄
//            => w₁ + 2 - 7 == w₁₄
//            => w₁ - 5 == w₁₄
//
// Final conditions
//
//   w₄  - 8 == w₅
//   w₆  - 4 == w₇
//   w₃  + 5 == w₈
//   w₉      == w₁₀
//   w₁₁ + 2 == w₁₂
//   w₂  + 1 == w₁₃
//   w₁  - 5 == w₁₄
//
// The above logic is coded below:

// digitVar is a variable representing a specific digit in a 14-digit number
// along with its position from the right.
type digitVar struct {
	// pos is the position of the digit from right, starting with 0.
	pos int
	// value is the value of digit at pos.
	value int
}

func formNumber(ds []*digitVar) int {
	sort.Slice(ds, func(i, j int) bool {
		return ds[i].pos > ds[j].pos
	})
	var n int
	for _, d := range ds {
		n += int(math.Pow10(d.pos)) * d.value
	}
	return n
}

// equation contains information regarding a specific form of equation:
//   left + constant == right OR left == right - constant
//
// Example:
//   x + 4 == y OR x == y - 4
type equation struct {
	left     *digitVar
	constant int
	right    *digitVar
}

// maximizeSolve will solve the equation to maximize the solution and update
// the variable values.
func (eq *equation) maximizeSolve() {
	if eq.constant < 0 {
		eq.left.value = 9
		eq.right.value = 9 + eq.constant
	} else {
		eq.right.value = 9
		eq.left.value = 9 - eq.constant
	}
}

// minimizeSolve will solve the equation to minimize the solution and update
// the variable values.
func (eq *equation) minimizeSolve() {
	if eq.constant < 0 {
		eq.right.value = 1
		eq.left.value = eq.right.value - eq.constant
	} else {
		eq.left.value = 1
		eq.right.value = eq.left.value + eq.constant
	}
}

func formEquations(instructions []string) []*equation {
	s := stack.New()
	equations := make([]*equation, 0, 7)
	pos := 13

	for i := 0; i < len(instructions); i += 18 {
		group := instructions[i : i+18]
		switch group[4] {
		case "div z 1":
			s.Push(&equation{
				left:     &digitVar{pos: pos},
				constant: util.MustAtoi(group[15][6:]),
			})
		case "div z 26":
			item := s.Pop()
			if item == nil {
				panic("empty stack")
			}
			eq := item.(*equation)
			eq.constant += util.MustAtoi(group[5][6:])
			eq.right = &digitVar{pos: pos}
			equations = append(equations, eq)
		}
		pos--
	}

	return equations
}

func Sol24(input string) error {
	lines, err := util.ReadLines(input)
	if err != nil {
		return err
	}

	equations := formEquations(lines)
	maximizedDigits := make([]*digitVar, 0, 14)
	for _, eq := range equations {
		eq.maximizeSolve()
		maximizedDigits = append(maximizedDigits, eq.left, eq.right)
	}
	largestModelNum := formNumber(maximizedDigits)

	minimizedDigits := make([]*digitVar, 0, 14)
	for _, eq := range equations {
		eq.minimizeSolve()
		minimizedDigits = append(minimizedDigits, eq.left, eq.right)
	}
	smallestModelNum := formNumber(minimizedDigits)

	// Let's fire up the ALU to verify our solution.
	alu := newAlu(lines)
	alu.run(largestModelNum)
	if alu.vars["z"] != 0 {
		return fmt.Errorf("z is not 0 for largest model number: %d", largestModelNum)
	}

	alu.reset()
	alu.run(smallestModelNum)
	if alu.vars["z"] != 0 {
		return fmt.Errorf("z is not 0 for smallest model number: %d", smallestModelNum)
	}

	fmt.Printf("24.1: %d\n24.2: %d\n", largestModelNum, smallestModelNum)
	return nil
}
